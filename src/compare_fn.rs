use crate::{Node, Nodes, Matrix, Cost};
use std::cmp::Ordering;

pub trait CompareFn {
    fn cmp(&self, lhs: Node, rhs: Node, seen: &Nodes, cost: &Matrix<Cost>) -> Ordering;
}

pub struct Order;

impl CompareFn for Order {
    fn cmp(&self, lhs: Node, rhs: Node, seen: &Nodes, cost: &Matrix<Cost>) -> Ordering {
        let last_seen = seen[seen.len() - 1];
        cost[last_seen][lhs].cmp(&cost[last_seen][rhs])
    }
}