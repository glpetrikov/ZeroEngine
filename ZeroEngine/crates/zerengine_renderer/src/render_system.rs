use zerengine_core::{AssetRef, Mat4, ResourceManager, Result, Vec3};
use zerengine_ecs::{EntitiesView, Inactive, Scene, System, Transform};

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
}

#[derive(Default)]
pub struct RenderSystem {
	items: Vec<SpriteRenderItem>,
}

impl RenderSystem {
	pub fn new() -> Self { Self::default() }

	pub fn render(&mut self, scene: &Scene, renderer: &mut Renderer, resources: &ResourceManager) -> Result<()> {
		self.render_scene(scene, renderer, resources)
	}

	fn render_scene(&mut self, scene: &Scene, renderer: &mut Renderer, resources: &ResourceManager) -> Result<()> {
		let Some(camera) = Self::find_primary_camera(scene, renderer.aspect_ratio()) else {
			zerengine_log::warn!("No primary camera found in scene `{}`", scene.name);
			return Ok(());
		};

		self.items = Self::collect_items(scene);
		renderer.request_sprite_redraw(&self.items, &camera, resources);
		Ok(())
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
				});
			}
		});

		items.sort_by_key(|item| item.layer);
		items
	}
}

impl System for RenderSystem {
	fn name(&self) -> &'static str { "RenderSystem" }

	fn update(&mut self, _scene: &mut Scene, _dt: f32) -> Result<()> { Ok(()) }

	fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
