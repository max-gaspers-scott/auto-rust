use crate::add_fastapi;
use crate::create_react_app;
use crate::gen_examples;
use crate::gen_toml;
use std::net::{SocketAddr, TcpListener};
// import func and match if it fail
use crate::add_axum_end;
use crate::add_compose;
use crate::add_minio;
use crate::add_top_boilerplate;
use crate::gen_docker;
use std::path::Path;
use std::process::Command;

pub fn setup(parent_dir: &Path, file_name: &str) -> Result<(), std::io::Error> {
    let project_dir = parent_dir.join(&file_name);

    let path = project_dir.join("src/main.rs");
    println!("parent in setup {} ", parent_dir.display());
    println!("prooject dir in stup: {} ", project_dir.display());

    let output = Command::new("cargo")
        .current_dir(&parent_dir)
        .arg("new")
        .arg(file_name)
        .output()?;

    if !output.status.success() {
        eprintln!(
            "Failed to create new project: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to create new project: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    //this is likly the os problem

    let gen_toml_res = gen_toml::gen_toml(&parent_dir.join(file_name));
    match gen_toml_res {
        Ok(_) => println!("Successfully generated TOML"),
        Err(e) => eprintln!("Failed to generate TOML: {}", e),
    };

    let top_res = add_top_boilerplate(&path);

    // let end_res = add_axum_end(func_names.clone(), &path);
    // TODO: this looks like a dublicat of the add_minio function
    // add_object(&path);
    let docker_res = gen_docker(
        project_dir
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .unwrap(),
    );
    match docker_res {
        Ok(_) => println!(
            "Dockerfile created at {}",
            project_dir.to_str().unwrap().to_owned()
        ),
        Err(e) => eprintln!("Error creating Dockerfile: {}", e),
    }
    let compose = add_compose(
        project_dir
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .unwrap(),
    );
    match compose {
        Ok(_) => println!(
            "Docker compose created at {}",
            project_dir.to_str().unwrap().to_owned()
        ),
        Err(e) => eprintln!("Error creating Docker compose: {}", e),
    }

    let minio = add_minio(&project_dir.join("src/main.rs"));
    match minio {
        Ok(_) => println!(
            "Minio added at {}",
            project_dir.to_str().unwrap().to_owned()
        ),
        Err(e) => eprintln!("Error adding Minio: {}", e),
    }

    let _ = create_react_app(
        "../".to_owned()
            + project_dir
                .file_name()
                .expect("Failed to get file name")
                .to_str()
                .unwrap(),
    );

    // let gen_examples_res = gen_examples(
    //     &project_dir
    //         .file_name()
    //         .expect("Failed to get file name")
    //         .to_str()
    //         .unwrap(),
    //     func_names.clone(),
    // );
    // match gen_examples_res {
    //     Ok(_) => println!(
    //         "Examples generated at {}",
    //         project_dir.to_str().unwrap().to_owned()
    //     ),
    //     Err(e) => eprintln!("Error generating examples: {}", e),
    // }
    //
    let end_res = add_axum_end(Vec::new(), &project_dir.join("src/main.rs"));
    match end_res {
        Ok(_) => println!("end added"),
        Err(e) => println!("error adding end: {}", e),
    }

    let port_num = 8081;

    let fastapi_res = add_fastapi(
        &project_dir
            .file_name()
            .expect("faild to get the file name for fast api func")
            .to_str()
            .unwrap(),
    );

    match fastapi_res {
        Ok(_) => println!("added the fastapi folder "),
        Err(e) => eprintln!("error while adding the fastapi folder: {}", e),
    }

    let addr: SocketAddr = "0.0.0.0:8081".parse().unwrap();
    match TcpListener::bind(&addr) {
        // If the bind operation is successful, it means the port was available.
        Ok(listener) => {
            println!("✅ Port 8081 is NOT in use.");
            // It's important to explicitly drop the listener to free up the port immediately.
            // This allows the program to exit cleanly.
            drop(listener);
        }
        // If the bind operation fails, an error is returned.
        // We can inspect the error kind to determine if the port is already in use.
        Err(e) => {
            // A common error is `AddrInUse`, which indicates the port is already taken.
            if e.kind() == std::io::ErrorKind::AddrInUse {
                println!("❌ Port {port_num} is already in uses!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            } else {
                // Handle other potential errors, such as permissions issues.
                eprintln!("An unexpected error occurred: {}", e);
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            }
        }
    }

    Ok(())
}
