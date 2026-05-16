fn main() {
    wesl::Wesl::new("assets/shaders")
        .build_artifact(&"package::triangle".parse().unwrap(), "triangle");
}