mod bound_fn;
mod compare_fn;

use std::io::stdin;
use std::time::Instant;
use crate::bound_fn::{AdvancedBound, DMinBound, BoundFn};
use crate::compare_fn::{Order, CompareFn};

type Node = usize;
type Nodes = Vec<Node>;

type Cost = i32;
type Matrix<T> = Vec<Vec<T>>;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Cannot read stdin");
    let nb_nodes = line.trim().parse()
        .expect("Cannot parse the number of nodes");

    let mut seen = Vec::with_capacity(nb_nodes);
    seen.push(0);
    let mut unseen = Vec::with_capacity(nb_nodes - 1);
    for i in 0..nb_nodes - 1 {
        unseen.push(i + 1);
    }

    let cost = create_cost(nb_nodes);

    let mut d_min = Cost::MAX;
    for line in &cost {
        for &value in line {
            d_min = Cost::min(d_min, value);
        }
    }

    // Q1
    // permute(&mut seen.clone(), &mut unseen.clone());

    // Q2.A
    // permute_len(&mut seen.clone(), &mut unseen.clone(), 0, &cost);

    // Q2.B
    /*
    let clock = Instant::now();
    let mut pcc = Cost::MAX;
    permute_len_pcc(&mut seen.clone(), &mut unseen.clone(), 0, &cost, &mut pcc);
    println!("{}", pcc);
    println!("Elapsed time: {:.3}s", clock.elapsed().as_secs_f32());
    */

    // Q3
    let clock = Instant::now();
    let mut pcc = Cost::MAX;
    permute_len_pcc_bounded(&mut seen.clone(), &mut unseen.clone(), 0, &cost, &mut pcc, &DMinBound { d_min });
    println!("{}", pcc);
    println!("Elapsed time: {:.3}s", clock.elapsed().as_secs_f32());

    // Q4
    let clock = Instant::now();
    let mut pcc = Cost::MAX;
    permute_len_pcc_bounded(&mut seen.clone(), &mut unseen.clone(), 0, &cost, &mut pcc, &AdvancedBound);
    println!("{}", pcc);
    println!("Elapsed time: {:.3}s", clock.elapsed().as_secs_f32());

    // Q5
    let clock = Instant::now();
    let mut pcc = Cost::MAX;
    permute_len_pcc_bounded_and_ordered(&mut seen.clone(), &mut unseen.clone(), 0, &cost, &mut pcc, &AdvancedBound, &Order);
    println!("{}", pcc);
    println!("Elapsed time: {:.3}s", clock.elapsed().as_secs_f32());

    // Q6
    let clock = Instant::now();
    let mut pcc = Cost::MAX;
    let d_max = 10;
    lds(&mut seen.clone(), &mut unseen.clone(), 0, d_max, 0, &cost, &mut pcc, &AdvancedBound, &Order);
    println!("{}", pcc);
    println!("Elapsed time: {:.3}s", clock.elapsed().as_secs_f32());

}

fn permute(seen: &mut Nodes, unseen: &mut Nodes) {
    assert!(!seen.is_empty() && seen[0] == 0);

    if unseen.is_empty() {
        println!("{:?}", seen);
    } else {
        for i in 0..unseen.len() {
            seen.push(unseen.swap_remove(i));
            permute(seen, unseen);
            unseen.swap_insert(i, seen.pop().unwrap());
        }
    }
}

fn permute_len(seen: &mut Nodes, unseen: &mut Nodes, path_len: i32, cost: &Matrix<Cost>) {
    assert!(!seen.is_empty() && seen[0] == 0);

    if unseen.is_empty() {
        let path_len = path_len + cost[seen[seen.len() - 1]][0];
        println!("{}", path_len);
    } else {
        for i in 0..unseen.len() {
            seen.push(unseen.swap_remove(i));
            let path_len = path_len + cost[seen[seen.len() - 2]][seen[seen.len() - 1]];
            permute_len(seen, unseen, path_len, cost);
            unseen.swap_insert(i, seen.pop().unwrap());
        }
    }
}

fn permute_len_pcc(
    seen: &mut Nodes,
    unseen: &mut Nodes,
    path_len: Cost,
    cost: &Matrix<Cost>,
    pcc: &mut Cost,
) {
    assert!(!seen.is_empty() && seen[0] == 0);

    if unseen.is_empty() {
        let path_len = path_len + cost[seen[seen.len() - 1]][0];
        println!("{}", path_len);
        *pcc = Cost::min(*pcc, path_len);
    } else {
        for i in 0..unseen.len() {
            seen.push(unseen.swap_remove(i));
            let path_len = path_len + cost[seen[seen.len() - 2]][seen[seen.len() - 1]];
            permute_len_pcc(seen, unseen, path_len, cost, pcc);
            unseen.swap_insert(i, seen.pop().unwrap());
        }
    }
}

