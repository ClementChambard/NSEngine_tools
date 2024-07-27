mod cache;
mod cmakelist_builder;

pub use cache::CacheFileContents;
pub use cmakelist_builder::get_cmake_lists_content;

#[derive(Default)]
pub struct DotNS {
    pub cachefile: CacheFileContents,
}

fn read_dot_ns(dir: &std::path::Path) -> DotNS {
    let mut dotfile = DotNS::default();
    cache::read_cache_file(&format!("{}cache.ns", dir.display()), &mut dotfile);
    dotfile
}

fn init_dot_ns(dir: &std::path::Path) {
    std::fs::create_dir(dir).unwrap();
    std::fs::create_dir(format!("{}build/", dir.display())).unwrap();
    cache::create_default_cache_file(&format!("{}cache.ns", dir.display()));
}

// maybe add verbosity as param
pub fn check_dot_ns_and_init() -> DotNS {
    let dir = std::path::Path::new("./.ns/");

    if !dir.exists() {
        init_dot_ns(dir);
    }

    read_dot_ns(dir)
}
