
use super::node::Node;

pub struct FileSystem {
    pub root: Node,
    current_path: Vec<String>
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: Self::build_root(),
            current_path: vec![]
        }
    }
    fn build_root() -> Node {
        let user = Node::new_directory("daniel", vec![
            Node::new_directory("Documents", vec![]),
            Node::new_directory("Desktop", vec![]),
        ]);

        let home = Node::new_directory("home", vec![user]);

        Node::new_directory("/", vec![
            Node::new_directory("bin", vec![
                Node::new_message("*contains binaries, irrelevant*"),
            ]),
            Node::new_directory("boot", vec![
                Node::new_message("*bootloader stuff, irrelevant*"),
            ]),
            Node::new_directory("dev", vec![
                Node::new_message("*device files, irrelevant*"),
            ]),
            Node::new_directory("etc", vec![]),
            home,
            Node::new_directory("lib", vec![
                Node::new_message("*shared libraries, irrelevant*"),
            ]),
            Node::new_directory("mnt", vec![
                Node::new_message("*mount point, irrelevant*"),
            ]),
            Node::new_directory("tmp", vec![
                Node::new_message("*temporary files, irrelevant*"),
            ]),
            Node::new_directory("usr", vec![
                Node::new_message("*shared files, irrelevant*"),
            ]),
            Node::new_directory("sys", vec![
                Node::new_message("*kernel interface, irrelevant*"),
            ]),
            Node::new_directory("var", vec![
                Node::new_directory("log", vec![]),
            ]),
        ])
    }
    
    // path is a slice of a String, meaning arrays and Vecs, both are fine
    fn get_node(&self, path: &[String]) -> Option<&Node> {
        let mut current = &self.root;

        for part in path {
            // find_child returns an Option, thats why we can use ? instead of having to write a
            // whole match block. if Some(node) it is node, else None
            // only works because this function returns Option too
            current = current.find_child(part)?;
        }

        Some(current)
    }

    pub fn pwd(&self) -> String {
        if self.current_path.is_empt() {
            return "/".to_string();
        }

        format!("/{}", self.current_path.join("/"))
    }
}
