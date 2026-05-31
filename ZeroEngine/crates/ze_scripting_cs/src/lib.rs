use std::{
	any::Any,
	cell::RefCell,
	collections::HashMap,
	env, fs,
	path::{Path, PathBuf},
	rc::Rc,
};

use fn_ptr::{WithAbi, abi::System as AbiSystem};
use netcorehost::{
	hostfxr::{
		AssemblyDelegateLoader, FnPtr, GetManagedFunctionError, Hostfxr, HostfxrContext, InitializedForRuntimeConfig,
	},
	pdcstr,
	pdcstring::{PdCStr, PdCString},
};
use ze_core::{Context, Result, anyhow};
use ze_ecs::{
	Component, Deserialize, EntitiesView, EntityId, JsonSchema, Scene, Serialize, System, registry::ComponentRegistry,
};

const ASSEMBLY_PATH: &str = "assets/scripts/bin/Scripts.dll";
const RUNTIME_CONFIG_PATH: &str = "assets/scripts/bin/Scripts.runtimeconfig.json";
const SCRIPT_ASSEMBLY_NAME: &str = "Scripts";
const SCRIPT_HOST_TYPE: &str = "ZeroEngine.ZEScript";

pub type ScriptFn = <fn(u64) as WithAbi<AbiSystem>>::F;
pub type ScriptInitFn = <fn(u64, *const EngineAPI, *const u8, i32) as WithAbi<AbiSystem>>::F;
pub type ScriptEntityFn = <fn(u64, u64) as WithAbi<AbiSystem>>::F;

#[repr(C)]
pub struct EngineAPI {
	pub is_key_pressed: extern "C" fn(i32) -> bool,
	pub is_key_just_pressed: extern "C" fn(i32) -> bool,
	pub is_key_released: extern "C" fn(i32) -> bool,
	pub is_key_just_released: extern "C" fn(i32) -> bool,
	pub is_mouse_button_pressed: extern "C" fn(i32) -> bool,
	pub is_mouse_button_just_pressed: extern "C" fn(i32) -> bool,
	pub get_mouse_position: extern "C" fn(*mut f32, *mut f32),
	pub get_mouse_delta: extern "C" fn(*mut f32, *mut f32),
	pub get_time_state_ptr: extern "C" fn() -> *const ScriptingTimeState,
	pub has_component: extern "C" fn(u64, u32) -> bool,
	pub get_velocity: extern "C" fn(u64, *mut f32, *mut f32),
	pub add_2d_force: extern "C" fn(u64, f32, f32),
	pub add_2d_impulse: extern "C" fn(u64, f32, f32),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ScriptingTimeState {
	pub delta_time: f32,
	pub fixed_delta_time: f32,
	pub unscaled_delta_time: f32,
	pub time_scale: f32,
	pub time_since_startup: f64,
	pub unscaled_time_since_startup: f64,
	pub fixed_time: f64,
	pub frame_count: u64,
	pub fixed_frame_count: u64,
	pub is_fixed_update: u8,
}

impl ScriptingTimeState {
	const fn initial() -> Self {
		Self {
			delta_time: 0.0,
			fixed_delta_time: 0.0,
			unscaled_delta_time: 0.0,
			time_scale: 1.0,
			time_since_startup: 0.0,
			unscaled_time_since_startup: 0.0,
			fixed_time: 0.0,
			frame_count: 0,
			fixed_frame_count: 0,
			is_fixed_update: 0,
		}
	}
}

static ENGINE_API: EngineAPI = EngineAPI {
	is_key_pressed: api::is_key_pressed,
	is_key_just_pressed: api::is_key_just_pressed,
	is_key_released: api::is_key_released,
	is_key_just_released: api::is_key_just_released,
	is_mouse_button_pressed: api::is_mouse_button_pressed,
	is_mouse_button_just_pressed: api::is_mouse_button_just_pressed,
	get_mouse_position: api::get_mouse_position,
	get_mouse_delta: api::get_mouse_delta,
	get_time_state_ptr: api::get_time_state_ptr,
	has_component: api::has_component,
	get_velocity: api::get_velocity,
	add_2d_force: api::add_2d_force,
	add_2d_impulse: api::add_2d_impulse,
};

#[derive(Component, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Script {
	pub path: String,
	#[serde(default = "default_enabled")]
	pub enabled: bool,
}

