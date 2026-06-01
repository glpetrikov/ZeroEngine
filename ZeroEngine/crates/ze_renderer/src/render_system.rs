use ze_core::{AssetRef, Mat4, ResourceManager, Result, Vec2, Vec3};
use ze_ecs::{
	Collider, ColliderShape, EntitiesView, EntityId, Inactive, PhysicsSettings, RigidBody, RigidBodyType, Scene,
	System, Transform,
};
use ze_input::{Input, ZKeyCode};

use crate::{
	Renderer,
	components::{Camera, CameraProjection, Sprite, SpriteColorSettings, SpriteSize},
};

#[derive(Debug, Clone)]
pub struct CameraRenderData {
	pub view_projection: Mat4,
	pub clear_color: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct SpriteRenderItem {
	pub transform: Mat4,
	pub texture: AssetRef,
	pub size: SpriteSize,
	pub color: SpriteColorSettings,
	pub layer: i32,
	pub texture_rotation_degrees: f32,
}

#[derive(Debug, Clone)]
pub struct DebugLine {
	pub start: Vec3,
	pub end: Vec3,
	pub color: [f32; 4],
}

#[derive(Default)]
pub struct RenderSystem {
	items: Vec<SpriteRenderItem>,
	debug_lines: Vec<DebugLine>,
}

impl RenderSystem {
	pub fn new() -> Self { Self::default() }

	pub fn render(&mut self, scene: &Scene, renderer: &mut Renderer, resources: &ResourceManager) {
		self.render_scene(scene, renderer, resources);
	}

	fn render_scene(&mut self, scene: &Scene, renderer: &mut Renderer, resources: &ResourceManager) {
		let Some(camera) = Self::find_primary_camera(scene, renderer.aspect_ratio()) else {
			ze_log::warn!("No primary camera found in scene `{}`", scene.name);
			return;
		};

		self.items = Self::collect_items(scene);
		self.debug_lines = Self::collect_debug_lines(scene);
		renderer.request_sprite_redraw(&self.items, &self.debug_lines, &camera, resources);
	}

	fn find_primary_camera(scene: &Scene, aspect: f32) -> Option<CameraRenderData> {
		let world = scene.world();
		let mut camera_data = None;

		world.run(|entities: EntitiesView| {
			for entity in entities.iter() {
				if world.get::<&Inactive>(entity).is_ok() {
					continue;
				}

				let Ok((transform, camera)) = world.get::<(&Transform, &Camera)>(entity) else {
					continue;
				};

				if !camera.primary {
					continue;
				}

				camera_data = Some(Self::build_camera_data(&transform, &camera, aspect));
				break;
			}
		});

		camera_data
	}

	fn build_camera_data(transform: &Transform, camera: &Camera, aspect: f32) -> CameraRenderData {
		let camera_transform = Mat4::from_scale_rotation_translation(Vec3::ONE, transform.rotation, transform.position);
		let view = camera_transform.inverse();

		let projection = match camera.projection {
			CameraProjection::Orthographic { size, near, far } => {
				let half_height = size * 0.5;
				let half_width = half_height * aspect;
				Mat4::orthographic_rh(-half_width, half_width, -half_height, half_height, near, far)
			}
			CameraProjection::Perspective {
				fov_y_radians,
				near,
				far,
			} => Mat4::perspective_rh(fov_y_radians, aspect, near, far),
		};

		CameraRenderData {
			view_projection: projection * view,
			clear_color: camera.clear_color,
		}
	}

	fn collect_items(scene: &Scene) -> Vec<SpriteRenderItem> {
		let mut items = Vec::new();
		let world = scene.world();

		world.run(|entities: EntitiesView| {
			for entity in entities.iter() {
				if world.get::<&Inactive>(entity).is_ok() {
					continue;
				}

				let Ok((transform, sprite)) = world.get::<(&Transform, &Sprite)>(entity) else {
					continue;
				};

				if !sprite.settings.visible {
					continue;
				}

				let mut scale = transform.scale;
				if sprite.settings.flip_x {
					scale.x = -scale.x;
				}
				if sprite.settings.flip_y {
					scale.y = -scale.y;
				}

				items.push(SpriteRenderItem {
					transform: Mat4::from_scale_rotation_translation(scale, transform.rotation, transform.position),
					texture: sprite.texture.clone(),
					size: sprite.size.clone(),
					color: sprite.color.clone(),
					layer: sprite.settings.layer,
					texture_rotation_degrees: sprite.settings.texture_rotation_degrees,
				});
			}
		});

		items.sort_by_key(|item| item.layer);
		items
	}

	fn collect_debug_lines(scene: &Scene) -> Vec<DebugLine> {
		if !Self::debug_draw_enabled(scene) {
			return Vec::new();
		}

		let mut lines = Vec::new();
		let world = scene.world();

		world.run(|entities: EntitiesView| {
			for entity in entities.iter() {
				if world.get::<&Inactive>(entity).is_ok() {
					continue;
				}

				let Ok((transform, collider)) = world.get::<(&Transform, &Collider)>(entity) else {
					continue;
				};

				let rigid_body = world.get::<&RigidBody>(entity).ok();
				let color = debug_color(&collider, rigid_body.as_deref().copied());
				append_collider_lines(&mut lines, &transform, &collider, color);
			}
		});

		lines
	}

