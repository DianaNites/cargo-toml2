use cargo_toml2::*;

fn manifest_test() {
    let toml: CargoToml = from_path("Cargo.toml").expect("Failed to deserialize Cargo.toml");
    println!("{:#?}", toml);
    to_path("Test.toml", toml).expect("Failed to serialize/write CargoToml");
}

fn config_test() {
    let toml: CargoConfig = from_path(".cargo/config").expect("Failed to deserialize CargoConfig");
    println!("{:#?}", toml);
    to_path("Test.toml", toml).expect("Failed to serialize/write CargoConfig");
}

fn main() {
    manifest_test();
    config_test();
}
