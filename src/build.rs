use crate::nsfile;
use crate::nsfolder;
use std::io::Write;

pub fn cmd() -> clap::Command {
    clap::Command::new("build")
        .about("Build the current project")
        .arg(clap::arg!(-v --verbose ... "Use verbose output"))
        .arg(clap::arg!(-q --quiet "Do not print log messages").action(clap::ArgAction::SetTrue))
        .arg(clap::arg!(-r --release "Build release version").action(clap::ArgAction::SetTrue))
}

pub fn run_subcmd(_matches: &clap::ArgMatches) {
    let _project_cache = nsfolder::check_dot_ns_and_init();

    // from cache, find out if it is necessary to re-create CMakeLists.txt files from project.ns

    let Some(res) = nsfile::parse_file("project.ns") else {
        panic!("ill formed project.ns file")
    };

    let project = res
        .find_str("PROJECT")
        .expect("A project name should be given.");
    let exe = res.find_str("EXE").unwrap_or(project);
    let nsengine_bloc = res
        .find_data_bloc("NSENGINE")
        .expect("A nsengine definition should be present");
    let nsengine_path = nsengine_bloc.find_str("PATH").expect("ns path is required");
    let nsengine_mods = nsengine_bloc.get_array("MODULES");
    let libs = res.get_array("LIBS");

    let cmakelists_contents = nsfolder::get_cmake_lists_content(
        "Debug",
        project,
        exe,
        nsengine_path,
        nsengine_mods,
        libs,
    );

    std::env::set_current_dir("./.ns").unwrap();
    let mut cmakelists = std::fs::File::create("CMakeLists.txt").unwrap();
    write!(cmakelists, "{}", cmakelists_contents).unwrap();

    std::fs::copy("CMakeLists.txt", "../CMakeLists.txt").unwrap();
    let _output = std::process::Command::new("cmake")
        .arg("-S")
        .arg("..")
        .arg("-B")
        .arg("./build")
        .arg("-GNinja")
        .stdout(std::io::stdout())
        .output()
        .unwrap();

    std::env::set_current_dir("./build").unwrap();
    let _output = std::process::Command::new("ninja")
        .stdout(std::io::stdout())
        .output()
        .unwrap();

    std::fs::remove_file("../../CMakeLists.txt").unwrap();
}
