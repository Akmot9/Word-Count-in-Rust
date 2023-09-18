use std::collections::HashMap;

#[derive(Debug)]
struct Word {
    word: String,
    count: u32,
}

impl Word {
    fn new(word: String) -> Word {
        Word { word, count: 1 }
    }
}

fn main() {
    let mut word_list: HashMap<String, Word> = HashMap::new();
    let input_words = vec!["apple", "banana", "apple", "orange", "banana", "apple"];

    for word in input_words.iter() {
        if let Some(entry) = word_list.get_mut(&word.to_string()) {
            entry.count += 1;
        } else {
            word_list.insert(word.to_string(), Word::new(word.to_string()));
        }
    }

    for (_, word) in &word_list {
        println!("Word: {}, Count: {}", word.word, word.count);
    }
}
