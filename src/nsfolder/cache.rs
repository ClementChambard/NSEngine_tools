use std::io::Write;

#[derive(Default)]
pub struct CacheFileContents {
    pub last_project_ns_read: u32, // should be timestamp
}

pub fn read_cache_file(_path: &str, dotfile: &mut super::DotNS) {
    dotfile.cachefile.last_project_ns_read = 0;
}

pub fn create_default_cache_file(path: &str) {
    let mut cache_file = std::fs::File::create(path).unwrap();
    write!(cache_file, "Default ns cache file.").unwrap();
}