	fn debug_draw_enabled(scene: &Scene) -> bool {
		let world = scene.world();
		let mut enabled = false;

		world.run(|entities: EntitiesView| {
			for entity in entities.iter() {
				let Ok(settings) = world.get::<&PhysicsSettings>(entity) else {
					continue;
				};

				enabled = settings.enable_debug_draw;
				break;
			}
		});

		enabled
	}

	fn toggle_debug_draw(scene: &mut Scene) -> Result<()> {
		if let Some(entity) = settings_entity(scene) {
			let mut settings = scene.world_mut().get::<&mut PhysicsSettings>(entity)?;
			settings.enable_debug_draw = !settings.enable_debug_draw;
			return Ok(());
		}

		let entity = scene.create_entity("PhysicsSettings");
		scene.entity_mut(entity).add_component(PhysicsSettings {
			enable_debug_draw: true,
			..PhysicsSettings::default()
		});
		Ok(())
	}
}

impl System for RenderSystem {
	fn name(&self) -> &'static str { "RenderSystem" }

	fn update(&mut self, scene: &mut Scene, _dt: f32) -> Result<()> {
		if Input::is_key_just_pressed(ZKeyCode::KF1) {
			Self::toggle_debug_draw(scene)?;
		}

		Ok(())
	}

	fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

fn settings_entity(scene: &Scene) -> Option<EntityId> {
	let world = scene.world();
	let mut matching_entity = None;

	world.run(|entities: EntitiesView| {
		for entity in entities.iter() {
			if world.get::<&PhysicsSettings>(entity).is_ok() {
				matching_entity = Some(entity);
				break;
			}
		}
	});

	matching_entity
}

fn debug_color(collider: &Collider, rigid_body: Option<&RigidBody>) -> [f32; 4] {
	if collider.is_sensor {
		return [1.0, 0.0, 0.0, 1.0];
	}

	match rigid_body.map(|body| body.body_type) {
		Some(RigidBodyType::Static) => [0.1, 0.35, 1.0, 1.0],
		_ => [0.0, 1.0, 0.0, 1.0],
	}
}

fn append_collider_lines(lines: &mut Vec<DebugLine>, transform: &Transform, collider: &Collider, color: [f32; 4]) {
	match &collider.shape {
		ColliderShape::Box { half_extents } => {
			let points = [
				Vec2::new(-half_extents.x, -half_extents.y),
				Vec2::new(half_extents.x, -half_extents.y),
				Vec2::new(half_extents.x, half_extents.y),
				Vec2::new(-half_extents.x, half_extents.y),
			];
			append_closed_polyline(lines, transform, &points, color);
		}
		ColliderShape::Circle { radius } => {
			let radius = radius * transform.scale.truncate().abs().max_element();
			let points = circle_points(radius, 32);
			append_world_space_closed_polyline(lines, transform, &points, color);
		}
		ColliderShape::Capsule { half_height, radius } => {
			let scale = transform.scale.truncate().abs();
			let half_height = half_height * scale.y;
			let radius = radius * scale.max_element();
			let points = capsule_points(half_height, radius, 12);
			append_world_space_closed_polyline(lines, transform, &points, color);
		}
		ColliderShape::ConvexPolygon { points } => {
			append_closed_polyline(lines, transform, points, color);
		}
	}
}

fn append_closed_polyline(lines: &mut Vec<DebugLine>, transform: &Transform, points: &[Vec2], color: [f32; 4]) {
	if points.len() < 2 {
		return;
	}

	for i in 0..points.len() {
		lines.push(DebugLine {
			start: transform_local_point(transform, points[i]),
			end: transform_local_point(transform, points[(i + 1) % points.len()]),
			color,
		});
	}
}

fn append_world_space_closed_polyline(
	lines: &mut Vec<DebugLine>,
	transform: &Transform,
	points: &[Vec2],
	color: [f32; 4],
) {
	if points.len() < 2 {
		return;
	}

	for i in 0..points.len() {
		lines.push(DebugLine {
			start: transform_unscaled_point(transform, points[i]),
			end: transform_unscaled_point(transform, points[(i + 1) % points.len()]),
			color,
		});
	}
}

fn transform_local_point(transform: &Transform, point: Vec2) -> Vec3 {
	let scaled = Vec3::new(point.x * transform.scale.x, point.y * transform.scale.y, 0.0);
	transform.position + transform.rotation * scaled
}

fn transform_unscaled_point(transform: &Transform, point: Vec2) -> Vec3 {
	transform.position + transform.rotation * Vec3::new(point.x, point.y, 0.0)
}

fn circle_points(radius: f32, segments: usize) -> Vec<Vec2> {
	(0..segments)
		.map(|i| {
			let angle = i as f32 / segments as f32 * std::f32::consts::TAU;
			Vec2::new(angle.cos() * radius, angle.sin() * radius)
		})
		.collect()
}

fn capsule_points(half_height: f32, radius: f32, arc_segments: usize) -> Vec<Vec2> {
	let mut points = Vec::with_capacity((arc_segments + 1) * 2);

	for i in 0..=arc_segments {
		let angle = i as f32 / arc_segments as f32 * std::f32::consts::PI;
		points.push(Vec2::new(
			angle.cos() * radius,
			angle.sin().mul_add(radius, half_height),
		));
	}

	for i in 0..=arc_segments {
		let angle =
			std::f32::consts::PI + (i as f32 / arc_segments as f32).mul_add(std::f32::consts::PI, std::f32::consts::PI);
		points.push(Vec2::new(
			angle.cos() * radius,
			-angle.sin().mul_add(radius, half_height),
		));
	}

	points
}
