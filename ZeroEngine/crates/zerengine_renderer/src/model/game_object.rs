use glam::Vec3;

pub struct Object {
	pub position: Vec3,
	pub angle: f32,
	pub scale: Vec3,
}
#[derive(Default)]
pub struct Camera {
	pub position: Vec3,
	pub forwards: Vec3,
	pub right: Vec3,
	pub up: Vec3,
	yaw: f32,
	pitch: f32,
}

impl Camera {
	pub fn new() -> Self {
		let mut camera = Self {
			position: Vec3::new(-5.0, 0.0, 0.5),
			yaw: 0.0,
			pitch: 0.0,
			forwards: Vec3::ZERO,
			right: Vec3::ZERO,
			up: Vec3::ZERO,
		};

		camera.update_vectors();
		camera
	}

	pub fn spin(&mut self, d_yaw: f32, d_pitch: f32) {
		self.yaw = (self.yaw + d_yaw).rem_euclid(360.0);
		self.pitch = (self.pitch + d_pitch).clamp(-89.0, 89.0);

		self.update_vectors();
	}

	fn update_vectors(&mut self) {
		let yaw = self.yaw.to_radians();
		let pitch = self.pitch.to_radians();

		let c_yaw = yaw.cos();
		let s_yaw = yaw.sin();
		let c_pitch = pitch.cos();
		let s_pitch = pitch.sin();

		self.forwards = Vec3::new(c_yaw * c_pitch, s_yaw * c_pitch, s_pitch).normalize();

		let world_up = Vec3::Z;

		self.right = self.forwards.cross(world_up).normalize();
		self.up = self.right.cross(self.forwards).normalize();
	}

	pub fn move_relative(&mut self, d_right: f32, d_forwards: f32) {
		self.position += self.right * d_right;
		self.position += self.forwards * d_forwards;
	}
}
