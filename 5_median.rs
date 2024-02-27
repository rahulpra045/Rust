fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        // Even length array
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] as f64 + arr[mid_right] as f64) / 2.0
    } else {
        // Odd length array
        let mid = len / 2;
        arr[mid] as f64
    }
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 5];
    let median = find_median(&sorted_array);
    println!("Median: {}", median);

    let sorted_array2 = vec![1, 2, 3, 4];
    let median2 = find_median(&sorted_array2);
    println!("Median: {}", median2);
}
