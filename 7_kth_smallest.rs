use std::collections::HashSet;
use std::io;

fn kth_smallest_with_duplicates(arr: &[i32], k: usize) -> Option<i32> {
    let distinct_elements: Vec<_> = arr.iter().cloned().collect::<HashSet<_>>().into_iter().collect();
    let mut sorted_distinct_elements = distinct_elements.iter().cloned().collect::<Vec<_>>();
    sorted_distinct_elements.sort();

    if k > sorted_distinct_elements.len() {
        return None;
    }

    Some(sorted_distinct_elements[k - 1])
}

fn main() {
    println!("Enter the elements of the array separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");
    let k: usize = k_input.trim().parse().expect("Invalid input");

    match kth_smallest_with_duplicates(&arr, k) {
        Some(kth_smallest) => println!("The {}th smallest element is: {}", k, kth_smallest),
        None => println!("Invalid value of k."),
    }
}
