use std::{
	collections::{HashMap, HashSet},
	sync::mpsc::{Receiver, Sender, channel},
};

use rapier2d::prelude::*;
use ze_core::{Quat, Result, Vec2};
use ze_ecs::{
	Collider, ColliderShape, CollisionDetection, EntitiesView, EntityId, PhysicsSettings, RigidBody, RigidBodyType,
	Scene, System, Transform,
};
use ze_scripting_cs::{
	ScriptingApiCommand, ScriptingRuntimeHandle, drain_scripting_api_commands, refresh_scripting_api_velocity_cache,
};

pub const DEFAULT_GRAVITY: Vec2 = Vec2::new(0.0, -9.81);
pub const DEFAULT_PHYSICS_TIMESTEP: f32 = 1.0 / 70.0;

pub struct PhysicsWorld {
	pipeline: PhysicsPipeline,
	gravity: Vector<Real>,
	integration_parameters: IntegrationParameters,
	island_manager: IslandManager,
	broad_phase: BroadPhaseBvh,
	narrow_phase: NarrowPhase,
	pub rigid_bodies: RigidBodySet,
	pub colliders: ColliderSet,
	impulse_joints: ImpulseJointSet,
	multibody_joints: MultibodyJointSet,
	ccd_solver: CCDSolver,
	entity_bodies: HashMap<EntityId, PhysicsBodyEntry>,
	entity_colliders: HashMap<EntityId, ColliderHandle>,
	collider_entities: HashMap<ColliderHandle, EntityId>,
	active_contact_pairs: HashSet<ColliderPairKey>,
	active_sensor_pairs: HashSet<ColliderPairKey>,
	collision_event_sender: Sender<CollisionEvent>,
	collision_event_receiver: Receiver<CollisionEvent>,
	contact_force_event_sender: Sender<ContactForceEvent>,
	contact_force_event_receiver: Receiver<ContactForceEvent>,
}

#[derive(Debug, Clone, Copy)]
struct PhysicsBodyEntry {
	handle: RigidBodyHandle,
	body_type: RigidBodyType,
}

type ColliderPairKey = (ColliderHandle, ColliderHandle);

impl PhysicsWorld {
	pub fn new() -> Self {
		let (collision_event_sender, collision_event_receiver) = channel();
		let (contact_force_event_sender, contact_force_event_receiver) = channel();

		Self {
			pipeline: PhysicsPipeline::new(),
			gravity: vector![DEFAULT_GRAVITY.x, DEFAULT_GRAVITY.y],
			integration_parameters: IntegrationParameters::default(),
			island_manager: IslandManager::new(),
			broad_phase: BroadPhaseBvh::new(),
			narrow_phase: NarrowPhase::new(),
			rigid_bodies: RigidBodySet::new(),
			colliders: ColliderSet::new(),
			impulse_joints: ImpulseJointSet::new(),
			multibody_joints: MultibodyJointSet::new(),
			ccd_solver: CCDSolver::new(),
			entity_bodies: HashMap::new(),
			entity_colliders: HashMap::new(),
			collider_entities: HashMap::new(),
			active_contact_pairs: HashSet::new(),
			active_sensor_pairs: HashSet::new(),
			collision_event_sender,
			collision_event_receiver,
			contact_force_event_sender,
			contact_force_event_receiver,
		}
	}

	pub fn with_gravity(gravity: Vec2) -> Self {
		let mut world = Self::new();
		world.set_gravity(gravity);
		world
	}

	pub fn gravity(&self) -> Vec2 { Vec2::new(self.gravity.x, self.gravity.y) }

	pub const fn set_gravity(&mut self, gravity: Vec2) { self.gravity = vector![gravity.x, gravity.y]; }

	pub fn add_2d_force(&mut self, entity: EntityId, force: Vec2) {
		let Some(entry) = self.entity_bodies.get(&entity).copied() else {
			return;
		};
		let Some(body) = self.rigid_bodies.get_mut(entry.handle) else {
			return;
		};

		body.add_force(vector![force.x, force.y], true);
	}

	pub fn add_2d_impulse(&mut self, entity: EntityId, impulse: Vec2) {
		let Some(entry) = self.entity_bodies.get(&entity).copied() else {
			return;
		};
		let Some(body) = self.rigid_bodies.get_mut(entry.handle) else {
			return;
		};

		body.apply_impulse(vector![impulse.x, impulse.y], true);
	}

