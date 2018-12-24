use cargo_toml2::*;

fn manifest_test() {
    let toml: CargoToml = from_path("Cargo.tomla").expect("Failed to deserialize Cargo.toml");
    println!("{:#?}", toml);
    std::fs::write(
        "Test.toml",
        toml::to_string(&toml).expect("Failed to serialize CargoToml"),
    )
    .expect("Failed to write CargoToml");
}

fn config_test() {
    let toml: CargoConfig = from_path(".cargo/config").expect("Failed to deserialize CargoConfig");
    println!("{:#?}", toml);
    std::fs::write(
        "Test.toml",
        toml::to_string(&toml).expect("Failed to serialize CargoConfig"),
    )
    .expect("Failed to write CargoConfig");
}

fn main() {
    manifest_test();
    config_test();
}
