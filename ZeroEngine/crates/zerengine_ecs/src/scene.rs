use std::{
	fs,
	path::{Path, PathBuf},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shipyard::{Component, EntitiesView, EntityId, World};
use zerengine_core::{Result, anyhow};

use crate::{
	components::{Inactive, Name, Tag, Transform},
	definitions::SaveFile,
	entity::Entity,
	registry::ComponentRegistry,
};

pub const SCENE_VERSION: &str = "0.1.0";
pub const SCENE_EXTENSION: &str = "zescene.json";
pub const SCENE_SCHEMA_FILE: &str = "zescene.schema.json";
pub const SCENE_SCHEMA_REF: &str = "../schemas/zescene.schema.json";

pub struct Scene {
	pub name: String,
	pub(crate) world: World,
	pub(crate) registry: ComponentRegistry,
}

impl Scene {
	pub fn new(name: &str) -> Self {
		let mut registry = ComponentRegistry::new();
		Self::register_defaults(&mut registry);

		Self {
			name: name.to_string(),
			world: World::new(),
			registry,
		}
	}

	pub fn from_registry(name: &str, registry: ComponentRegistry) -> Self {
		Self {
			name: name.to_string(),
			world: World::new(),
			registry,
		}
	}
}

impl Scene {
	pub fn world(&self) -> &World { &self.world }

	pub fn world_mut(&mut self) -> &mut World { &mut self.world }

	pub fn registry(&self) -> &ComponentRegistry { &self.registry }

	pub fn registry_mut(&mut self) -> &mut ComponentRegistry { &mut self.registry }
}

impl Scene {
	pub fn register_component<T>(&mut self, type_id: &str)
	where
		T: Component + Clone + Serialize + for<'de> Deserialize<'de> + JsonSchema + 'static,
	{
		self.registry.register::<T>(type_id);
	}

	pub fn register_defaults(registry: &mut ComponentRegistry) {
		registry.register::<Name>("ze.name");
		registry.register::<Tag>("ze.tag");
		registry.register::<Inactive>("ze.inactive");
		registry.register::<Transform>("ze.transform");
	}
}

impl Scene {
	pub fn create_entity(&mut self, name: &str) -> EntityId {
		self.world.add_entity((Name { name: name.to_string() },))
	}

	pub fn destroy_entity(&mut self, entity: EntityId) { self.world.delete_entity(entity); }

	pub fn entity_mut(&mut self, entity: EntityId) -> Entity<'_> { Entity::new(entity, self) }

	pub(crate) fn add_component<T>(&mut self, entity: EntityId, component: T)
	where
		T: Component + Clone + Serialize + for<'de> Deserialize<'de>,
	{
		self.world.add_component(entity, (component,));
	}
}

impl Scene {
	pub fn from_name(directory: PathBuf, name: &str) -> Result<Self> {
		let mut registry = ComponentRegistry::new();
		Self::register_defaults(&mut registry);
		Self::from_name_with_registry(directory, name, registry)
	}

	pub fn from_name_with_registry(directory: PathBuf, name: &str, registry: ComponentRegistry) -> Result<Self> {
		let path = scene_path(directory, name);
		Self::from_path_with_registry(path, registry)
	}

	pub fn from_path(path: PathBuf) -> Result<Self> {
		let mut registry = ComponentRegistry::new();
		Self::register_defaults(&mut registry);
		Self::from_path_with_registry(path, registry)
	}

	pub fn from_path_with_registry(path: PathBuf, registry: ComponentRegistry) -> Result<Self> {
		let json_text = fs::read_to_string(&path)?;
		let json_value: serde_json::Value = serde_json::from_str(&json_text)?;

		let schema = Self::schema_value();

		jsonschema::validate(&schema, &json_value).map_err(|err| anyhow!("Invalid scene file: {}", err))?;

		let save_file: SaveFile = serde_json::from_value(json_value)?;

		let name = path
			.file_name()
			.and_then(|name| name.to_str())
			.and_then(|name| name.strip_suffix(".zescene.json"))
			.unwrap_or("")
			.to_string();

		let mut world = World::new();

		for saved_entity in &save_file.entities {
			let entity = EntityId::from(saved_entity.id);
			world.spawn(entity);
		}

		for saved_entity in save_file.entities {
			let entity = EntityId::from(saved_entity.id);

			for component in saved_entity.components {
				registry
					.load_component(entity, &mut world, component)
					.map_err(|err| anyhow!("Cannot load scene: {:?}", err))?;
			}
		}

		Ok(Self { name, world, registry })
	}

	pub fn save(&self, directory: PathBuf, file_name: &str) -> Result<()> {
		fs::create_dir_all(&directory)?;

		let path = scene_path(&directory, file_name);

		let save_file = self.world.run(|entities: EntitiesView| SaveFile {
			schema: SCENE_SCHEMA_REF.to_string(),
			version: SCENE_VERSION.to_string(),
			entities: entities
				.iter()
				.map(|entity| self.registry.save_entity(entity, &self.world))
				.collect(),
		});

		let json = serde_json::to_string_pretty(&save_file)?;
		fs::write(path, json)?;

		Ok(())
	}

	pub fn write_schema(directory: PathBuf) -> Result<()> {
		fs::create_dir_all(&directory)?;

		let path = schema_path(directory);
		let schema = Self::schema_value();
		let json = serde_json::to_string_pretty(&schema)?;

		fs::write(path, json)?;

		Ok(())
	}

	pub fn schema_value() -> serde_json::Value {
		serde_json::to_value(schemars::schema_for!(SaveFile)).expect("scene schema serialization failed")
	}
}

fn scene_path(directory: impl AsRef<Path>, name: &str) -> PathBuf {
	directory.as_ref().join(format!("{name}.zescene.json"))
}

fn schema_path(directory: impl AsRef<Path>) -> PathBuf { directory.as_ref().join("zescene.schema.json") }
