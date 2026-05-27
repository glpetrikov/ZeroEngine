use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shipyard::EntityId;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SaveFile {
	#[serde(rename = "$schema")]
	pub schema: String,

	pub version: String,
	pub entities: Vec<SavedEntity>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct SavedEntityId {
	pub index: u64,

	#[serde(rename = "gen")]
	pub generation: u16,
}

impl From<EntityId> for SavedEntityId {
	fn from(id: EntityId) -> Self {
		Self {
			index: id.index(),
			generation: id.r#gen(),
		}
	}
}

impl From<SavedEntityId> for EntityId {
	fn from(id: SavedEntityId) -> Self { EntityId::new_from_index_and_gen(id.index, id.generation) }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SavedEntity {
	pub id: SavedEntityId,
	pub components: Vec<SavedComponent>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SavedComponent {
	pub component_type: String,
	pub value: Value,
}
