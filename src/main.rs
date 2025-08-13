mod add;
mod build;
mod new;
mod nsfile;
mod nsfolder;
mod run;
mod verbosity;

fn cmd() -> clap::Command {
    clap::command!()
        .subcommand_required(true)
        .subcommand(new::cmd())
        .subcommand(build::cmd())
        .subcommand(run::cmd())
        .subcommand(add::cmd())
}

fn main() {
    let matches = cmd().get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        new::run_subcmd(matches);
    }

    if let Some(matches) = matches.subcommand_matches("build") {
        build::run_subcmd(matches);
    }

    if let Some(matches) = matches.subcommand_matches("run") {
        run::run_subcmd(matches);
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        add::run_subcmd(matches);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn verify_cmd() {
        super::cmd().debug_assert();
    }
}