impl Default for Script {
	fn default() -> Self {
		Self {
			path: "Scripts.Script".to_string(),
			enabled: true,
		}
	}
}

pub fn register_scripting_components(registry: &mut ComponentRegistry) {
	registry.register::<Script>("ze.scripting.script");
}

pub struct ScriptingEngine {
	_context: HostfxrContext<InitializedForRuntimeConfig>,
	loader: AssemblyDelegateLoader,
}

impl ScriptingEngine {
	pub fn new() -> Result<Self> { Self::from_paths(ASSEMBLY_PATH.into(), RUNTIME_CONFIG_PATH.into()) }

	pub fn from_paths(assembly_path: PathBuf, runtime_config_path: PathBuf) -> Result<Self> {
		let assembly_path = normalize_path(assembly_path)?;
		let runtime_config_path = normalize_path(runtime_config_path)?;

		let assembly_path = PdCString::from_os_str(&assembly_path)
			.with_context(|| format!("failed to encode C# assembly path {}", assembly_path.display()))?;
		let runtime_config_path = PdCString::from_os_str(&runtime_config_path).with_context(|| {
			format!(
				"failed to encode C# runtime config path {}",
				runtime_config_path.display()
			)
		})?;

		let hostfxr = load_hostfxr().context("failed to load .NET hostfxr")?;
		let context = hostfxr
			.initialize_for_runtime_config(&runtime_config_path)
			.context("failed to initialize .NET runtime for Scripts")?;
		let loader = context
			.get_delegate_loader_for_assembly(assembly_path)
			.context("failed to load C# assembly")?;

		Ok(Self {
			_context: context,
			loader,
		})
	}

	pub fn load_script(&self, class_path: &str) -> Result<ScriptLifecycle> {
		ScriptLifecycle::load(&self.loader, class_path)
	}
}

pub struct ScriptLifecycle {
	pub on_engine_init: Option<ScriptInitFn>,
	pub on_create: Option<ScriptFn>,
	pub on_start: Option<ScriptFn>,
	pub on_destroy: Option<ScriptFn>,
	pub on_update: Option<ScriptFn>,
	pub on_fixed_update: Option<ScriptFn>,
	pub on_enable: Option<ScriptFn>,
	pub on_disable: Option<ScriptFn>,
	pub on_contact_enter: Option<ScriptEntityFn>,
	pub on_contact_stay: Option<ScriptEntityFn>,
	pub on_contact_exit: Option<ScriptEntityFn>,
	pub on_sensor_enter: Option<ScriptEntityFn>,
	pub on_sensor_stay: Option<ScriptEntityFn>,
	pub on_sensor_exit: Option<ScriptEntityFn>,
}

