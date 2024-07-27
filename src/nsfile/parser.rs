use super::{KeyValue, NSData, NSFile, NSFileContent, NSValue};

fn parse_identifier(s: &str) -> Option<(String, &str)> {
    if s.is_empty() {
        return None;
    }
    let mut it = s.chars();
    match it.next().unwrap() {
        'a'..='z' | 'A'..='Z' | '_' => {}
        _ => return None,
    };
    let mut cnt = 1;
    for c in it {
        match c {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => cnt += 1,
            _ => break,
        }
    }
    Some((String::from(&s[..cnt]), &s[cnt..]))
}

fn parse_str(s: &str) -> Option<(String, &str)> {
    if s.len() < 2 {
        return None;
    }
    let mut it = s.chars();
    if it.next().unwrap() != '"' {
        return None;
    }
    let mut out = String::new();
    let mut cnt = 1;
    let mut bs = false;
    for c in it {
        if c == '\\' && !bs {
            bs = true;
        } else if c == '"' && !bs {
            cnt += 1;
            break;
        } else if c == '"' && bs {
            out.push(c);
        } else if bs {
            out.push('\\');
            out.push(c);
        } else {
            out.push(c);
        }
        cnt += 1;
    }
    Some((out, &s[cnt..]))
}

fn skip_whitespace(s: &str) -> &str {
    let mut skip_cnt = 0;
    for c in s.chars() {
        if c.is_whitespace() {
            skip_cnt += 1;
        } else {
            break;
        }
    }
    &s[skip_cnt..]
}

fn parse_data(s: &str, id: Option<String>) -> Option<(NSData, &str)> {
    let s = skip_whitespace(s);
    if !s.starts_with('{') {
        return None;
    }
    let mut s = skip_whitespace(&s[1..]);
    let mut inner = Vec::new();
    while !s.starts_with('}') {
        if let Some((kv, new_s)) = parse_entry(s) {
            inner.push(kv);
            s = skip_whitespace(new_s);
        } else {
            return None;
        }
    }
    Some((
        NSData {
            typename: id,
            inner,
        },
        &s[1..],
    ))
}

fn parse_array(s: &str) -> Option<(Vec<String>, &str)> {
    let s = skip_whitespace(s);
    if !s.starts_with('[') {
        return None;
    }
    let mut s = skip_whitespace(&s[1..]);
    let mut inner = Vec::new();
    while !s.starts_with(']') {
        if let Some((id, new_s)) = parse_identifier(s) {
            inner.push(id);
            s = skip_whitespace(new_s);
        } else if let Some((st, new_s)) = parse_str(s) {
            inner.push(st);
            s = skip_whitespace(new_s);
        } else {
            return None;
        }
    }
    Some((inner, &s[1..]))
}

fn parse_value(s: &str) -> Option<(NSValue, &str)> {
    let s = skip_whitespace(s);
    if let Some((id, s)) = parse_identifier(s) {
        let s = skip_whitespace(s);
        if s.starts_with('{') {
            if let Some((dat, s)) = parse_data(s, Some(id)) {
                return Some((NSValue::Data(dat), s));
            }
        } else {
            return Some((NSValue::Str(id), s));
        }
    } else if let Some((st, s)) = parse_str(s) {
        return Some((NSValue::Str(st), s));
    } else if s.starts_with('{') {
        if let Some((dat, s)) = parse_data(s, None) {
            return Some((NSValue::Data(dat), s));
        }
    } else if s.starts_with('[') {
        if let Some((arr, s)) = parse_array(s) {
            return Some((NSValue::Array(arr), s));
        }
    }
    None
}

fn parse_entry(s: &str) -> Option<(KeyValue, &str)> {
    let s = skip_whitespace(s);
    let (id, s) = parse_identifier(s)?;
    let s = skip_whitespace(s);
    if !s.starts_with('=') {
        return None;
    }
    let s = skip_whitespace(&s[1..]);
    let (val, s) = parse_value(s)?;
    Some(((id, val), s))
}

pub fn parse_file(filename: &str) -> Option<NSFile> {
    let mut content = NSFileContent::new();
    let file_content = std::fs::read_to_string(filename).ok()?;
    let mut s = file_content.as_str();

    while !s.is_empty() {
        let (kv, new_s) = parse_entry(s)?;
        s = skip_whitespace(new_s);
        content.push(kv);
    }

    Some(NSFile { content })
}
