struct binary_tree{
    root:Option<Box<node>>
}

struct Node{
    value: i32,
    left:Option<Box<node>>,
    right: Option<Box<node>>
}
