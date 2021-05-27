use crate::{Nodes, Matrix, Cost};

pub trait BoundFn {
    fn invoke(&self, seen: &Nodes, unseen: &Nodes, cost: &Matrix<Cost>) -> Cost;
}

pub struct DMinBound {
    pub d_min: i32,
}

impl BoundFn for DMinBound {
    fn invoke(&self, _: &Nodes, unseen: &Nodes, _: &Matrix<Cost>) -> Cost {
        self.d_min * (unseen.len() + 1) as Cost
    }
}

pub struct AdvancedBound;

impl BoundFn for AdvancedBound {
    fn invoke(&self, seen: &Nodes, unseen: &Nodes, cost: &Matrix<Cost>) -> Cost {
        if unseen.is_empty() {
            cost[seen[seen.len() - 1]][0]
        } else {
            let mut l = Cost::MAX;

            let last_seen = seen[seen.len() - 1];
            for &sj in unseen {
                if cost[last_seen][sj] < l {
                    l = cost[last_seen][sj];
                }
            }

            let mut sum_li = 0;
            for &lhs in unseen {
                let mut li = cost[lhs][0];
                for &rhs in unseen {
                    if lhs != rhs && cost[lhs][rhs] < li {
                        li = cost[lhs][rhs];
                    }
                }

                sum_li += li;
            }

            l + sum_li
        }
    }
}