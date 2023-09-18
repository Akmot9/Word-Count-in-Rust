use std::collections::HashMap;
use csv::Writer;

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

    let mut wtr = Writer::from_path("word_count.csv").expect("Cannot create file");

    wtr.write_record(&["Word", "Count"]).expect("CSV write error");

    for (_, word) in &word_list {
        wtr.write_record(&[word.word.as_str(), word.count.to_string().as_str()])
            .expect("CSV write error");
    }

    wtr.flush().expect("CSV write error");
}