	pub fn body_velocities(&self) -> Vec<(EntityId, f32, f32)> {
		self.entity_bodies
			.iter()
			.filter_map(|(entity, entry)| {
				self.rigid_bodies.get(entry.handle).map(|body| {
					let velocity = body.linvel();
					(*entity, velocity.x, velocity.y)
				})
			})
			.collect()
	}

	pub fn step(&mut self, dt: f32) -> Vec<CollisionEvent> {
		self.integration_parameters.dt = dt.max(0.0);

		let event_handler = ChannelEventCollector::new(
			self.collision_event_sender.clone(),
			self.contact_force_event_sender.clone(),
		);

		self.pipeline.step(
			&self.gravity,
			&self.integration_parameters,
			&mut self.island_manager,
			&mut self.broad_phase,
			&mut self.narrow_phase,
			&mut self.rigid_bodies,
			&mut self.colliders,
			&mut self.impulse_joints,
			&mut self.multibody_joints,
			&mut self.ccd_solver,
			&(),
			&event_handler,
		);

		self.contact_force_event_receiver.try_iter().for_each(drop);
		self.collision_event_receiver.try_iter().collect()
	}

	pub fn register_entity(
		&mut self,
		entity: EntityId,
		rigid_body: &RigidBody,
		collider: &Collider,
		transform: &Transform,
	) {
		if self.entity_bodies.contains_key(&entity) {
			return;
		}

		let body = RigidBodyBuilder::new(to_rapier_body_type(rigid_body.body_type))
			.translation(vector![transform.position.x, transform.position.y])
			.rotation(transform.rotation.to_euler(ze_core::glam::EulerRot::XYZ).2)
			.gravity_scale(if rigid_body.use_gravity {
				rigid_body.gravity_scale
			} else {
				0.0
			})
			.linear_damping(rigid_body.linear_damping)
			.angular_damping(rigid_body.angular_damping)
			.locked_axes(to_locked_axes(rigid_body))
			.ccd_enabled(matches!(rigid_body.collision_detection, CollisionDetection::Continuous))
			.build();

		let body_handle = self.rigid_bodies.insert(body);
		let collider_handle = self.colliders.insert_with_parent(
			build_collider(collider, rigid_body.mass, transform.scale.truncate()),
			body_handle,
			&mut self.rigid_bodies,
		);

		self.entity_bodies.insert(
			entity,
			PhysicsBodyEntry {
				handle: body_handle,
				body_type: rigid_body.body_type,
			},
		);
		self.entity_colliders.insert(entity, collider_handle);
		self.collider_entities.insert(collider_handle, entity);
	}

	pub fn entity_for_collider(&self, collider: ColliderHandle) -> Option<EntityId> {
		self.collider_entities.get(&collider).copied()
	}

	pub fn collider_is_sensor(&self, collider: ColliderHandle) -> bool {
		self.colliders
			.get(collider)
			.is_some_and(rapier2d::geometry::Collider::is_sensor)
	}

	pub fn sync_from_ecs(&mut self, scene: &Scene) {
		let transforms = self
			.entity_bodies
			.iter()
			.filter(|(_, entry)| is_kinematic(entry.body_type))
			.filter_map(|(entity, _)| {
				scene
					.world()
					.get::<&Transform>(*entity)
					.ok()
					.map(|transform| (*entity, transform.position.x, transform.position.y, transform.rotation))
			})
			.collect::<Vec<_>>();

		for (entity, x, y, rotation) in transforms {
			let Some(entry) = self.entity_bodies.get(&entity).copied() else {
				continue;
			};
			let Some(body) = self.rigid_bodies.get_mut(entry.handle) else {
				continue;
			};

			let angle = rotation.to_euler(ze_core::glam::EulerRot::XYZ).2;
			body.set_position(Isometry::new(vector![x, y], angle), true);
		}
	}

	pub fn sync_to_ecs(&self, scene: &mut Scene) -> Result<()> {
		let updates = self
			.entity_bodies
			.iter()
			.filter_map(|(entity, entry)| {
				self.rigid_bodies.get(entry.handle).map(|body| {
					let position = body.translation();
					(*entity, position.x, position.y, body.rotation().angle())
				})
			})
			.collect::<Vec<_>>();

		for (entity, x, y, angle) in updates {
			let mut transform = scene.world_mut().get::<&mut Transform>(entity)?;
			transform.position.x = x;
			transform.position.y = y;
			transform.rotation = Quat::from_rotation_z(angle);
		}

		Ok(())
	}
}

