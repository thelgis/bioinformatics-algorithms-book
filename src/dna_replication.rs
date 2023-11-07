use std::collections::{HashMap, HashSet};

/// Finds the number of times that the `pattern` is found in the `text`
pub fn pattern_count(text: &str, pattern: &str) -> usize {

    let text_as_chars = text.chars().collect::<Vec<char>>();
    let pattern_as_chars = pattern.chars().collect::<Vec<char>>();

    text_as_chars
        .windows(pattern.len())
        .map(|window| window == pattern_as_chars)
        .filter(|x| *x == true)
        .count()
}

/// Finds the most frequent k-mers for a certain `k` in the `text`
pub fn frequent_words(text: &str, k: usize) -> HashSet<String> {

    /// Builds a HashMap <k-mer->frequency> of the frequency at which each k-mer for a certain `k`
    /// appears in the `text`
    fn frequency_table(text: &str, k: usize) -> HashMap<String, i64> {
        let text_as_chars = text.chars().collect::<Vec<char>>();
        let mut frequencies: HashMap<String, i64> = HashMap::new();

        text_as_chars.windows(k)
            .for_each(|k_mer| {
                let k_mer_str = k_mer.iter().collect::<String>();
                let existing_freq = frequencies.get_mut(&k_mer_str);

                if existing_freq == None {
                    frequencies.insert(k_mer_str.to_string(), 0);
                } else {
                    *existing_freq.unwrap() += 1;
                }
            });

        frequencies
    }

    /// Returns the keys of the map that have the maximum value
    fn max_map(frequency_map: HashMap<String, i64>) -> HashSet<String> {
        let max_value_opt = frequency_map.values().max();

        match max_value_opt {
            Some(max_value) => {
                let result: HashSet<String> = frequency_map
                    .iter()
                    .filter_map(|key_value|
                        if key_value.1 == max_value { Some(key_value.0) } else { None }
                    )
                    .cloned()
                    .collect();

                result
            },
            None => HashSet::new()
        }


    }

    max_map(frequency_table(text, k))

}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use crate::dna_replication::{frequent_words, pattern_count};

    #[test]
    fn pattern_count_test() {
        let correct_answers_per_file = HashMap::from([
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 4),
            (5, 2),
            (6, 5),
            (7, 9),
            (8, 294),
        ]);

        for iter in 1..9 {

            let file_contents = fs::read_to_string(
                format!("resources/dna_replication/pattern_count/inputs/input_{iter}.txt")
            ).expect("Should have been able to read the file");

            let file_contents_vec: Vec<&str> = file_contents
                .split("\n")
                .collect();

            let text = file_contents_vec[0];
            let pattern = file_contents_vec[1];

            assert_eq!(pattern_count(text, pattern), correct_answers_per_file[&iter])
        }

    }

    #[test]
    fn frequent_words_test() {
        assert_eq!(
            frequent_words("ACGTTGCATGTCGCATGATGCATGAGAGCT", 4),
            HashSet::from(["CATG".to_string(), "GCAT".to_string()])
        );

        assert_eq!(
            frequent_words("", 4),
            HashSet::new()
        );
    }
}