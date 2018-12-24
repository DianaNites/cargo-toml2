use cargo_toml2::*;

fn main() {
    let toml = std::fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let toml: CargoToml = toml::from_str(&toml).expect("Failed to deserialize Cargo.toml");
    println!("{:#?}", toml);
    std::fs::write(
        "Test.toml",
        toml::to_string(&toml).expect("Failed to serialize CargoToml"),
    )
    .expect("Failed to write CargoToml");
}
