use std::io;

fn calc_edit_distance(str1: &str, str2: &str) -> u32 {
    todo!()
}

fn main() {
    // Get word list
    let mut word_list: Vec<String> = vec![];
    loop {
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();
        word = word.trim().to_string();

        if word == "#" { // End of word list
            break;
        }
        word_list.push(word);
    }

    // Get misspelled words
    let lines = io::stdin()
        .lines()
        .map(|x| x.unwrap());

    for misspelled_word in lines {
        let mut shortest_distance = u32::MAX;
        let mut matching_words: Vec<String> = vec![];

        // Calculate edit distance between misspelled word and every word in the word list
        // Find the ones with the lowest edit distance
        for word in &word_list {
            let distance = calc_edit_distance(&misspelled_word, word);

            if distance <= shortest_distance {
                shortest_distance = distance;
                matching_words.push(word.to_string());
            }
        }

        // Print result
        print!("{misspelled_word} ({shortest_distance})");
        for word in matching_words {
            print!(" {word}");
        }
        println!();
    }
}