impl Default for PhysicsWorld {
	fn default() -> Self { Self::new() }
}

pub struct PhysicsSystem {
	world: PhysicsWorld,
	initialized: bool,
	accumulator: f32,
	scripting: Option<ScriptingRuntimeHandle>,
}

impl PhysicsSystem {
	pub fn new() -> Self {
		Self {
			world: PhysicsWorld::new(),
			initialized: false,
			accumulator: 0.0,
			scripting: None,
		}
	}

	pub fn with_scripting(scripting: ScriptingRuntimeHandle) -> Self {
		Self {
			world: PhysicsWorld::new(),
			initialized: false,
			accumulator: 0.0,
			scripting: Some(scripting),
		}
	}

	pub const fn world(&self) -> &PhysicsWorld { &self.world }

	pub const fn world_mut(&mut self) -> &mut PhysicsWorld { &mut self.world }

	fn register_scene_bodies(&mut self, scene: &Scene) {
		let ecs_world = scene.world();
		let mut entities_to_register = Vec::new();

		ecs_world.run(|entities: EntitiesView| {
			for entity in entities.iter() {
				let Ok((transform, rigid_body, collider)) =
					ecs_world.get::<(&Transform, &RigidBody, &Collider)>(entity)
				else {
					continue;
				};

				entities_to_register.push((entity, rigid_body.clone(), collider.clone(), transform.clone()));
			}
		});

		for (entity, rigid_body, collider, transform) in entities_to_register {
			self.world.register_entity(entity, &rigid_body, &collider, &transform);
		}
	}
}

impl Default for PhysicsSystem {
	fn default() -> Self { Self::new() }
}

impl System for PhysicsSystem {
	fn name(&self) -> &'static str { "PhysicsSystem" }

	fn update(&mut self, scene: &mut Scene, dt: f32) -> Result<()> {
		if !self.initialized {
			self.register_scene_bodies(scene);
			self.initialized = true;
		}

		let settings = scene_physics_settings(scene);
		let fixed_dt = settings.physics_timestep.max(f32::EPSILON);

		self.accumulator += dt.max(0.0);
		let steps = (self.accumulator / fixed_dt) as usize;
		for _ in 0..steps {
			self.world.set_gravity(settings.gravity);
			self.apply_scripting_api_commands();
			self.world.sync_from_ecs(scene);
			let collision_events = self.world.step(fixed_dt);
			self.world.sync_to_ecs(scene)?;

			if let Some(scripting) = self.scripting.clone() {
				refresh_scripting_api_velocity_cache(self.world.body_velocities());
				scripting.fixed_update(scene, fixed_dt)?;
				self.apply_scripting_api_commands();
				self.dispatch_script_collision_events(&scripting, collision_events);
				self.apply_scripting_api_commands();
			}

			self.accumulator -= fixed_dt;
		}

		Ok(())
	}

	fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

impl PhysicsSystem {
	fn apply_scripting_api_commands(&mut self) {
		for command in drain_scripting_api_commands() {
			match command {
				ScriptingApiCommand::Add2DForce { entity, x, y } => {
					self.world.add_2d_force(entity, Vec2::new(x, y));
				}
				ScriptingApiCommand::Add2DImpulse { entity, x, y } => {
					self.world.add_2d_impulse(entity, Vec2::new(x, y));
				}
			}
		}
	}

	fn dispatch_script_collision_events(
		&mut self,
		scripting: &ScriptingRuntimeHandle,
		collision_events: Vec<CollisionEvent>,
	) {
		let previous_contact_pairs = self.world.active_contact_pairs.clone();
		let previous_sensor_pairs = self.world.active_sensor_pairs.clone();
		let mut contact_events = Vec::new();
		let mut sensor_events = Vec::new();

		for event in collision_events {
			if event.sensor() {
				sensor_events.push(event);
			} else {
				contact_events.push(event);
			}
		}

		for event in contact_events {
			self.dispatch_contact_event(scripting, event.collider1(), event.collider2(), event.started());
		}

		let contact_stays = self
			.world
			.active_contact_pairs
			.iter()
			.copied()
			.filter(|pair| previous_contact_pairs.contains(pair))
			.collect::<Vec<_>>();

		for (collider1, collider2) in contact_stays {
			self.dispatch_contact_stay(scripting, collider1, collider2);
		}

		for event in sensor_events {
			self.dispatch_sensor_event(scripting, event.collider1(), event.collider2(), event.started());
		}

		let sensor_stays = self
			.world
			.active_sensor_pairs
			.iter()
			.copied()
			.filter(|pair| previous_sensor_pairs.contains(pair))
			.collect::<Vec<_>>();

		for (collider1, collider2) in sensor_stays {
			self.dispatch_sensor_stay(scripting, collider1, collider2);
		}
	}

