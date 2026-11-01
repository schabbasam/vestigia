
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
        if self.current_path.is_empty() {
            return "/".to_string();
        }

        format!("/{}", self.current_path.join("/"))
    }
    
    // this function returns a 'Result'
    // if it is a success, return () (empty value)
    // if it isnt a success, return a String (with error message)
    pub fn cd(&mut self, path: &str) -> Result<(), String> {
        let mut new_path = if path.starts_with('/') {
            Vec::new()
        } else {
            // without .clone(), the ownership would be transferred to new_path
            // but we don't want to edit current_path
            self.current_path.clone()
        };

        for part in path.split('/') {
            match part {
                "" | "."    => continue,
                // the curly brackets are necessary
                // since we don't want to return anything and all arms have to match
                ".."        => {new_path.pop();}
                name        => {new_path.push(name.to_string());}
            }
        }

        match self.get_node(&new_path) {
            Some(node) if node.is_directory() => {
                self.current_path = new_path;
                Ok(()) // part of Result<>, signals success with ()
            }
            // Err shows that these outcomes are not desired
            Some(_) => Err(format!("'{}' is not a directory", path)),
            None    => Err(format!("directory '{}' not found", path))
        }
    }

    pub fn ls(&self) -> Option<&[Node]> {
        // ? stops the program if node is not found
        self.get_node(&self.current_path)?.get_children()
    }

}
