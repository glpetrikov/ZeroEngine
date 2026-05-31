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

pub type ScriptFn = <fn() as WithAbi<AbiSystem>>::F;
pub type ScriptUpdateFn = <fn(f32) as WithAbi<AbiSystem>>::F;
pub type ScriptEntityFn = <fn(u64) as WithAbi<AbiSystem>>::F;

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
	pub on_create: Option<ScriptFn>,
	pub on_start: Option<ScriptFn>,
	pub on_destroy: Option<ScriptFn>,
	pub on_update: Option<ScriptUpdateFn>,
	pub on_fixed_update: Option<ScriptUpdateFn>,
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
		let type_name = PdCString::from_os_str(format!("{class_path}, {SCRIPT_ASSEMBLY_NAME}"))
			.with_context(|| format!("failed to encode C# script class path `{class_path}`"))?;

		Ok(Self {
			on_create: load_optional_function::<fn()>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnCreate",
				pdcstr!("OnCreate"),
			)?,
			on_start: load_optional_function::<fn()>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnStart",
				pdcstr!("OnStart"),
			)?,
			on_destroy: load_optional_function::<fn()>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnDestroy",
				pdcstr!("OnDestroy"),
			)?,
			on_update: load_optional_function::<fn(f32)>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnUpdate",
				pdcstr!("OnUpdate"),
			)?,
			on_fixed_update: load_optional_function::<fn(f32)>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnFixedUpdate",
				pdcstr!("OnFixedUpdate"),
			)?,
			on_enable: load_optional_function::<fn()>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnEnable",
				pdcstr!("OnEnable"),
			)?,
			on_disable: load_optional_function::<fn()>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnDisable",
				pdcstr!("OnDisable"),
			)?,
			on_contact_enter: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnContactEnter",
				pdcstr!("OnContactEnter"),
			)?,
			on_contact_stay: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnContactStay",
				pdcstr!("OnContactStay"),
			)?,
			on_contact_exit: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnContactExit",
				pdcstr!("OnContactExit"),
			)?,
			on_sensor_enter: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnSensorEnter",
				pdcstr!("OnSensorEnter"),
			)?,
			on_sensor_stay: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnSensorStay",
				pdcstr!("OnSensorStay"),
			)?,
			on_sensor_exit: load_optional_function::<fn(u64)>(
				loader,
				type_name.as_ref(),
				class_path,
				"OnSensorExit",
				pdcstr!("OnSensorExit"),
			)?,
		})
	}

	pub fn on_create(&self) {
		if let Some(on_create) = self.on_create {
			on_create();
		}
	}

	pub fn on_start(&self) {
		if let Some(on_start) = self.on_start {
			on_start();
		}
	}

	pub fn on_destroy(&self) {
		if let Some(on_destroy) = self.on_destroy {
			on_destroy();
		}
	}

	pub fn on_update(&self, dt: f32) {
		if let Some(on_update) = self.on_update {
			on_update(dt);
		}
	}

	pub fn on_fixed_update(&self, dt: f32) {
		if let Some(on_fixed_update) = self.on_fixed_update {
			on_fixed_update(dt);
		}
	}

	pub fn on_enable(&self) {
		if let Some(on_enable) = self.on_enable {
			on_enable();
		}
	}

	pub fn on_disable(&self) {
		if let Some(on_disable) = self.on_disable {
			on_disable();
		}
	}

	pub fn on_contact_enter(&self, other_entity: u64) {
		if let Some(on_contact_enter) = self.on_contact_enter {
			on_contact_enter(other_entity);
		}
	}

	pub fn on_contact_stay(&self, other_entity: u64) {
		if let Some(on_contact_stay) = self.on_contact_stay {
			on_contact_stay(other_entity);
		}
	}

	pub fn on_contact_exit(&self, other_entity: u64) {
		if let Some(on_contact_exit) = self.on_contact_exit {
			on_contact_exit(other_entity);
		}
	}

	pub fn on_sensor_enter(&self, other_entity: u64) {
		if let Some(on_sensor_enter) = self.on_sensor_enter {
			on_sensor_enter(other_entity);
		}
	}

	pub fn on_sensor_stay(&self, other_entity: u64) {
		if let Some(on_sensor_stay) = self.on_sensor_stay {
			on_sensor_stay(other_entity);
		}
	}

	pub fn on_sensor_exit(&self, other_entity: u64) {
		if let Some(on_sensor_exit) = self.on_sensor_exit {
			on_sensor_exit(other_entity);
		}
	}
}

