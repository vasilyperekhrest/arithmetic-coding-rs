pub mod counter {
    use std::collections::{HashMap, HashSet};

    pub fn char_counter(string: &str) -> HashMap<char, usize>{
        let mut char_counter_map: HashMap<char, usize> = HashMap::new();
        let alpha: HashSet<char> = string.chars().collect();

        for char in &alpha {
            char_counter_map.insert(char.clone(), string.matches(char.clone()).count());
        }

        char_counter_map
    }
}