fn shortest_words(sentence: &str) -> Vec<&str> {
    let mut shortest = Vec::new();
    let mut shortest_len = usize::MAX;

    for word in sentence.split_whitespace() {
        let len = word.len();
        if len < shortest_len {
            shortest_len = len;
            shortest.clear();
            shortest.push(word);
        } else if len == shortest_len {
            shortest.push(word);
        }
    }

    shortest
}

fn main() {
    let sentence = "Assigment is to find shortest word in a sentence";
    let shortest = shortest_words(sentence);
    if shortest.is_empty() {
        println!("No words found in the sentence.");
    } else {
        println!("The shortest words are: {:?}", shortest);
    }
}
