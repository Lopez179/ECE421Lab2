#[derive(Debug)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>,
}

impl<'a> TreeNode<'a> {
    pub fn insert_node(&mut self, data: &'a str) {
        if self.data == data {
            return
        }

        let c_node = if data < self.data { &mut self.left_child } else {&mut self.right_child };

        match c_node {
            Some(node) => node.insert_node(data),

            None => {*c_node = Some(Box::new(TreeNode{data: data, left_child: None, right_child: None}))},
        }
    }
}

fn main() {
    let mut parent_node  = TreeNode {
        data: "5",
        left_child: None,
        right_child: None,
    };

    parent_node.insert_node("3");
    parent_node.insert_node("2");
    parent_node.insert_node("4");
    parent_node.insert_node("7");
    parent_node.insert_node("6");
    parent_node.insert_node("8");
    println!("{:#?}",&parent_node);
}
    