pub fn entity_id_to_script_arg(entity: EntityId) -> u64 { entity.index() }

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
					let instance = self.create_instance(script)?;
					self.instances.insert(*entity, instance);
				}
				continue;
			}

			let Some(instance) = self.instances.get_mut(entity) else {
				continue;
			};

			match (script.enabled, instance.enabled) {
				(true, false) => {
					instance.lifecycle.on_enable();
					instance.enabled = true;
				}
				(false, true) => {
					instance.lifecycle.on_disable();
					instance.enabled = false;
				}
				_ => {}
			}
		}

		Ok(())
	}

	fn create_instance(&self, script: &Script) -> Result<ScriptInstance> {
		let lifecycle = self.engine.load_script(&script.path)?;
		lifecycle.on_create();
		lifecycle.on_enable();

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
			instance.lifecycle.on_disable();
		}

		instance.lifecycle.on_destroy();
	}

	fn update(&mut self, scene: &Scene, dt: f32) -> Result<()> {
		let scripts = scene_scripts(scene);
		self.sync_instances(&scripts)?;

		for instance in self.instances.values_mut().filter(|instance| instance.enabled) {
			if !instance.started {
				instance.lifecycle.on_start();
				instance.started = true;
			}

			instance.lifecycle.on_update(dt);
		}

		Ok(())
	}

	fn fixed_update(&mut self, scene: &Scene, dt: f32) -> Result<()> {
		let scripts = scene_scripts(scene);
		self.sync_instances(&scripts)?;

		for instance in self.instances.values().filter(|instance| instance.enabled) {
			instance.lifecycle.on_fixed_update(dt);
		}

		Ok(())
	}

	fn contact_enter(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance
				.lifecycle
				.on_contact_enter(entity_id_to_script_arg(other_entity));
		}
	}

	fn contact_stay(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance
				.lifecycle
				.on_contact_stay(entity_id_to_script_arg(other_entity));
		}
	}

	fn contact_exit(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance
				.lifecycle
				.on_contact_exit(entity_id_to_script_arg(other_entity));
		}
	}

	fn sensor_enter(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance
				.lifecycle
				.on_sensor_enter(entity_id_to_script_arg(other_entity));
		}
	}

	fn sensor_stay(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance.lifecycle.on_sensor_stay(entity_id_to_script_arg(other_entity));
		}
	}

	fn sensor_exit(&self, entity: EntityId, other_entity: EntityId) {
		if let Some(instance) = self.enabled_instance(entity) {
			instance.lifecycle.on_sensor_exit(entity_id_to_script_arg(other_entity));
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

	pub fn from_runtime(runtime: ScriptingRuntimeHandle) -> Self { Self { runtime } }

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

fn find_hostfxr_path() -> Option<PathBuf> {
	dotnet_roots()
		.into_iter()
		.filter_map(|root| latest_hostfxr_path(&root))
		.next()
}

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
		.filter_map(|entry| entry.ok())
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

fn hostfxr_library_name() -> &'static str {
	if cfg!(windows) {
		"hostfxr.dll"
	} else if cfg!(target_os = "macos") {
		"libhostfxr.dylib"
	} else {
		"libhostfxr.so"
	}
}

fn default_enabled() -> bool { true }

mod api {
	pub(crate) fn log(_message_ptr: *const u8, _message_len: usize) {}

	pub(crate) fn spawn_entity() -> u64 { 0 }

	pub(crate) fn destroy_entity(_entity: u64) {}

	pub(crate) fn set_enabled(_entity: u64, _enabled: bool) {}

	pub(crate) fn get_component(_entity: u64, _component_type: u64, _out_component: *mut std::ffi::c_void) -> bool {
		false
	}
}

macro_rules! export_scripting_api {
	($(
		fn $export_name:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $return_ty:ty => $wrapper:path;
	)*) => {
		$(
			#[unsafe(no_mangle)]
			pub extern "C" fn $export_name($($arg: $arg_ty),*) -> $return_ty {
				$wrapper($($arg),*)
			}
		)*
	};
}

export_scripting_api! {
	fn ze_script_log(message_ptr: *const u8, message_len: usize) -> () => api::log;
	fn ze_script_spawn_entity() -> u64 => api::spawn_entity;
	fn ze_script_destroy_entity(entity: u64) -> () => api::destroy_entity;
	fn ze_script_set_enabled(entity: u64, enabled: bool) -> () => api::set_enabled;
	fn ze_script_get_component(entity: u64, component_type: u64, out_component: *mut std::ffi::c_void) -> bool => api::get_component;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn calls_csharp_on_update() -> Result<()> {
		let engine = ScriptingEngine::new()?;
		let lifecycle = engine.load_script("Scripts.Script")?;
		lifecycle.on_update(0.016);
		Ok(())
	}
}
