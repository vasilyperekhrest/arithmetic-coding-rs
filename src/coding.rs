pub mod arithmetic {
    use rug::Rational;
    use std::collections::HashMap;

    pub fn encode(ctr: &HashMap<char, usize>, s: &str) -> Rational {
        let mut segments: HashMap<char, [Rational; 2]> = HashMap::new();
        let mut counter: Rational = Rational::new();
        let mut left: Rational = Rational::new();
        let mut right: Rational = Rational::from((1, 1));

        for (symbol, count) in ctr {
            let probability: Rational = Rational::from((*count, s.chars().count()));
            segments.insert(
                symbol.clone(),
                [counter.clone(), counter.clone() + probability.clone()]
            );
            counter += probability;
        }

        for symbol in s.chars() {
            let new_left = left.clone() + (right.clone() - left.clone()) * segments[&symbol][0].clone();
            let new_right = left.clone() + (right.clone() - left.clone()) * segments[&symbol][1].clone();

            right = new_right;
            left = new_left;
        }

        (left + right) / 2
    }

    pub fn decode(ctr: &HashMap<char, usize>, code: &Rational) -> String {
        let mut segments: HashMap<char, [Rational; 2]> = HashMap::new();
        let mut char_code: Rational = Rational::from(code);

        let mut counter: Rational = Rational::new();

        let str_size: usize = ctr.values().sum();
        let mut decoded_string: String = String::with_capacity(str_size);

        for (symbol, count) in ctr {
            let probability: Rational = Rational::from((*count, str_size));
            segments.insert(
                symbol.clone(),
                [counter.clone(), counter.clone() + probability.clone()]
            );
            counter += probability;
        }

        for _ in 0..str_size {
            for (symbol, [first, second]) in &segments {
                if *first <= char_code && char_code < *second {
                    decoded_string.push(*symbol);
                    char_code = (char_code - first.clone()) / (second.clone() - first.clone());
                    break;
                }
            }
        }

        decoded_string
    }
}