
use super::node::Node;
use crate::texts;

pub struct FileSystem {
    pub root: Node,
    current_path: Vec<String>
}

fn file(name: &str, content: &str) -> Node {
    Node::new_file(name, content)
}

fn dir(name: &str, children: Vec<Node>) -> Node {
    Node::new_directory(name, children)
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: Self::build_root(),
            current_path: vec![]
        }
    }

    fn build_root() -> Node {
        let user = dir("daniel", vec![
            dir("Desktop", vec![
                file("welcome.txt", texts::WELCOME),
            ]),
            dir("Documents", vec![
                file("personnel_record.txt", texts::PERSONNEL_RECORD),
                file("personal_notes.txt", texts::PERSONAL_NOTES),
                file("access_manual.txt", texts::ACCESS_MANUAL),
/*
                dir("research", vec![
                    file("project_overview.txt", texts::PROJECT_OVERVIEW),
                    file("experiment_log.log", texts::EXPERIMENT_LOG),
                    file("prototype_notes.txt", texts::PROTOTYPE_NOTES),
                    file("internal_report.txt", texts::INTERNAL_REPORT),
                ]),
*/
                dir("maintenance", vec![
                    file("maintenance_log.log", texts::MAINTENANCE_LOG),
                    file("equipment_inventory.txt", texts::EQUIPMENT_INVENTORY),
                ]),
            ]),
        ]);

        let home = dir("home", vec![user]);

        let etc = dir("etc", vec![
            file("system_info.txt", texts::SYSTEM_INFO),
            file("terminal_manual.txt", texts::ACCESS_MANUAL),
        ]);

        let var = dir("var", vec![
            dir("log", vec![
                file("system.log", texts::SYSTEM_LOG),
                file("access.log", texts::ACCESS_LOG),
            ]),

            dir("mail", vec![
                dir("daniel", vec![
                    file("01_birthday.txt", texts::MAIL_BIRTHDAY),
/*
                    file("02_cabinet.txt", texts::MAIL_CABINET),
                    file("03_orpheus.txt", texts::MAIL_ORPHEUS),
                    file("04_demonstration.txt", texts::MAIL_DEMONSTRATION),
*/
                ]),
            ]),
/*
            dir("archive", vec![
                file("archive_index.txt", texts::ARCHIVE_INDEX),
                file("facility_map.dat", texts::FACILITY_MAP),
            ]),
*/
        ]);

        dir("/", vec![
            dir("bin", vec![
                Node::new_message("[contains binaries, irrelevant]"),
            ]),
            dir("boot", vec![
                Node::new_message("[bootloader stuff, irrelevant]"),
            ]),
            dir("dev", vec![
                Node::new_message("[device files, irrelevant]"),
            ]),
            etc,
            home,
            dir("lib", vec![
                Node::new_message("[shared libraries, irrelevant]"),
            ]),
            dir("mnt", vec![
                Node::new_message("[mount point, irrelevant]"),
            ]),
            dir("tmp", vec![
                Node::new_message("[temporary files, irrelevant]"),
            ]),
            dir("usr", vec![
                Node::new_message("[shared files, irrelevant]"),
            ]),
            dir("sys", vec![
                Node::new_message("[kernel interface, irrelevant]"),
            ]),
            var,
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
        let new_path = self.resolve_path(path);

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
    
    fn resolve_path(&self, path: &str) -> Vec<String> {
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
                ".."        => { new_path.pop(); }
                name        => { new_path.push(name.to_string()); }
            }
        }

        new_path
    }
 
    pub fn cat(&self, path: &str) -> Result<&str, String> {
        let resolved_path = self.resolve_path(path);

        match self.get_node(&resolved_path) {
            Some(node) => match node.get_content() {
                Some(content) => Ok(content),
                None if node.is_directory() => {
                    Err(format!("'{}' is a directory", path))
                }
                None => {
                    Err(format!("'{}' is not a readable file", path))
                }
            },
            None => Err(format!("file '{}' not found", path))
        }
    }

}
