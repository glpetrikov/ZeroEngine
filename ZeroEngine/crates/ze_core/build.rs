fn main() {
	println!("cargo:rerun-if-changed=../../../assets");
	wesl::Wesl::new("../../../assets/shaders/engine")
		.build_artifact(&"package::sprite".parse().unwrap(), "engine_sprite");
}
