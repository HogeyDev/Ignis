use crate::config::Configuration;
use std::path::Path;

pub fn resolve_path(config: &Configuration, rel: String) -> Option<String> {
    let mut paths_with_rel = config.import_path_priority.clone();
    paths_with_rel.insert(0, ".".to_owned());

    for path in paths_with_rel.iter() {
        let full_path = Path::new(path).join(&rel);

        if full_path.exists() {
            return Some(full_path.to_string_lossy().into_owned());
        }
    }

    None
}