impl ScriptLifecycle {
	fn load(loader: &AssemblyDelegateLoader, class_path: &str) -> Result<Self> {
		let type_name = PdCString::from_os_str(format!("{SCRIPT_HOST_TYPE}, {SCRIPT_ASSEMBLY_NAME}"))
			.with_context(|| format!("failed to encode C# script host type `{SCRIPT_HOST_TYPE}`"))?;

		Ok(Self {
			on_engine_init: load_optional_function::<fn(u64, *const EngineAPI, *const u8, i32)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnEngineInit",
				pdcstr!("NativeOnEngineInit"),
			)?,
			on_create: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnCreate",
				pdcstr!("NativeOnCreate"),
			)?,
			on_start: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnStart",
				pdcstr!("NativeOnStart"),
			)?,
			on_destroy: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnDestroy",
				pdcstr!("NativeOnDestroy"),
			)?,
			on_update: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnUpdate",
				pdcstr!("NativeOnUpdate"),
			)?,
			on_fixed_update: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnFixedUpdate",
				pdcstr!("NativeOnFixedUpdate"),
			)?,
			on_enable: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnEnable",
				pdcstr!("NativeOnEnable"),
			)?,
			on_disable: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnDisable",
				pdcstr!("NativeOnDisable"),
			)?,
			on_contact_enter: load_optional_function::<fn(u64, u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnContactEnter",
				pdcstr!("NativeOnContactEnter"),
			)?,
			on_contact_stay: load_optional_function::<fn(u64, u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnContactStay",
				pdcstr!("NativeOnContactStay"),
			)?,
			on_contact_exit: load_optional_function::<fn(u64, u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnContactExit",
				pdcstr!("NativeOnContactExit"),
			)?,
			on_sensor_enter: load_optional_function::<fn(u64, u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnSensorEnter",
				pdcstr!("NativeOnSensorEnter"),
			)?,
			on_sensor_stay: load_optional_function::<fn(u64, u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnSensorStay",
				pdcstr!("NativeOnSensorStay"),
			)?,
			on_sensor_exit: load_optional_function::<fn(u64, u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"NativeOnSensorExit",
				pdcstr!("NativeOnSensorExit"),
			)?,
		})
	}

	pub fn on_engine_init(&self, entity: EntityId, class_path: &str) {
		if let Some(on_engine_init) = self.on_engine_init {
			let class_path_len = i32::try_from(class_path.len()).unwrap_or(i32::MAX);
			on_engine_init(
				entity_id_to_script_arg(entity),
				&raw const ENGINE_API,
				class_path.as_ptr(),
				class_path_len,
			);
		}
	}

	pub fn on_create(&self, entity: EntityId) {
		if let Some(on_create) = self.on_create {
			on_create(entity_id_to_script_arg(entity));
		}
	}

	pub fn on_start(&self, entity: EntityId) {
		if let Some(on_start) = self.on_start {
			on_start(entity_id_to_script_arg(entity));
		}
	}

	pub fn on_destroy(&self, entity: EntityId) {
		if let Some(on_destroy) = self.on_destroy {
			on_destroy(entity_id_to_script_arg(entity));
		}
	}

	pub fn on_update(&self, entity: EntityId) {
		if let Some(on_update) = self.on_update {
			on_update(entity_id_to_script_arg(entity));
		}
	}

	pub fn on_fixed_update(&self, entity: EntityId) {
		if let Some(on_fixed_update) = self.on_fixed_update {
			on_fixed_update(entity_id_to_script_arg(entity));
		}
	}

	pub fn on_enable(&self, entity: EntityId) {
		if let Some(on_enable) = self.on_enable {
			on_enable(entity_id_to_script_arg(entity));
		}
	}

	pub fn on_disable(&self, entity: EntityId) {
		if let Some(on_disable) = self.on_disable {
			on_disable(entity_id_to_script_arg(entity));
		}
	}

	pub fn on_contact_enter(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(on_contact_enter) = self.on_contact_enter {
			on_contact_enter(entity_id_to_script_arg(entity), entity_id_to_script_arg(other_entity));
		}
	}

	pub fn on_contact_stay(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(on_contact_stay) = self.on_contact_stay {
			on_contact_stay(entity_id_to_script_arg(entity), entity_id_to_script_arg(other_entity));
		}
	}

	pub fn on_contact_exit(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(on_contact_exit) = self.on_contact_exit {
			on_contact_exit(entity_id_to_script_arg(entity), entity_id_to_script_arg(other_entity));
		}
	}

	pub fn on_sensor_enter(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(on_sensor_enter) = self.on_sensor_enter {
			on_sensor_enter(entity_id_to_script_arg(entity), entity_id_to_script_arg(other_entity));
		}
	}

	pub fn on_sensor_stay(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(on_sensor_stay) = self.on_sensor_stay {
			on_sensor_stay(entity_id_to_script_arg(entity), entity_id_to_script_arg(other_entity));
		}
	}

	pub fn on_sensor_exit(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(on_sensor_exit) = self.on_sensor_exit {
			on_sensor_exit(entity_id_to_script_arg(entity), entity_id_to_script_arg(other_entity));
		}
	}
}

