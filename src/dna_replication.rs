use std::collections::{HashMap, HashSet};
use crate::utils::bio_utils::DNA_COMPLEMENT;

#[cfg(test)]
mod tests;

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


/// Finds the reverse complement of a `dna` string
pub fn dna_reverse_complement(dna: &str) -> String {
    dna.chars()
        .map(|nucleotide| DNA_COMPLEMENT
            .get(&nucleotide)
            .expect("DNA must be valid nucleotides [A, T, G, C]. Case is irrelevant.")
        )
        .rev()
        .collect()
}


/// Creates a list of offsets specifying all starting positions where
/// `pattern` appears as a substring of `genome`.
pub fn pattern_matching(pattern: &str, genome: &str) -> Vec<i128> {

    let pattern_as_chars = pattern.chars().collect::<Vec<char>>();
    let genome_as_chars = genome.chars().collect::<Vec<char>>();

    let pattern_len = pattern.len();
    let mut indices: Vec<i128> = Vec::new();

    for index in 0..genome.len() - pattern_len + 1 { // + 1 because for loop is exclusive on the right
        if pattern_as_chars == &genome_as_chars[index ..= index + pattern_len - 1] {
            indices.push(index as i128);
        }
    }

    indices

}