fn main() {
	println!("cargo:rerun-if-changed=../../../assets");
	wesl::Wesl::new("../../../assets/shaders/engine").build_artifact(
		&"package::sprite"
			.parse()
			.expect("Failed to parse hardcoded engine sprite package name"),
		"engine_sprite",
	);
}