pub fn entity_id_to_script_arg(entity: EntityId) -> u64 {
	((u64::from(entity.r#gen())) << 32) | (entity.index() & 0xffff_ffff)
}

pub const fn script_arg_to_entity_id(entity: u64) -> EntityId {
	let index = entity & 0xffff_ffff;
	let generation = (entity >> 32) as u16;
	EntityId::new_from_index_and_gen(index, generation)
}

#[derive(Debug, Clone, Copy)]
pub enum ScriptingApiCommand {
	Add2DForce { entity: EntityId, x: f32, y: f32 },
	Add2DImpulse { entity: EntityId, x: f32, y: f32 },
}

pub fn refresh_scripting_api_velocity_cache(velocities: impl IntoIterator<Item = (EntityId, f32, f32)>) {
	api::refresh_velocity_cache(velocities);
}

pub fn drain_scripting_api_commands() -> Vec<ScriptingApiCommand> { api::drain_commands() }

struct ScriptInstance {
	path: String,
	lifecycle: ScriptLifecycle,
	started: bool,
	enabled: bool,
}

struct ScriptingRuntime {
	engine: ScriptingEngine,
	instances: HashMap<EntityId, ScriptInstance>,
}

impl ScriptingRuntime {
	fn new() -> Result<Self> {
		Ok(Self {
			engine: ScriptingEngine::new()?,
			instances: HashMap::new(),
		})
	}

	fn sync_instances(&mut self, scripts: &[(EntityId, Script)]) -> Result<()> {
		let removed = self
			.instances
			.keys()
			.copied()
			.filter(|entity| !scripts.iter().any(|(script_entity, _)| script_entity == entity))
			.collect::<Vec<_>>();

		for entity in removed {
			self.destroy_instance(entity);
		}

		for (entity, script) in scripts {
			if self
				.instances
				.get(entity)
				.is_some_and(|instance| instance.path != script.path)
			{
				self.destroy_instance(*entity);
			}

			if !self.instances.contains_key(entity) {
				if script.enabled {
					let instance = self.create_instance(*entity, script)?;
					self.instances.insert(*entity, instance);
				}
				continue;
			}

			let Some(instance) = self.instances.get_mut(entity) else {
				continue;
			};

			match (script.enabled, instance.enabled) {
				(true, false) => {
					instance.lifecycle.on_enable(*entity);
					instance.enabled = true;
				}
				(false, true) => {
					instance.lifecycle.on_disable(*entity);
					instance.enabled = false;
				}
				_ => {}
			}
		}

		Ok(())
	}

	fn create_instance(&self, entity: EntityId, script: &Script) -> Result<ScriptInstance> {
		let lifecycle = self.engine.load_script(&script.path)?;
		lifecycle.on_engine_init(entity, &script.path);
		lifecycle.on_create(entity);
		lifecycle.on_enable(entity);

		Ok(ScriptInstance {
			path: script.path.clone(),
			lifecycle,
			started: false,
			enabled: true,
		})
	}

	fn destroy_instance(&mut self, entity: EntityId) {
		let Some(instance) = self.instances.remove(&entity) else {
			return;
		};

		if instance.enabled {
			instance.lifecycle.on_disable(entity);
		}

		instance.lifecycle.on_destroy(entity);
	}

	fn update(&mut self, scene: &Scene, dt: f32) -> Result<()> {
		api::begin_update(dt);
		api::refresh_scene_cache(scene);
		let scripts = scene_scripts(scene);
		self.sync_instances(&scripts)?;

		for (entity, instance) in self.instances.iter_mut().filter(|(_, instance)| instance.enabled) {
			if !instance.started {
				instance.lifecycle.on_start(*entity);
				instance.started = true;
			}

			instance.lifecycle.on_update(*entity);
		}

		Ok(())
	}

	fn fixed_update(&mut self, scene: &Scene, dt: f32) -> Result<()> {
		api::begin_fixed_update(dt);
		api::refresh_scene_cache(scene);
		let scripts = scene_scripts(scene);
		self.sync_instances(&scripts)?;

		for (entity, instance) in self.instances.iter().filter(|(_, instance)| instance.enabled) {
			instance.lifecycle.on_fixed_update(*entity);
		}

		Ok(())
	}

	fn contact_enter(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance.lifecycle.on_contact_enter(entity, other_entity);
		}
	}

	fn contact_stay(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance.lifecycle.on_contact_stay(entity, other_entity);
		}
	}

	fn contact_exit(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance.lifecycle.on_contact_exit(entity, other_entity);
		}
	}

	fn sensor_enter(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance.lifecycle.on_sensor_enter(entity, other_entity);
		}
	}

	fn sensor_stay(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance.lifecycle.on_sensor_stay(entity, other_entity);
		}
	}

	fn sensor_exit(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance.lifecycle.on_sensor_exit(entity, other_entity);
		}
	}

	fn enabled_instance(&self, entity: EntityId) -> Option<&ScriptInstance> {
		self.instances.get(&entity).filter(|instance| instance.enabled)
	}

	fn destroy_all(&mut self) {
		let entities = self.instances.keys().copied().collect::<Vec<_>>();
		for entity in entities {
			self.destroy_instance(entity);
		}
	}
}

#[derive(Clone)]
pub struct ScriptingRuntimeHandle(Rc<RefCell<ScriptingRuntime>>);

