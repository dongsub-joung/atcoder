#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>)

#[derive(Debug)]
struct Node<T: Ord>{
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

#[derive(Debug)]
pub struct BinaryTree<T: Ord>{
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T>{
    
}
