use cargo_toml2::*;

fn manifest_test() {
    let toml = std::fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let toml: CargoToml = toml::from_str(&toml).expect("Failed to deserialize Cargo.toml");
    println!("{:#?}", toml);
    std::fs::write(
        "Test.toml",
        toml::to_string(&toml).expect("Failed to serialize CargoToml"),
    )
    .expect("Failed to write CargoToml");
}

fn config_test() {
    let toml = std::fs::read_to_string(".cargo/config").expect("Failed to read .cargo/config");
    let toml: CargoConfig = toml::from_str(&toml).expect("Failed to deserialize CargoConfig");
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
