
pub enum Node {
    File {
        name: String,
        content: String
    },
    Directory {
        name: String,
        children: Vec<Node>
    },
    Message {
        name: String
    }
}

impl Node {

    // constructors for all three types
    pub fn new_directory(name: &str, children: Vec<Node>) -> Self {
        Node::Directory {
            name: name.to_string(),
            children
        }
    }
    
    pub fn new_file(name: &str, content: &str) -> Self {
        Node::File{
            name: name.to_string(),
            content: content.to_string()
        }
    }

    pub fn new_message(name: &str) -> Self {
        Node::Message {
            name: name.to_string()
        }
    }
    
    // other useful functions
    pub fn get_name(&self) -> &str {
        // if a multiple patterns should do the same thing, you can use '|'
        match self {
            Node::File { name, .. }
            | Node::Directory { name, .. }
            | Node::Message { name } => name
        }
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, Node::Directory {..})
    }

    pub fn get_children(&self) -> Option<&[Node]> {
        match self {
            Node::Directory { children, .. } => Some(children),
            _ => None
        }
    }

    pub fn get_content(&self) -> Option<&str> {
        match self {
            Node::File { content, .. } => Some(content),
            _ => None
        }
    }

    pub fn find_child(&self, name: &str) -> Option<&Node> {
        self.get_children()?.
            iter().
            find(|child| child.get_name() == name)
    }

    pub fn add_child(&mut self, child: Node) {
        if let Node::Directory {children, ..} = self {
            children.push(child);
        }
    }

    pub fn get_child_mut(&mut self, name: &str) -> Option<&mut Node> {
        match self {
            Node::Directory { children, .. } => {
                children.iter_mut().find(|child| child.get_name() == name)
            }
            _ => None
        }
    }
}
