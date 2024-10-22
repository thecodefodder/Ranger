use std::error::Error;
use std::{env, fs};
use std::path::Path;
use dialoguer::Select;

const GITHUB_REPO: &str = "https://raw.githubusercontent.com/thecodefodder/Ranger/main/templates/";

async fn download_file(url: &str, dest: &Path) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    fs::write(dest, content).expect("Failed to write to disk.");

    Ok(())
}

async fn create_cpp_project(project_name: &str, build_system: &str) -> Result<(), Box<dyn Error>> {
    let project_path = Path::new(project_name);
    fs::create_dir_all(project_path)?;

    let base_url = format!("{}{}", GITHUB_REPO, build_system);

    let template_files = vec![
        (format!("{}/CMakeLists.txt", base_url), project_path.join("CMakeLists.txt")),
        (format!("{}/src/main.cpp", base_url), project_path.join("src/main.cpp")),
        (format!("{}/meson.build", base_url), project_path.join("meson.build")),
    ];

    fs::create_dir_all(project_path.join("src"))?;

    for (url, dest) in template_files {
        download_file(&url, &dest).await?;

        let mut content = fs::read_to_string(&dest)?;

        content = content.replace("${PROJECT_NAME}", project_name);
        content = content.replace("${EXECUTABLE_NAME}", project_name);

        fs::write(dest, content)?;
    }

    Ok(())
}

// TODO: Add support for C
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = vec!["CMake-Cpp", "Make-Cpp", "Meson-Cpp"];

    let selection = Select::new()
        .with_prompt("Please select a build system")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();

    let project_name = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run <project_name>");
        std::process::exit(1);
    });

    create_cpp_project(&project_name, options[selection]).await?;
    println!("C++ project '{}' created successfully with {}.", project_name, options[selection]);

    Ok(())
}