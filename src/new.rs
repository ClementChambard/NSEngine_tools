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
    let ver = crate::verbosity::Verbosity::new(matches);

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
        println!("Error: can't create project in existing directory: {dir:?}");
        return;
    }

    let _has_nsengine = if !nsengine_dir.exists() {
        println!("Warning: can't find NSEngine.");
        false
    } else {
        ver.msg(1, &format!("Using NSEngine from dir: {nsengine_dir:?}"));
        true
    };

    ver.msg(1, &format!("Using NSEngine from dir: {nsengine_dir:?}"));
    ver.create_dir(dir);
    ver.set_current_dir(dir);
    ver.create_dir("src");

    // TEMP
    std::process::Command::new("cp")
        .arg("-r")
        .arg(format!("{}/assets", nsengine_dir.display()))
        .arg(".")
        .output()
        .expect(&format!(
            "Error running command 'cp -r {}/assets .'.",
            nsengine_dir.display()
        ));
    // ver.create_dir("assets");

    ver.create_file_with_data(
        "project.ns",
        &crate::nsfile::make_default_project_ns(name, &format!("{}", nsengine_dir.display())),
    );

    ver.create_file_with_data("src/main.cpp", DEFAULT_MAIN_CPP);
    ver.create_file_with_data("src/Game.hpp", DEFAULT_GAME_HPP);
    ver.create_file_with_data("src/Game.cpp", DEFAULT_GAME_CPP);
    ver.create_file_with_data(".gitignore", ".ns/\n");
}

const DEFAULT_MAIN_CPP: &str = r#"#include "Game.hpp"

int main() {{
    Game().run();
    return 0;
}}
"#;

const DEFAULT_GAME_HPP: &str = r#"#ifndef GAME_HEADER_INCLUDED
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
"#;

const DEFAULT_GAME_CPP: &str = r#"#include "Game.hpp"

void Game::on_create() {{}}

void Game::on_update() {{}}

void Game::on_render() {{}}

void Game::on_destroy() {{}}
"#;
