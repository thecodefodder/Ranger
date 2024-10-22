use dialoguer::Select;
use std::error::Error;
use std::path::Path;
use std::{env, fs};
use std::fmt::format;

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
    let mut template_files = vec![(
        format!("{}/src/main.cpp", base_url),
        project_path.join("src/main.cpp"),
    )];

    match build_system {
        "CMake-Cpp" => {
            template_files.push((
                format!("{}/CMakeLists.txt", base_url),
                project_path.join("CMakeLists.txt"),
            ));
        }
        "Make-Cpp" => {
            template_files.push((
                format!("{}/Makefile", base_url),
                project_path.join("Makefile"),
            ));
        }
        "Meson-Cpp" => {
            template_files.push((
                format!("{}/meson.build", base_url),
                project_path.join("meson.build"),
            ));
        }
        "Premake5-Cpp" => {
            template_files.push((
                format!("{}/premake5.lua", base_url),
                project_path.join("premake5.lua"),
            ));
        }
        _ => {}
    }

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
    let project_name = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run <project_name>");
        std::process::exit(1);
    });

    let options = vec!["CMake-Cpp", "Make-Cpp", "Meson-Cpp", "Premake5-Cpp"];

    let selection = Select::new()
        .with_prompt("Please select a build system")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();

    create_cpp_project(&project_name, options[selection]).await?;
    println!(
        "C++ project '{}' created successfully with {}.",
        project_name, options[selection]
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;

    #[tokio::test]
    async fn test_create_cpp_project() {
        let project_name = "TestProject";
        let build_system = "CMake-Cpp";

        create_cpp_project(project_name, build_system)
            .await
            .unwrap();

        let project_path = Path::new(project_name);
        assert!(project_path.exists());
        assert!(project_path.join("CMakeLists.txt").exists());
        assert!(project_path.join("src").exists());
        assert!(project_path.join("src/main.cpp").exists());

        remove_dir_all(project_path).unwrap();
    }

    #[tokio::test]
    async fn test_create_make_cpp_project() {
        let project_name = "TestMakeProject";
        let build_system = "Make-Cpp";

        create_cpp_project(project_name, build_system)
            .await
            .unwrap();

        let project_path = Path::new(project_name);
        assert!(project_path.exists());
        assert!(project_path.join("Makefile").exists());
        assert!(project_path.join("src").exists());
        assert!(project_path.join("src/main.cpp").exists());

        remove_dir_all(project_path).unwrap();
    }

    #[tokio::test]
    async fn test_create_meson_cpp_project() {
        let project_name = "TestMesonProject";
        let build_system = "Meson-Cpp";

        create_cpp_project(project_name, build_system)
            .await
            .unwrap();

        let project_path = Path::new(project_name);
        assert!(project_path.exists());
        assert!(project_path.join("meson.build").exists());
        assert!(project_path.join("src").exists());
        assert!(project_path.join("src/main.cpp").exists());

        remove_dir_all(project_path).unwrap();
    }

    #[tokio::test]
    async fn test_create_premake_cpp_project() {
        let project_name = "TestPremakeProject";
        let build_system = "Premake5-Cpp";

        create_cpp_project(project_name, build_system)
            .await
            .unwrap();

        let project_path = Path::new(project_name);
        assert!(project_path.exists());
        assert!(project_path.join("premake5.lua").exists());
        assert!(project_path.join("src").exists());
        assert!(project_path.join("src/main.cpp").exists());

        remove_dir_all(project_path).unwrap();
    }
}
