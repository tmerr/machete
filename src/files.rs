pub struct FileGroup {
    ext: String,
    filenames: Vec<String>,
}

/// Returns vectors of source filenames grouped by extension.
pub fn gather_files(path: String, exts: Vec<String>) -> Vec<FileGroup> {
    vec![FileGroup {
        ext: "test".to_string(),
        filenames: vec!["test".to_string()]}
    ]
}
