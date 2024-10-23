use core::fmt;
use std::error::Error;
use std::fs;
use std::path::Path;
use crate::GITHUB_REPO;
use crate::utils::download_file;

#[derive(Debug)]
pub enum BuildSystem {
    CMake,
    Make,
    Meson,
    Premake5
}

impl fmt::Display for BuildSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildSystem::CMake => write!(f, "CMake"),
            BuildSystem::Make => write!(f, "Make"),
            BuildSystem::Meson => write!(f, "Meson"),
            BuildSystem::Premake5 => write!(f, "Premake5"),
        }
    }
}

pub async fn create_cpp_project(project_name: &str, build_system: BuildSystem) -> Result<(), Box<dyn Error>> {
    let project_path = Path::new(project_name);
    fs::create_dir_all(project_path)?;

    let base_url = format!("{}{}", GITHUB_REPO, build_system);
    let mut template_files = vec![(
        format!("{}/src/main.cpp", base_url),
        project_path.join("src/main.cpp"),
    )];

    match build_system {
        BuildSystem::CMake => {
            template_files.push((
                format!("{}/CMakeLists.txt", base_url),
                project_path.join("CMakeLists.txt"),
            ));
        }
        BuildSystem::Make => {
            template_files.push((
                format!("{}/Makefile", base_url),
                project_path.join("Makefile"),
            ));
        }
        BuildSystem::Meson => {
            template_files.push((
                format!("{}/meson.build", base_url),
                project_path.join("meson.build"),
            ));
        }
        BuildSystem::Premake5 => {
            template_files.push((
                format!("{}/premake5.lua", base_url),
                project_path.join("premake5.lua"),
            ));
        }
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