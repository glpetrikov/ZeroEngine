use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shipyard::{Component, EntityId};
use ze_core::{Quat, Vec3};

#[derive(Component, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Name {
	pub name: String,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Tag {
	pub tag: String,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Transform {
	#[schemars(with = "[f32; 3]")]
	pub position: Vec3,

	#[schemars(with = "[f32; 3]")]
	pub scale: Vec3,

	#[schemars(with = "[f32; 4]")]
	pub rotation: Quat,
}

impl Default for Transform {
	fn default() -> Self {
		Self {
			position: Vec3::ZERO,
			scale: Vec3::ONE,
			rotation: Quat::IDENTITY,
		}
	}
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Parent {
	pub id: EntityId,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Children {
	pub ids: Vec<EntityId>,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Inactive;
