mod defaults;
mod file;
mod parser;

pub use defaults::make_default_project_ns;

pub use file::NSData;
pub use file::NSFile;

#[derive(Clone, Debug)]
pub enum NSValue {
    Str(String),
    Array(Vec<String>),
    Data(NSData),
}

pub type KeyValue = (String, NSValue);

pub type NSFileContent = Vec<KeyValue>;

pub use parser::parse_file;
