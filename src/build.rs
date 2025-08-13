use crate::nsfile;
use crate::nsfolder;

pub fn cmd() -> clap::Command {
    clap::Command::new("build")
        .about("Build the current project")
        .arg(clap::arg!(-v --verbose ... "Use verbose output"))
        .arg(clap::arg!(-q --quiet "Do not print log messages").action(clap::ArgAction::SetTrue))
        .arg(clap::arg!(-r --release "Build release version").action(clap::ArgAction::SetTrue))
}

pub fn run_subcmd(matches: &clap::ArgMatches) {
    let _project_cache = nsfolder::check_dot_ns_and_init();
    let ver = crate::verbosity::Verbosity::new(matches);

    // from cache, find out if it is necessary to re-create CMakeLists.txt files from project.ns

    let Some(res) = nsfile::parse_file("project.ns") else {
        panic!("Ill formed project.ns file")
    };

    ver.msg(2, &format!("{res:#?}"));

    let project = res
        .find_str("PROJECT")
        .expect("A project name should be given (project.ns:PROJECT).");
    let exe = res.find_str("EXE").unwrap_or(project);
    let nsengine_bloc = res
        .find_data_bloc("NSENGINE")
        .expect("A nsengine definition should be present (project.ns:NSENGINE).");
    let nsengine_path = nsengine_bloc
        .find_str("PATH")
        .expect("Ns path is required (project.ns:NSENGINE:PATH).");
    let nsengine_mods = nsengine_bloc.get_array("MODULES");
    ver.msg(2, &format!("Loading modules: {nsengine_mods:?}"));

    let libs = res.get_array("LIBS");

    let cmakelists_contents = nsfolder::get_cmake_lists_content(
        "Debug",
        project,
        exe,
        nsengine_path,
        nsengine_mods,
        libs,
    );

    ver.set_current_dir("./.ns");
    ver.create_file_with_data("CMakeLists.txt", &cmakelists_contents);
    ver.fs_copy("CMakeLists.txt", "../CMakeLists.txt");

    let _output = std::process::Command::new("cmake")
        .arg("-S")
        .arg("..")
        .arg("-B")
        .arg("./build")
        .arg("-GNinja")
        .stdout(std::io::stdout())
        .output()
        .expect("Error running command: 'cmake -S .. -B ./build -GNinja'.");

    ver.set_current_dir("./build");

    let _output = std::process::Command::new("ninja")
        .stdout(std::io::stdout())
        .output()
        .expect("Error running command: 'ninja'.");

    ver.remove_file("../../CMakeLists.txt");
}
