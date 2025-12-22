use std::process::Command;

pub fn add_structs() -> String {
    Command::new("docker-compose")
        .args(["up", "-f", "temp-postgres.yaml"])
        .spawn()
        .expect("could not start postgres for creating structs")
        .wait();
    "made output structs".to_string()
}

// docker run --name my-postgres-container -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432
//https://crates.io/crates/sql-gen
