use std::io::fs::walk_dir;
use std::path::posix::Path;
use std::collections::HashMap;


pub type GroupedFiles = HashMap<String, Vec<Path>>;

/// Walks through the directory gathering files with the given extensions.
/// Returns a map from each extension to file paths.
pub fn gather_files(path: String, exts: &[String]) -> GroupedFiles {
    let thepath = Path::new(path);
    let mut groups = HashMap::new();

    match walk_dir(&thepath) {
        Ok(mut fpaths) => {
            for fpath in fpaths {
                if let Some(s) = fpath.extension_str() {
                    let string = s.to_string();
                    if exts.contains(&string) {
                        if !groups.contains_key(&string) {
                            groups.insert(string.clone(), vec![]);
                        }
                        groups.get_mut(&string).unwrap().push(fpath.clone());
                    }
                }
            }
        },
        Err(_) => panic!("Failed to open directory"),
    };

    groups
}