fn permute_len_pcc_bounded<B>(
    seen: &mut Nodes,
    unseen: &mut Nodes,
    path_len: Cost,
    cost: &Matrix<Cost>,
    pcc: &mut Cost,
    bound: &B,
)
    where B: BoundFn
{
    assert!(!seen.is_empty() && seen[0] == 0);

    if unseen.is_empty() {
        let path_len = path_len + cost[seen[seen.len() - 1]][0];
        *pcc = Cost::min(*pcc, path_len);
    } else {
        for i in 0..unseen.len() {
            seen.push(unseen.swap_remove(i));

            let new_path_len = path_len + cost[seen[seen.len() - 2]][seen[seen.len() - 1]];
            if new_path_len + bound.invoke(seen, unseen, cost) < *pcc {
                permute_len_pcc_bounded(
                    seen,
                    unseen,
                    new_path_len,
                    cost,
                    pcc,
                    bound,
                );
            }

            unseen.swap_insert(i, seen.pop().unwrap());
        }
    }
}

fn permute_len_pcc_bounded_and_ordered<B, F>(
    seen: &mut Nodes,
    unseen: &mut Nodes,
    path_len: Cost,
    cost: &Matrix<Cost>,
    pcc: &mut Cost,
    bound: &B,
    comparison: &F,
)
    where B: BoundFn,
          F: CompareFn
{
    assert!(!seen.is_empty() && seen[0] == 0);

    if unseen.is_empty() {
        let path_len = path_len + cost[seen[seen.len() - 1]][0];
        *pcc = Cost::min(*pcc, path_len);
    } else {
        let mut unseen = unseen.clone();
        unseen.sort_by(|&lhs, &rhs| comparison.cmp(lhs, rhs, seen, cost));
        for i in 0..unseen.len() {
            seen.push(unseen.swap_remove(i));

            let new_path_len = path_len + cost[seen[seen.len() - 2]][seen[seen.len() - 1]];
            if new_path_len < *pcc && new_path_len + bound.invoke(seen, &unseen, cost) < *pcc {
                permute_len_pcc_bounded_and_ordered(
                    seen,
                    &mut unseen,
                    new_path_len,
                    cost,
                    pcc,
                    bound,
                    comparison,
                );
            }

            unseen.swap_insert(i, seen.pop().unwrap());
        }
    }
}

fn lds<B, F>(
    seen: &mut Nodes,
    unseen: &mut Nodes,
    discrepancy: usize,
    max_discrepancy: usize,
    path_len: Cost,
    cost: &Matrix<Cost>,
    pcc: &mut Cost,
    bound: &B,
    comparison: &F,
)
    where B: BoundFn,
          F: CompareFn
{
    assert!(!seen.is_empty() && seen[0] == 0);

    if unseen.is_empty() {
        let path_len = path_len + cost[seen[seen.len() - 1]][0];
        *pcc = Cost::min(*pcc, path_len);
    } else {
        let mut unseen = unseen.clone();

        unseen.sort_by(|&lhs, &rhs| comparison.cmp(lhs, rhs, seen, cost));
        for i in 0..unseen.len() {
            seen.push(unseen.swap_remove(i));

            let new_path_len = path_len + cost[seen[seen.len() - 2]][seen[seen.len() - 1]];
            if discrepancy + i <= max_discrepancy && new_path_len + bound.invoke(seen, &unseen, cost) < *pcc {
                lds(
                    seen,
                    &mut unseen,
                    discrepancy + i,
                    max_discrepancy,
                    new_path_len,
                    cost,
                    pcc,
                    bound,
                    comparison,
                );
            }

            unseen.swap_insert(i, seen.pop().unwrap());
        }
    }
}

fn create_cost(n: usize) -> Matrix<Cost> {
    let mut cost = Vec::with_capacity(n);
    let max_cost = 1000;
    let mut seed = 1;

    for i in 0..n {
        cost.push(Vec::with_capacity(n));
        for j in 0..n {
            if i == j { cost[i].push(max_cost + 1); } else {
                seed = next_rand(seed);
                cost[i].push(1 + seed % max_cost);
            }
        }
    }

    cost
}

fn next_rand(seed: i32) -> i32 {
    let i = 16807 * (seed % 127773) - 2836 * (seed / 127773);
    if i > 0 { i } else { 2147483647 + i }
}

trait SwapInsert<T> {
    fn swap_insert(&mut self, index: usize, element: T);
}

impl<T> SwapInsert<T> for Vec<T> {
    #[inline]
    fn swap_insert(&mut self, index: usize, element: T) {
        self.push(element);
        let last_index = self.len() - 1;
        self.swap(index, last_index);
    }
}