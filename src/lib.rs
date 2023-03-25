// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_count: HashMap<&str, u16> = HashMap::new();

    for word in magazine {
        word_count.entry(word)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }

    let mut result = true;

    for word in note {
        if let Some(count) = word_count.get(word)
    }

    println!("{:?}", word_count);
    result
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_note_construction() {
        let magazine = "two times three is not four".split_whitespace().collect::<Vec<&str>>();
        let note = "two times two is four".split_whitespace().collect::<Vec<&str>>();
        let _ = can_construct_note(&magazine, &note);
    }
}