impl ScriptingRuntimeHandle {
	pub fn new() -> Result<Self> { Ok(Self(Rc::new(RefCell::new(ScriptingRuntime::new()?)))) }

	pub fn update(&self, scene: &Scene, dt: f32) -> Result<()> { self.0.borrow_mut().update(scene, dt) }

	pub fn fixed_update(&self, scene: &Scene, dt: f32) -> Result<()> { self.0.borrow_mut().fixed_update(scene, dt) }

	pub fn on_contact_enter(&self, entity: EntityId, other_entity: EntityId) {
		self.0.borrow().contact_enter(entity, other_entity);
	}

	pub fn on_contact_stay(&self, entity: EntityId, other_entity: EntityId) {
		self.0.borrow().contact_stay(entity, other_entity);
	}

	pub fn on_contact_exit(&self, entity: EntityId, other_entity: EntityId) {
		self.0.borrow().contact_exit(entity, other_entity);
	}

	pub fn on_sensor_enter(&self, entity: EntityId, other_entity: EntityId) {
		self.0.borrow().sensor_enter(entity, other_entity);
	}

	pub fn on_sensor_stay(&self, entity: EntityId, other_entity: EntityId) {
		self.0.borrow().sensor_stay(entity, other_entity);
	}

	pub fn on_sensor_exit(&self, entity: EntityId, other_entity: EntityId) {
		self.0.borrow().sensor_exit(entity, other_entity);
	}

	fn destroy_all(&self) { self.0.borrow_mut().destroy_all(); }
}

pub struct ScriptingSystem {
	runtime: ScriptingRuntimeHandle,
}

impl ScriptingSystem {
	pub fn new() -> Result<Self> {
		Ok(Self {
			runtime: ScriptingRuntimeHandle::new()?,
		})
	}

	pub const fn from_runtime(runtime: ScriptingRuntimeHandle) -> Self { Self { runtime } }

	pub fn runtime(&self) -> ScriptingRuntimeHandle { self.runtime.clone() }
}

impl System for ScriptingSystem {
	fn name(&self) -> &'static str { "ScriptingSystem" }

	fn update(&mut self, scene: &mut Scene, dt: f32) -> Result<()> { self.runtime.update(scene, dt) }

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

impl Drop for ScriptingSystem {
	fn drop(&mut self) { self.runtime.destroy_all(); }
}

fn scene_scripts(scene: &Scene) -> Vec<(EntityId, Script)> {
	let world = scene.world();
	let mut scripts = Vec::new();

	world.run(|entities: EntitiesView| {
		for entity in entities.iter() {
			let Ok(script) = world.get::<&Script>(entity) else {
				continue;
			};

			scripts.push((entity, script.clone()));
		}
	});

	scripts
}

fn load_optional_function<F>(
	loader: &AssemblyDelegateLoader,
	type_name: &PdCStr,
	class_path: &str,
	label: &'static str,
	method_name: &PdCStr,
) -> Result<Option<<F as WithAbi<AbiSystem>>::F>>
where
	F: FnPtr + WithAbi<AbiSystem>,
	<F as WithAbi<AbiSystem>>::F: Copy,
{
	match loader.get_function_with_unmanaged_callers_only::<F>(type_name, method_name) {
		Ok(function) => Ok(Some(*function)),
		Err(GetManagedFunctionError::TypeOrMethodNotFound) => Ok(None),
		Err(error) => Err(anyhow!(
			"failed to load C# script method `{class_path}.{label}`: {error}"
		)),
	}
}

fn normalize_path(path: PathBuf) -> Result<PathBuf> {
	if path.exists() {
		return Ok(path);
	}

	let repo_path = workspace_root()?.join(&path);
	if repo_path.exists() {
		return Ok(repo_path);
	}

	Err(anyhow!("path does not exist: {}", path.display()))
}

fn workspace_root() -> Result<PathBuf> {
	let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

	manifest_dir
		.ancestors()
		.find(|path| path.join("Cargo.toml").is_file() && path.join("ZeroEngine").is_dir())
		.map(PathBuf::from)
		.ok_or_else(|| anyhow!("failed to locate ZeroEngine workspace root"))
}

fn load_hostfxr() -> Result<Hostfxr> {
	let hostfxr_path = find_hostfxr_path().context("failed to locate libhostfxr in a .NET installation")?;
	Hostfxr::load_from_path(&hostfxr_path)
		.with_context(|| format!("failed to load hostfxr from {}", hostfxr_path.display()))
}

fn find_hostfxr_path() -> Option<PathBuf> { dotnet_roots().into_iter().find_map(|root| latest_hostfxr_path(&root)) }

fn dotnet_roots() -> Vec<PathBuf> {
	let mut roots = Vec::new();

	if let Some(root) = env::var_os("DOTNET_ROOT") {
		roots.push(root.into());
	}
	if let Some(home) = env::var_os("HOME") {
		roots.push(PathBuf::from(home).join(".dotnet"));
	}

	roots.push(PathBuf::from("/usr/share/dotnet"));
	roots.push(PathBuf::from("/usr/local/share/dotnet"));
	roots
}

fn latest_hostfxr_path(dotnet_root: &Path) -> Option<PathBuf> {
	let fxr_root = dotnet_root.join("host/fxr");
	let mut versions = fs::read_dir(fxr_root)
		.ok()?
		.filter_map(std::result::Result::ok)
		.filter(|entry| entry.file_type().is_ok_and(|file_type| file_type.is_dir()))
		.map(|entry| entry.path())
		.collect::<Vec<_>>();

	versions.sort();
	versions.reverse();

	versions
		.into_iter()
		.map(|version_dir| version_dir.join(hostfxr_library_name()))
		.find(|path| path.exists())
}

const fn hostfxr_library_name() -> &'static str {
	if cfg!(windows) {
		"hostfxr.dll"
	} else if cfg!(target_os = "macos") {
		"libhostfxr.dylib"
	} else {
		"libhostfxr.so"
	}
}

