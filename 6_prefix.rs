fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let first_string = strings[0];
    let mut prefix = String::new();

    'outer: for (i, c) in first_string.chars().enumerate() {
        for s in &strings[1..] {
            if let Some(sc) = s.chars().nth(i) {
                if sc != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(c);
    }

    prefix
}

fn main() {
    let strings = ["flower", "flow", "flight"];
    let prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", prefix);

    let strings2 = ["dog", "racecar", "car"];
    let prefix2 = longest_common_prefix(&strings2);
    println!("Longest common prefix: {}", prefix2);
}
