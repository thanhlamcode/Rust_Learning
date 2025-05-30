use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 1, 2, 2, 1, 1, 2, 4, 2, 5, 3, 4, 2, 6];


    let median = v[v.len() / 2];
    println!("median: {}", median);

    let mut counts = HashMap::new();

    for nums in v {
        *counts.entry(nums).or_insert(0) += 1;
    }
    println!("counts: {:?}", counts);

    let mode = counts.into_iter().max_by_key(|&(_, count)| count).unwrap();

    println!("mode: {:?}", mode);
}