const fn default_enabled() -> bool { true }

mod api {
	use std::{
		cell::{RefCell, UnsafeCell},
		collections::{HashMap, HashSet},
	};

	use ze_ecs::{
		Children, Collider, EntitiesView, EntityId, Inactive, Name, Parent, PhysicsSettings, RigidBody, Scene, Tag,
		Transform,
	};
	use ze_input::{Input, ZKeyCode, ZMouseCode};
	use ze_renderer::{Camera, Sprite};

	use crate::{Script, ScriptingApiCommand, ScriptingTimeState, script_arg_to_entity_id};

	#[derive(Default)]
	struct ApiState {
		commands: Vec<ScriptingApiCommand>,
		components: HashSet<(EntityId, ComponentKind)>,
		velocities: HashMap<EntityId, (f32, f32)>,
	}

	struct TimeStateCell(UnsafeCell<ScriptingTimeState>);

	unsafe impl Sync for TimeStateCell {}

	static TIME_STATE: TimeStateCell = TimeStateCell(UnsafeCell::new(ScriptingTimeState::initial()));

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	enum ComponentKind {
		Name = 1,
		Tag = 2,
		Transform = 3,
		Parent = 4,
		Children = 5,
		Inactive = 6,
		Rigidbody = 7,
		PhysicsSettings = 8,
		Collider = 9,
		Sprite = 10,
		Camera = 11,
		Script = 12,
	}

	thread_local! {
		static API_STATE: RefCell<ApiState> = RefCell::new(ApiState::default());
	}

	pub fn drain_commands() -> Vec<ScriptingApiCommand> {
		API_STATE.with(|state| state.borrow_mut().commands.drain(..).collect())
	}

	fn time_state_mut() -> &'static mut ScriptingTimeState { unsafe { &mut *TIME_STATE.0.get() } }

	pub fn begin_update(dt: f32) {
		let dt = dt.max(0.0);
		let time = time_state_mut();
		let time_scale = time.time_scale;
		time.delta_time = dt * time_scale;
		time.unscaled_delta_time = dt;
		time.time_since_startup += f64::from(time.delta_time);
		time.unscaled_time_since_startup += f64::from(dt);
		time.frame_count = time.frame_count.saturating_add(1);
		time.is_fixed_update = 0;
	}

	pub fn begin_fixed_update(dt: f32) {
		let dt = dt.max(0.0);
		let time = time_state_mut();
		let time_scale = time.time_scale;
		time.delta_time = dt * time_scale;
		time.fixed_delta_time = dt;
		time.unscaled_delta_time = dt;
		time.fixed_time += f64::from(dt);
		time.fixed_frame_count = time.fixed_frame_count.saturating_add(1);
		time.is_fixed_update = 1;
	}

