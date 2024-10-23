mod utils;
mod project;

use dialoguer::Select;
use std::env;
use anyhow::Result;
use crate::project::{create_cpp_project, BuildSystem};

const GITHUB_REPO: &str = "https://raw.githubusercontent.com/thecodefodder/Ranger/main/templates/";

// TODO: Add support for C
#[tokio::main]
async fn main() -> Result<()> {
    let project_name = env::args().nth(1).ok_or_else(|| {
        anyhow::anyhow!("Usage: cargo run <project_name>")
    })?;

    let options = vec!["CMake-Cpp", "Make-Cpp", "Meson-Cpp", "Premake5-Cpp"];

    let selection = Select::new()
        .with_prompt("Please select a build system")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();

    let build_system = match selection {
        0 => BuildSystem::CMake,
        1 => BuildSystem::Make,
        2 => BuildSystem::Meson,
        3 => BuildSystem::Premake5,
        _ => panic!("Invalid selection"),
    };

    create_cpp_project(&project_name, build_system).await?;
    println!(
        "C++ project '{}' created successfully with {}.",
        project_name, options[selection]
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::remove_dir_all, path::Path};

    #[tokio::test]
    async fn test_create_cpp_project() {
        let project_name = "TestProject";
        let build_system = BuildSystem::CMake;

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
        let build_system = BuildSystem::Make;

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
        let build_system = BuildSystem::Meson;

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
        let build_system = BuildSystem::Premake5;

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
