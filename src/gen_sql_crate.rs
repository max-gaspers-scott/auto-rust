use std::{
    env::{current_dir, current_exe},
    process::Command,
};

pub fn gen_sql_crate() -> Result<(), std::io::Error> {
    let sql_path = current_dir().unwrap().join("migrations/0001_data.sql");
    // Clean up any existing container with the same name
    let _ = Command::new("docker")
        .args(["rm", "-f", "sql_gen_con"])
        .status();

    let postgres_res = Command::new("docker")
        .args([
            "run",
            "--name",
            "sql_gen_con",
            "-e",
            "POSTGRES_PASSWORD=secret",
            "-p",
            "12345:5432",
            "-d",
            "postgres",
        ])
        .status();

    match postgres_res {
        Ok(res) => {
            println!("postgres result: {}", res);
        }
        Err(e) => {
            println!("postgres result: {}", e);
        }
    }

    // Wait for PostgreSQL to be ready
    println!("Waiting for PostgreSQL to start...");
    let mut attempts = 0;
    let max_attempts = 30;
    loop {
        let check_res = Command::new("docker")
            .args(["exec", "sql_gen_con", "pg_isready", "-U", "postgres"])
            .status();

        match check_res {
            Ok(status) if status.success() => {
                println!("PostgreSQL is ready!");

                // Create the database
                let create_db_res = Command::new("docker")
                    .args([
                        "exec",
                        "sql_gen_con",
                        "psql",
                        "-U",
                        "postgres",
                        "-c",
                        "CREATE DATABASE sql_gen_con;",
                    ])
                    .status();

                match create_db_res {
                    Ok(_) => println!("Database created successfully!"),
                    Err(e) => println!("Note: Database might already exist: {}", e),
                }

                break;
            }
            _ => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "PostgreSQL failed to start within timeout",
                    ));
                }
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
    }
    Command::new("docker")
        .args([
            "cp",
            sql_path.to_str().expect("failed to get sql path"),
            "sql_gen_con:/",
        ])
        .status()?;

    Command::new("docker")
        .args([
            "exec",
            "sql_gen_con",
            "psql",
            "-U",
            "postgres",
            "-d",
            "postgres",
            "-f",
            &format!("/{}", sql_path.file_name().unwrap().to_str().unwrap()),
        ])
        .status()?;
    // TODO: this assumes that sql-gen is installed witch is bad. Find a fix
    // TODO: breaks if port already alocated for the docker containers that need to be set up

    println!("Waiting for database to be fully ready...");
    let mut db_attempts = 0;
    let max_db_attempts = 10;

    loop {
        let test_res = Command::new("psql")
            .args([
                "postgres://postgres:secret@localhost:12345/postgres",
                "-c",
                "SELECT 1;",
            ])
            .status();

        match test_res {
            Ok(status) if status.success() => {
                println!("Database connection verified!");
                break;
            }
            _ => {
                db_attempts += 1;
                if db_attempts >= max_db_attempts {
                    eprintln!("Warning: Could not verify database connection, proceeding anyway");
                    break;
                }
                println!("Database not ready, waiting 2 seconds...");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
    }

    let gen_sql_status = Command::new("sql-gen")
        .args([
            "--db-url",
            "postgres://postgres:secret@localhost:12345/postgres",
            "--output",
            "backend/src/models/",
        ])
        .status()?;
    Ok(())
}