	pub fn refresh_scene_cache(scene: &Scene) {
		let world = scene.world();
		let mut components = HashSet::new();

		world.run(|entities: EntitiesView| {
			for entity in entities.iter() {
				if world.get::<&Name>(entity).is_ok() {
					components.insert((entity, ComponentKind::Name));
				}
				if world.get::<&Tag>(entity).is_ok() {
					components.insert((entity, ComponentKind::Tag));
				}
				if world.get::<&Transform>(entity).is_ok() {
					components.insert((entity, ComponentKind::Transform));
				}
				if world.get::<&Parent>(entity).is_ok() {
					components.insert((entity, ComponentKind::Parent));
				}
				if world.get::<&Children>(entity).is_ok() {
					components.insert((entity, ComponentKind::Children));
				}
				if world.get::<&Inactive>(entity).is_ok() {
					components.insert((entity, ComponentKind::Inactive));
				}
				if world.get::<&RigidBody>(entity).is_ok() {
					components.insert((entity, ComponentKind::Rigidbody));
				}
				if world.get::<&PhysicsSettings>(entity).is_ok() {
					components.insert((entity, ComponentKind::PhysicsSettings));
				}
				if world.get::<&Collider>(entity).is_ok() {
					components.insert((entity, ComponentKind::Collider));
				}
				if world.get::<&Sprite>(entity).is_ok() {
					components.insert((entity, ComponentKind::Sprite));
				}
				if world.get::<&Camera>(entity).is_ok() {
					components.insert((entity, ComponentKind::Camera));
				}
				if world.get::<&Script>(entity).is_ok() {
					components.insert((entity, ComponentKind::Script));
				}
			}
		});

		API_STATE.with(|state| {
			state.borrow_mut().components = components;
		});
	}

	pub fn refresh_velocity_cache(velocities: impl IntoIterator<Item = (EntityId, f32, f32)>) {
		API_STATE.with(|state| {
			state.borrow_mut().velocities = velocities.into_iter().map(|(entity, x, y)| (entity, (x, y))).collect();
		});
	}

	pub extern "C" fn is_key_pressed(key: i32) -> bool { Input::is_key_pressed(key_code_from_i32(key)) }

	pub extern "C" fn is_key_just_pressed(key: i32) -> bool { Input::is_key_just_pressed(key_code_from_i32(key)) }

	pub extern "C" fn is_key_released(key: i32) -> bool { Input::is_key_released(key_code_from_i32(key)) }

	pub extern "C" fn is_key_just_released(key: i32) -> bool { Input::is_key_just_released(key_code_from_i32(key)) }

	pub extern "C" fn is_mouse_button_pressed(button: i32) -> bool {
		Input::is_mouse_button_pressed(mouse_code_from_i32(button))
	}

	pub extern "C" fn is_mouse_button_just_pressed(button: i32) -> bool {
		Input::is_mouse_button_just_pressed(mouse_code_from_i32(button))
	}

	pub extern "C" fn get_mouse_position(out_x: *mut f32, out_y: *mut f32) {
		if out_x.is_null() || out_y.is_null() {
			return;
		}

		let position = Input::get_mouse_pos();
		write_position(out_x, out_y, position.x, position.y);
	}

	pub extern "C" fn get_mouse_delta(out_x: *mut f32, out_y: *mut f32) {
		if out_x.is_null() || out_y.is_null() {
			return;
		}

		let delta = Input::get_mouse_delta();
		write_position(out_x, out_y, delta.x, delta.y);
	}

	pub extern "C" fn get_time_state_ptr() -> *const ScriptingTimeState { TIME_STATE.0.get().cast_const() }

	pub extern "C" fn has_component(entity: u64, component_type: u32) -> bool {
		let entity = script_arg_to_entity_id(entity);
		let Some(component_kind) = component_kind_from_u32(component_type) else {
			return false;
		};

		API_STATE.with(|state| state.borrow().components.contains(&(entity, component_kind)))
	}

	pub extern "C" fn get_velocity(entity: u64, out_x: *mut f32, out_y: *mut f32) {
		if out_x.is_null() || out_y.is_null() {
			return;
		}

		let entity = script_arg_to_entity_id(entity);
		let (x, y) = API_STATE
			.with(|state| state.borrow().velocities.get(&entity).copied())
			.unwrap_or((0.0, 0.0));

		write_position(out_x, out_y, x, y);
	}

