pub fn cmd() -> clap::Command {
    clap::Command::new("add")
        .about("Add a module or a library to the project")
        .arg(clap::arg!(-v --verbose ... "Use verbose output"))
        .arg(clap::arg!(-q --quiet "Do not print log messages").action(clap::ArgAction::SetTrue))
        .arg(clap::arg!(--lib "Add libraries instead of modules").action(clap::ArgAction::SetTrue))
        .arg(clap::Arg::new("item").action(clap::ArgAction::Append))
}

pub fn run_subcmd(matches: &clap::ArgMatches) {
    let _is_lib = matches.get_flag("lib");
    let _items = matches.get_many::<String>("item").unwrap_or_default();
}
