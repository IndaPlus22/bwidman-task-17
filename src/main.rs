// Solution to Kattis problem "Rättstavning"
// Author: Benjamin Widman (bwidman)
use std::io;
use std::io::prelude::*;
use core::cmp;

// Minimum of three values
fn min(a: usize, b: usize, c: usize) -> usize {
    cmp::min(a, cmp::min(b, c))
}

// Calculates the Levenshtein distance between two strings dynamically bottom-up instead of recursively top-down,
// resulting in linear time complexity instead of exponential
// Big inspiration from https://en.wikipedia.org/wiki/Levenshtein_distance
fn calc_edit_distance(str1: &str, str2: &str) -> usize {
    // str1 spans the left side while str2 spans the top
    let m = str1.len() + 1;
    let n = str2.len() + 1;
    let mut matrix = vec![vec![0; n]; m];

    // Prefixes of str1 can be transformed into the empty string (0th prefix of str2)
    // By removing all characters
    for i in 1..m {
        matrix[i][0] = i;
    }
    
    // Empty str1 can be transformed into the prefixes of str2 by adding every character
    for j in 1..n {
        matrix[0][j] = j;
    }

    // Fill remaining matrix with edit distances between the substrings in matrix[i][j]
    for i in 1..m {
        for j in 1..n {
            let mut replace_cost = 0;
            if str1.as_bytes()[i - 1] != str2.as_bytes()[j - 1] { // If 
                replace_cost = 1;
            }

            // This cell's edit distance is the minimum out of the three possible edits
            matrix[i][j] = min(
                matrix[i - 1][j] + 1,                 // Deletion
                matrix[i][j - 1] + 1,                // Insertion
                matrix[i - 1][j - 1] + replace_cost // Replacement
            );
        }
    }

    return matrix[m - 1][n - 1];
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
    let lines = io::stdin().lock()
        .lines()
        .map(|x| x.unwrap());

    for misspelled_word in lines {
        let mut shortest_distance = usize::MAX;
        let mut matching_words: Vec<String> = vec![]; // Words with lowest edit distance

        // Calculate edit distance between misspelled word and every word in the word list
        // Save the ones with the lowest edit distance
        for word in &word_list {
            let distance = calc_edit_distance(&misspelled_word, word);

            if distance <= shortest_distance {
                // Clear previous matches words if new shortest edit distance is found
                if distance < shortest_distance {
                    matching_words.clear();
                    shortest_distance = distance; // Update shortest distance
                }
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
