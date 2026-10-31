
pub struct FileSystem {
    pub root: Node,
    pub current_path: Vec<String>
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: Self::build_root(),
            current_path: vec![]
        }
    }
}
