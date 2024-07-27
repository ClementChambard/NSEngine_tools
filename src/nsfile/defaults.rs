// TODO: Should take into account the different nsengine configurations

pub fn make_default_project_ns(project_name: &str, engine_path: &str) -> String {
    format!(
        r#"PROJECT = {}
NSENGINE = BUILD {{
  PATH = "{}"
}}
"#,
        project_name, engine_path
    )
}
