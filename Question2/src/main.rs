#[derive(Debug)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>,
}

// Rust requires knowing the size of all types at compile time, because unlike most other languages, rust doesn't allow implicit heap
// alocation (this is necessary as part of having syntax dependent on memory safety). 

// When struct contain borrowed references, lifetimes need to be explicitly specified. Other languages don't need this because they use garbage
// collection or manual memory management. The lifetime specifier tells the compiler that data needs to live aslong as TreeNode.

fn main() {

}