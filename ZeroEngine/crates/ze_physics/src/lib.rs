use std::collections::HashMap;

use rapier2d::prelude::*;
use ze_core::{Quat, Result, Vec2};
use ze_ecs::{
	Collider, ColliderShape, CollisionDetection, EntitiesView, EntityId, PhysicsSettings, RigidBody, RigidBodyType,
	Scene, System, Transform,
};

pub const DEFAULT_GRAVITY: Vec2 = Vec2::new(0.0, -9.81);

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
}

#[derive(Debug, Clone, Copy)]
struct PhysicsBodyEntry {
	handle: RigidBodyHandle,
	body_type: RigidBodyType,
}

impl PhysicsWorld {
	pub fn new() -> Self {
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
		}
	}

	pub fn with_gravity(gravity: Vec2) -> Self {
		let mut world = Self::new();
		world.set_gravity(gravity);
		world
	}

	pub fn gravity(&self) -> Vec2 { Vec2::new(self.gravity.x, self.gravity.y) }

	pub fn set_gravity(&mut self, gravity: Vec2) { self.gravity = vector![gravity.x, gravity.y]; }

	pub fn step(&mut self, dt: f32) {
		self.integration_parameters.dt = dt.max(0.0);

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
			&(),
		);
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
}

impl PhysicsSystem {
	pub fn new() -> Self {
		Self {
			world: PhysicsWorld::new(),
			initialized: false,
		}
	}

	pub fn world(&self) -> &PhysicsWorld { &self.world }

	pub fn world_mut(&mut self) -> &mut PhysicsWorld { &mut self.world }

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

		self.world.set_gravity(scene_gravity(scene));
		self.world.sync_from_ecs(scene);
		self.world.step(dt);
		self.world.sync_to_ecs(scene)
	}

	fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

fn scene_gravity(scene: &Scene) -> Vec2 {
	let world = scene.world();
	let mut gravity = DEFAULT_GRAVITY;

	world.run(|entities: EntitiesView| {
		for entity in entities.iter() {
			let Ok(settings) = world.get::<&PhysicsSettings>(entity) else {
				continue;
			};

			gravity = settings.gravity;
			break;
		}
	});

	gravity
}

fn to_rapier_body_type(body_type: RigidBodyType) -> rapier2d::dynamics::RigidBodyType {
	match body_type {
		RigidBodyType::Static => rapier2d::dynamics::RigidBodyType::Fixed,
		RigidBodyType::Dynamic => rapier2d::dynamics::RigidBodyType::Dynamic,
		RigidBodyType::KinematicPositionBased => rapier2d::dynamics::RigidBodyType::KinematicPositionBased,
		RigidBodyType::KinematicVelocityBased => rapier2d::dynamics::RigidBodyType::KinematicVelocityBased,
	}
}

fn is_kinematic(body_type: RigidBodyType) -> bool {
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
		.sensor(collider.is_sensor);

	let builder = if let Some(mass) = mass {
		builder.mass(mass)
	} else {
		builder.density(collider.density)
	};

	builder.build()
}
