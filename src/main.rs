use std::{
    cmp::Ordering,
    io,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct MatrixDescription {
    column_size: u32,
    row_size: u32,
}

impl MatrixDescription {
    fn create(column_size: u32, row_size: u32) -> MatrixDescription {
        MatrixDescription {
            column_size,
            row_size,
        }
    }

    fn inv(&self) -> MatrixDescription {
        MatrixDescription {
            column_size: self.row_size,
            row_size: self.column_size,
        }
    }

    fn can_multiply(&self, other: &MatrixDescription) -> bool {
        (&self).column_size == other.row_size
    }
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
    MatrixDescription::create(3, 4);
}
