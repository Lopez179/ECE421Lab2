#[derive(Debug)]
enum Tree<T: Ord> {
    Node {
            data: T,
            left_child: Box<Tree<T>>,
            right_child: Box<Tree<T>>,
        },
    Empty,
}

impl<T: Ord> Tree<T> {
    pub fn insert_node(&mut self, inserted_data: T) {
        match self {
            Tree::Empty => {
                *self = Tree::Node{data: inserted_data, left_child: Box::new(Tree::Empty), right_child: Box::new(Tree::Empty)};
            }
            Tree::Node { data, left_child, right_child } => {
                if *data == inserted_data {
                    return
                }

                if inserted_data < *data { 
                    left_child.insert_node(inserted_data);
                }
                else {
                    right_child.insert_node(inserted_data);
                }
            }
        }
    }
}

fn main() {
    let mut tree: Tree<i32> = Tree::Empty;
    tree.insert_node(4);
    tree.insert_node(2);
    tree.insert_node(6);
    tree.insert_node(9);
    println!("{:?}", tree);
}

// Options are essentially enums with variants Some and None. Empty behaves much like the none in options. That's why this code does not
// options like in question 3.
//
// The struct based approach is probably better as it's use has clearer intentions, and is more memory efficient. Enums came with overhead
// to being more flexible with the types of data they can use. For this same reason, it's harder to read code that use the enums.
