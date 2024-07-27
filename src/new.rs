use std::io::Write;
use std::path::PathBuf;

pub fn cmd() -> clap::Command {
    clap::Command::new("new")
        .about("Create a new project")
        .arg(clap::arg!(<name> "The name of the new project"))
        .arg(
            clap::arg!(-d --dir <PATH> "The directory to put the project in")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            clap::arg!(--"nsengine-location" <PATH> "Location of NSEngine library")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(clap::arg!(-v --verbose ... "Use verbose output"))
        .arg(clap::arg!(-q --quiet "Do not print log messages").action(clap::ArgAction::SetTrue))
}

pub fn run_subcmd(matches: &clap::ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    let verbosity = crate::verbosity::calculate_verbosity(matches);

    let dir = if let Some(d) = matches.get_one::<PathBuf>("dir") {
        d.clone()
    } else {
        PathBuf::from(name)
    };

    let nsengine_loc = if let Some(d) = matches.get_one::<PathBuf>("nsengine-location") {
        d.clone()
    } else {
        // TODO: try to find NSEngine installation
        println!("TODO! don't hardcode path");
        PathBuf::from("/home/clement/dev/NSEngine")
    };

    let dir = std::path::Path::new(&dir);
    let nsengine_dir = std::path::Path::new(&nsengine_loc);

    if dir.exists() {
        println!(
            "Error: can't create project in existing directory: {}",
            dir.display()
        );
        return;
    }

    let has_nsengine = if !nsengine_dir.exists() {
        println!("Warning: can't find NSEngine.");
        false
    } else {
        if verbosity > 1 {
            println!("using NSEngine from dir: {}", nsengine_dir.display());
        }
        true
    };

    if verbosity > 1 {
        println!("creating project in directory: {}", dir.display());
    }

    std::fs::create_dir(dir).unwrap();
    if verbosity > 2 {
        println!("created directory: {}", dir.display());
    }

    std::env::set_current_dir(dir).unwrap();
    if verbosity > 2 {
        println!("navigating to {}", dir.display());
    }

    std::fs::create_dir("src").unwrap();
    if verbosity > 2 {
        println!("created directory: src");
    }

    // TEMP
    std::process::Command::new("cp")
        .arg("-r")
        .arg(&format!("{}/assets", nsengine_dir.display()))
        .arg(".")
        .output()
        .unwrap();
    // std::fs::create_dir("assets").unwrap();
    // if verbosity > 2 {
    //     println!("created directory: assets");
    // }

    let mut ns_file = std::fs::File::create("project.ns").unwrap();
    write!(
        ns_file,
        "{}",
        crate::nsfile::make_default_project_ns(name, &format!("{}", nsengine_dir.display()))
    )
    .unwrap();

    if verbosity > 2 {
        println!("wrote default project.ns file");
    }

    let mut main_file = std::fs::File::create("src/main.cpp").unwrap();
    write!(
        main_file,
        r#"#include "Game.hpp"

int main() {{
    Game().run();
    return 0;
}}
"#
    )
    .unwrap();

    let mut game_header = std::fs::File::create("src/Game.hpp").unwrap();
    write!(
        game_header,
        r#"#ifndef GAME_HEADER_INCLUDED
#define GAME_HEADER_INCLUDED

#include <NSEngine.hpp>

class Game : public ns::IEngine {{
public:
    Game() : IEngine(1280, 960, "{name}") {{}}
    ~Game() override {{}}

    void on_create() override;
    void on_update() override;
    void on_render() override;
    void on_destroy() override;

private:
    // Put some game state here.
}};

#endif // GAME_HEADER_INCLUDED
"#
    )
    .unwrap();

    let mut game_file = std::fs::File::create("src/Game.cpp").unwrap();
    write!(
        game_file,
        r#"#include "Game.hpp"

void Game::on_create() {{}}

void Game::on_update() {{}}

void Game::on_render() {{}}

void Game::on_destroy() {{}}
"#
    )
    .unwrap();

    let mut gitignore = std::fs::File::create(".gitignore").unwrap();
    writeln!(gitignore, ".ns/").unwrap();
}
