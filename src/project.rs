use std::error::Error;
use std::fs;
use std::path::Path;
use crate::GITHUB_REPO;
use crate::utils::download_file;

pub async fn create_cpp_project(project_name: &str, build_system: &str) -> Result<(), Box<dyn Error>> {
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