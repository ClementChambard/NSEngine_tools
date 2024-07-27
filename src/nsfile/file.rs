use super::{NSFileContent, NSValue};

#[derive(Default, Clone, Debug)]
pub struct NSFile {
    pub content: NSFileContent,
}

impl NSFile {
    pub fn find_value(&self, key: &str) -> Option<&NSValue> {
        for kv in &self.content {
            if kv.0 == key {
                return Some(&kv.1);
            }
        }
        None
    }
    pub fn find_data_bloc(&self, key: &str) -> Option<&NSData> {
        match self.find_value(key)? {
            NSValue::Data(d) => Some(d),
            _ => None,
        }
    }
    pub fn find_str(&self, key: &str) -> Option<&str> {
        match self.find_value(key)? {
            NSValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn find_array(&self, key: &str) -> Option<&[String]> {
        match self.find_value(key)? {
            NSValue::Array(a) => Some(a),
            _ => None,
        }
    }
    pub fn get_array(&self, key: &str) -> &[String] {
        self.find_array(key).unwrap_or(&[])
    }
}

#[derive(Default, Clone, Debug)]
pub struct NSData {
    pub typename: Option<String>,
    pub inner: NSFileContent,
}

impl NSData {
    pub fn of_type(&self, ty: &str) -> Option<&Self> {
        if let Some(s) = &self.typename {
            if s == ty {
                return Some(self);
            }
        }
        None
    }
    pub fn find_value(&self, key: &str) -> Option<&NSValue> {
        for kv in &self.inner {
            if kv.0 == key {
                return Some(&kv.1);
            }
        }
        None
    }
    pub fn find_data_bloc(&self, key: &str) -> Option<&NSData> {
        match self.find_value(key)? {
            NSValue::Data(d) => Some(d),
            _ => None,
        }
    }
    pub fn find_str(&self, key: &str) -> Option<&str> {
        match self.find_value(key)? {
            NSValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn find_array(&self, key: &str) -> Option<&[String]> {
        match self.find_value(key)? {
            NSValue::Array(a) => Some(a),
            _ => None,
        }
    }
    pub fn get_array(&self, key: &str) -> &[String] {
        self.find_array(key).unwrap_or(&[])
    }
}