	fn dispatch_contact_event(
		&mut self,
		scripting: &ScriptingRuntimeHandle,
		collider1: ColliderHandle,
		collider2: ColliderHandle,
		started: bool,
	) {
		let Some((entity1, entity2)) = self.entities_for_pair(collider1, collider2) else {
			return;
		};

		let pair = collider_pair_key(collider1, collider2);

		if started {
			self.world.active_contact_pairs.insert(pair);
			scripting.on_contact_enter(entity1, entity2);
			scripting.on_contact_enter(entity2, entity1);
		} else {
			self.world.active_contact_pairs.remove(&pair);
			scripting.on_contact_exit(entity1, entity2);
			scripting.on_contact_exit(entity2, entity1);
		}
	}

	fn dispatch_contact_stay(
		&self,
		scripting: &ScriptingRuntimeHandle,
		collider1: ColliderHandle,
		collider2: ColliderHandle,
	) {
		let Some((entity1, entity2)) = self.entities_for_pair(collider1, collider2) else {
			return;
		};

		scripting.on_contact_stay(entity1, entity2);
		scripting.on_contact_stay(entity2, entity1);
	}

	fn dispatch_sensor_event(
		&mut self,
		scripting: &ScriptingRuntimeHandle,
		collider1: ColliderHandle,
		collider2: ColliderHandle,
		started: bool,
	) {
		let pair = collider_pair_key(collider1, collider2);

		if started {
			self.world.active_sensor_pairs.insert(pair);
			self.dispatch_sensor_enter(scripting, collider1, collider2);
		} else {
			self.world.active_sensor_pairs.remove(&pair);
			self.dispatch_sensor_exit(scripting, collider1, collider2);
		}
	}

	fn dispatch_sensor_stay(
		&self,
		scripting: &ScriptingRuntimeHandle,
		collider1: ColliderHandle,
		collider2: ColliderHandle,
	) {
		self.dispatch_sensor_callbacks(
			scripting,
			collider1,
			collider2,
			ScriptingRuntimeHandle::on_contact_stay,
			ScriptingRuntimeHandle::on_sensor_stay,
		);
	}

	fn dispatch_sensor_enter(
		&self,
		scripting: &ScriptingRuntimeHandle,
		collider1: ColliderHandle,
		collider2: ColliderHandle,
	) {
		self.dispatch_sensor_callbacks(
			scripting,
			collider1,
			collider2,
			ScriptingRuntimeHandle::on_contact_enter,
			ScriptingRuntimeHandle::on_sensor_enter,
		);
	}

	fn dispatch_sensor_exit(
		&self,
		scripting: &ScriptingRuntimeHandle,
		collider1: ColliderHandle,
		collider2: ColliderHandle,
	) {
		self.dispatch_sensor_callbacks(
			scripting,
			collider1,
			collider2,
			ScriptingRuntimeHandle::on_contact_exit,
			ScriptingRuntimeHandle::on_sensor_exit,
		);
	}

	fn dispatch_sensor_callbacks(
		&self,
		scripting: &ScriptingRuntimeHandle,
		collider1: ColliderHandle,
		collider2: ColliderHandle,
		contact_callback: fn(&ScriptingRuntimeHandle, EntityId, EntityId),
		sensor_callback: fn(&ScriptingRuntimeHandle, EntityId, EntityId),
	) {
		let Some((entity1, entity2)) = self.entities_for_pair(collider1, collider2) else {
			return;
		};

		let collider1_is_sensor = self.world.collider_is_sensor(collider1);
		let collider2_is_sensor = self.world.collider_is_sensor(collider2);

		match (collider1_is_sensor, collider2_is_sensor) {
			(true, false) => {
				contact_callback(scripting, entity1, entity2);
				sensor_callback(scripting, entity2, entity1);
			}
			(false, true) => {
				contact_callback(scripting, entity2, entity1);
				sensor_callback(scripting, entity1, entity2);
			}
			(true, true) => {
				contact_callback(scripting, entity1, entity2);
				contact_callback(scripting, entity2, entity1);
				sensor_callback(scripting, entity1, entity2);
				sensor_callback(scripting, entity2, entity1);
			}
			(false, false) => {}
		}
	}

