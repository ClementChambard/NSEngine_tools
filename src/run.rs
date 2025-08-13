use crate::nsfile;

pub fn cmd() -> clap::Command {
    clap::Command::new("run")
        .about("Run the current project")
        .arg(clap::arg!(-v --verbose ... "Use verbose output"))
        .arg(clap::arg!(-q --quiet "Do not print log messages").action(clap::ArgAction::SetTrue))
        .arg(clap::arg!(-r --release "Build release version").action(clap::ArgAction::SetTrue))
}

pub fn run_subcmd(matches: &clap::ArgMatches) {
    let Some(res) = nsfile::parse_file("project.ns") else {
        panic!("ill formed project.ns file")
    };

    let ver = crate::verbosity::Verbosity::new(matches);

    let project = res
        .find_str("PROJECT")
        .expect("A project name should be given.");
    let exe = format!("./{}", res.find_str("EXE").unwrap_or(project));

    ver.set_current_dir("./.ns/build");

    let _output = std::process::Command::new(exe)
        .stdout(std::io::stdout())
        .output()
        .expect("Failed to run executable.");
}
