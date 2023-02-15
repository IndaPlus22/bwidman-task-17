use std::io;
use core::cmp;

fn min(a: usize, b: usize, c: usize) -> usize {
    cmp::min(a, cmp::min(b, c))
}

fn calc_edit_distance(str1: &str, str2: &str) -> usize {
    let m = str1.len() + 1;
    let n = str2.len() + 1;
    let mut matrix = vec![vec![0; m]; n];

    // Prefixes of str1 can be transformed into the empty string (0th prefix of str2)
    // By removing all characters
    for i in 1..m {
        matrix[0][i] = i;
    }
    
    // Empty str1 can be transformed into the prefixes by adding every character
    for i in 1..n {
        matrix[i][0] = i;
    }

    for i in 1..n {
        for j in 1..m {
            let mut cost = 0;
            if str1.as_bytes()[j] != str2.as_bytes()[i] {
                cost = 1;
            }

            matrix[j][i] = min(i, j, j + 1); // Test
        }
    }

    return matrix[n - 1][m - 1];
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
        let mut shortest_distance = usize::MAX;
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