	fn entities_for_pair(&self, collider1: ColliderHandle, collider2: ColliderHandle) -> Option<(EntityId, EntityId)> {
		Some((
			self.world.entity_for_collider(collider1)?,
			self.world.entity_for_collider(collider2)?,
		))
	}
}

fn collider_pair_key(collider1: ColliderHandle, collider2: ColliderHandle) -> ColliderPairKey {
	if collider1.into_raw_parts() <= collider2.into_raw_parts() {
		(collider1, collider2)
	} else {
		(collider2, collider1)
	}
}

fn scene_physics_settings(scene: &Scene) -> PhysicsStepSettings {
	let world = scene.world();
	let mut settings = PhysicsStepSettings::default();

	world.run(|entities: EntitiesView| {
		for entity in entities.iter() {
			let Ok(physics_settings) = world.get::<&PhysicsSettings>(entity) else {
				continue;
			};

			settings.gravity = physics_settings.gravity;
			settings.physics_timestep = physics_settings.physics_timestep;
			break;
		}
	});

	settings
}

#[derive(Clone, Copy)]
struct PhysicsStepSettings {
	gravity: Vec2,
	physics_timestep: f32,
}

impl Default for PhysicsStepSettings {
	fn default() -> Self {
		Self {
			gravity: DEFAULT_GRAVITY,
			physics_timestep: DEFAULT_PHYSICS_TIMESTEP,
		}
	}
}

const fn to_rapier_body_type(body_type: RigidBodyType) -> rapier2d::dynamics::RigidBodyType {
	match body_type {
		RigidBodyType::Static => rapier2d::dynamics::RigidBodyType::Fixed,
		RigidBodyType::Dynamic => rapier2d::dynamics::RigidBodyType::Dynamic,
		RigidBodyType::KinematicPositionBased => rapier2d::dynamics::RigidBodyType::KinematicPositionBased,
		RigidBodyType::KinematicVelocityBased => rapier2d::dynamics::RigidBodyType::KinematicVelocityBased,
	}
}

const fn is_kinematic(body_type: RigidBodyType) -> bool {
	matches!(
		body_type,
		RigidBodyType::KinematicPositionBased | RigidBodyType::KinematicVelocityBased
	)
}

fn to_locked_axes(rigid_body: &RigidBody) -> LockedAxes {
	let mut axes = LockedAxes::empty();

	axes.set(LockedAxes::TRANSLATION_LOCKED_X, rigid_body.freeze_position_x);
	axes.set(LockedAxes::TRANSLATION_LOCKED_Y, rigid_body.freeze_position_y);
	axes.set(LockedAxes::ROTATION_LOCKED_X, rigid_body.freeze_rotation_x);
	axes.set(LockedAxes::ROTATION_LOCKED_Y, rigid_body.freeze_rotation_y);
	axes.set(LockedAxes::ROTATION_LOCKED_Z, rigid_body.freeze_rotation_z);

	axes
}

fn build_collider(collider: &Collider, mass: Option<f32>, scale: Vec2) -> rapier2d::geometry::Collider {
	let scale = scale.abs().max(Vec2::splat(f32::EPSILON));
	let builder = match collider.shape {
		ColliderShape::Box { half_extents } => {
			let half_extents = half_extents * scale;
			ColliderBuilder::cuboid(half_extents.x, half_extents.y)
		}
		ColliderShape::Circle { radius } => ColliderBuilder::ball(radius * scale.max_element()),
		ColliderShape::Capsule { half_height, radius } => {
			ColliderBuilder::capsule_y(half_height * scale.y, radius * scale.max_element())
		}
		ColliderShape::ConvexPolygon { ref points } => {
			let scaled_points = points
				.iter()
				.map(|point| point![point.x * scale.x, point.y * scale.y])
				.collect::<Vec<_>>();

			ColliderBuilder::convex_hull(&scaled_points).unwrap_or_else(|| ColliderBuilder::cuboid(0.5, 0.5))
		}
	};

	let builder = builder
		.restitution(collider.restitution)
		.friction(collider.friction)
		.sensor(collider.is_sensor)
		.active_events(ActiveEvents::COLLISION_EVENTS);

	let builder = if let Some(mass) = mass {
		builder.mass(mass)
	} else {
		builder.density(collider.density)
	};

	builder.build()
}