	pub extern "C" fn add_2d_force(entity: u64, x: f32, y: f32) {
		push_command(ScriptingApiCommand::Add2DForce {
			entity: script_arg_to_entity_id(entity),
			x,
			y,
		});
	}

	pub extern "C" fn add_2d_impulse(entity: u64, x: f32, y: f32) {
		push_command(ScriptingApiCommand::Add2DImpulse {
			entity: script_arg_to_entity_id(entity),
			x,
			y,
		});
	}

	fn push_command(command: ScriptingApiCommand) {
		API_STATE.with(|state| {
			state.borrow_mut().commands.push(command);
		});
	}

	fn write_position(out_x: *mut f32, out_y: *mut f32, x: f32, y: f32) {
		unsafe {
			*out_x = x;
			*out_y = y;
		}
	}

	const fn key_code_from_i32(key: i32) -> ZKeyCode {
		match key {
			0 => ZKeyCode::Escape,
			1 => ZKeyCode::Space,
			2 => ZKeyCode::Q,
			3 => ZKeyCode::W,
			4 => ZKeyCode::E,
			5 => ZKeyCode::R,
			6 => ZKeyCode::T,
			7 => ZKeyCode::Y,
			8 => ZKeyCode::U,
			9 => ZKeyCode::I,
			10 => ZKeyCode::O,
			11 => ZKeyCode::P,
			12 => ZKeyCode::A,
			13 => ZKeyCode::S,
			14 => ZKeyCode::D,
			15 => ZKeyCode::F,
			16 => ZKeyCode::G,
			17 => ZKeyCode::H,
			18 => ZKeyCode::J,
			19 => ZKeyCode::K,
			20 => ZKeyCode::L,
			21 => ZKeyCode::Z,
			22 => ZKeyCode::X,
			23 => ZKeyCode::C,
			24 => ZKeyCode::V,
			25 => ZKeyCode::B,
			26 => ZKeyCode::N,
			27 => ZKeyCode::M,
			28 => ZKeyCode::Enter,
			29 => ZKeyCode::LCtrl,
			30 => ZKeyCode::LShift,
			31 => ZKeyCode::K1,
			32 => ZKeyCode::K2,
			33 => ZKeyCode::K3,
			34 => ZKeyCode::K4,
			35 => ZKeyCode::K5,
			36 => ZKeyCode::K6,
			37 => ZKeyCode::K7,
			38 => ZKeyCode::K8,
			39 => ZKeyCode::K9,
			40 => ZKeyCode::K0,
			41 => ZKeyCode::KF1,
			42 => ZKeyCode::KF2,
			43 => ZKeyCode::KF3,
			44 => ZKeyCode::KF4,
			45 => ZKeyCode::KF5,
			46 => ZKeyCode::KF6,
			47 => ZKeyCode::KF7,
			48 => ZKeyCode::KF8,
			49 => ZKeyCode::KF9,
			50 => ZKeyCode::KF10,
			51 => ZKeyCode::KF11,
			52 => ZKeyCode::KF12,
			_ => ZKeyCode::Unknown,
		}
	}

	const fn mouse_code_from_i32(button: i32) -> ZMouseCode {
		match button {
			0 => ZMouseCode::Left,
			1 => ZMouseCode::Right,
			2 => ZMouseCode::Middle,
			3 => ZMouseCode::Back,
			4 => ZMouseCode::Forward,
			_ => ZMouseCode::Other,
		}
	}

	const fn component_kind_from_u32(component_type: u32) -> Option<ComponentKind> {
		match component_type {
			1 => Some(ComponentKind::Name),
			2 => Some(ComponentKind::Tag),
			3 => Some(ComponentKind::Transform),
			4 => Some(ComponentKind::Parent),
			5 => Some(ComponentKind::Children),
			6 => Some(ComponentKind::Inactive),
			7 => Some(ComponentKind::Rigidbody),
			8 => Some(ComponentKind::PhysicsSettings),
			9 => Some(ComponentKind::Collider),
			10 => Some(ComponentKind::Sprite),
			11 => Some(ComponentKind::Camera),
			12 => Some(ComponentKind::Script),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn calls_csharp_on_update() -> Result<()> {
		let engine = ScriptingEngine::new()?;
		let lifecycle = engine.load_script("Scripts.Script")?;
		let entity = EntityId::new_from_index_and_gen(4, 0);
		lifecycle.on_engine_init(entity, "Scripts.Script");
		lifecycle.on_update(entity);
		Ok(())
	}
}
