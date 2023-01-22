use std::rc::{Rc, Weak};

struct MatrixDescription {
    column_size: u32,
    row_size: u32,
}

struct GraphNode {
    desc: MatrixDescription,
    parent: Weak<GraphOperator>,
    childs: Vec<Rc<GraphOperator>>,
}

enum GraphOperation {
    none,
    plus,
    minus,
    multiple,
    divide,
}

struct GraphOperator {
    op: GraphOperation,
    parents: Vec<Weak<GraphNode>>,
    childs: Vec<Rc<GraphNode>>,
}

fn simulate_graph(node: GraphNode) {}

fn main() {
    println!("Hello, world!");
}
