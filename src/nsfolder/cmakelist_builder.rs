pub fn get_cmake_lists_content(
    build_type: &str,
    project_name: &str,
    exe_name: &str,
    nsengine_dir: &str,
    modules_str: &[String],
    libs_str: &[String],
) -> String {
    let mut lib_str = String::new();
    for lib in libs_str {
        lib_str.push(' ');
        lib_str.push_str(&get_cmake_lib_str(lib));
    }
    let mut module_str = String::new();
    for module in modules_str {
        module_str.push_str(&get_cmake_module_str(module));
        module_str.push('\n');
    }
    let mut nsengine = String::from(nsengine_dir);
    if !nsengine.ends_with('/') {
        nsengine.push('/');
    }
    get_cmake_lists(
        build_type,
        project_name,
        exe_name,
        &nsengine,
        &module_str,
        &lib_str,
    )
}

fn get_cmake_lib_str(lib: &str) -> String {
    format!("-l{lib}")
}

fn get_cmake_module_str(module: &str) -> String {
    format!("set(NS_USE_{0} ON CACHE BOOL \"enable {0}\" FORCE)", module)
}

fn get_cmake_lists(
    build_type: &str,
    project_name: &str,
    exe_name: &str,
    nsengine_dir: &str,
    modules_str: &str,
    libs_str: &str,
) -> String {
    format!(
        r#"# required
cmake_minimum_required(VERSION 3.20)

# set some flags
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_BUILD_TYPE "{0}")

add_compile_options(-Wall -Wextra -Werror)
add_compile_options(-DNS_IMPORT)

if(NOT CMAKE_BUILD_TYPE STREQUAL "Debug")
  add_compile_options(-O4)
endif()

# set the project name and version
project("{1}" VERSION 0.1)

# add the NSEngine lib
{4}add_subdirectory("{3}" "{3}build")

# get source files
file(GLOB_RECURSE SRCS src/*.cpp src/*.c)
file(GLOB_RECURSE INCLUDES src/*.hpp src/*.h)

# add the executable
add_executable("{2}" ${{SRCS}} ${{INCLUDES}})

# set executable props
set_property(TARGET "{2}" PROPERTY CXX_STANDARD 20)
set_property(TARGET "{2}" PROPERTY CXX_STANDARD_REQUIRED True)

# link the NSEngine library. All the dependencies are added directly to the engine
target_link_libraries("{2}" PUBLIC NSEngine{5})

# TODO do this better (copy assets in build directory)
file(COPY assets DESTINATION ${{PROJECT_BINARY_DIR}})
"#,
        build_type, project_name, exe_name, nsengine_dir, modules_str, libs_str
    )
}
