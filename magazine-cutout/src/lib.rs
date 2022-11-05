// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_totals = HashMap::new();

    for word in magazine {
        let word_count = word_totals.get_mut(&word.to_string());
        match word_count {
            None => {
                word_totals.insert(word.to_string(), 1 as u32);
            },
            Some(prev_total) => {
                *prev_total += 1;
            }
        }
    }

    for word in note {
        let word_count = word_totals.get_mut(&word.to_string());
        match word_count {
            None => {
                return false;
            },
            Some(prev_total) => {
                if (*prev_total == 0)
                {
                    return false; // No words left
                }
                *prev_total -= 1 as u32;
            }
        }
    }
    return true;
}
