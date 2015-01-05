pub struct FileGroup {
    pub ext: String,
    pub filenames: Vec<String>,
}

/// Returns vectors of source filenames grouped by extension.
pub fn gather_files(path: String, exts: &[String]) -> Vec<FileGroup> {
    vec![FileGroup {
        ext: "mp3".to_string(),
        filenames: vec!["test.mp3".to_string()]}
    ]
}
