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
                frequencies.insert(k_mer_str.to_string(), 1);
            } else {
                *existing_freq.unwrap() += 1;
            }
        });

    frequencies
}


/// From a `map`, returns all the keys where the value returns true for a given `predicate`
fn find_values_in_map<F>(
    map: &HashMap<String, i64>,
    predicate: F
) -> HashSet<String> where F: Fn(&i64) -> bool {
    map
        .iter()
        .filter_map(|key_value|
            if predicate(key_value.1) { Some(key_value.0) } else { None }
        )
        .cloned()
        .collect()
}


/// Returns the keys of the map that have the maximum value
fn max_map(frequency_map: &HashMap<String, i64>) -> HashSet<String> {
    let max_value_opt = frequency_map.values().max();

    match max_value_opt {
        Some(max_value) => {
            let result: HashSet<String> = find_values_in_map(
                frequency_map,
                |map_value| map_value == max_value
            );
            result
        },
        None => HashSet::new()
    }

}

/// Finds the most frequent k-mers for a certain `k` in the `text`
pub fn frequent_words(text: &str, k: usize) -> HashSet<String> {
    max_map(&frequency_table(text, k))
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


/// Finds k-mers that are clumps in a `genome`.
///
/// We define a `k`-mer as a "clump" if it appears many times within a short interval of the genome.
/// More formally, given a part of the genome with a certain `length`, and a certain `frequency`,
/// a k-mer Pattern forms a (`length`, `frequency`)-clump inside a (longer) string `genome`
/// if there is an interval of `genome` of that `length` in which this k-mer appears at least
/// `frequency` times.
///
/// This definition assumes that the k-mer completely fits within the interval and does not take
/// reverse complements into account. Also, it's a naive implementation following the course
/// structure to re-use methods and NOT optimised for performance.
pub fn find_clumps(genome: &str, length: usize, frequency: i64, k: usize) -> HashSet<String> {
    let genome_as_chars = genome.chars().collect::<Vec<char>>();

    let all_clumps: Vec<HashSet<String>> = genome_as_chars
        .windows(length)
        .filter_map(|window| {

            let genome_interval: String = window.iter().collect();
            let frequency_table = frequency_table(&genome_interval, k);
            let k_mers = find_values_in_map(
                &frequency_table,
                |map_value| map_value >= &frequency
            );

            if k_mers.is_empty() { None } else { Some(k_mers) }
        })
        .collect();

    // TODO Need to come back here and understand better the reference handling
    let combined_sets: Option<HashSet<String>> = all_clumps
        .into_iter()
        .reduce(|left, right|
            left.union(&right).cloned().collect()
        );

    match combined_sets {
        Some(set) => set.to_owned(),
        None => HashSet::new()
    }

}


/// Given a `genome`, creates a Skew vector that can be used to notice statistical patterns on the
/// 'G' and 'C' content which can help detect an Ori region, or detect if we are traversing the
/// forward or the reverse DNA half-strand.
pub fn gc_skew(genome: &str) -> Vec<i32> {

    let mut skew = 0;
    let mut skew_result: Vec<i32> = vec![0];

    genome
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .for_each(|nucleotide| {
            match nucleotide {
                'G' => skew = skew + 1,
                'C' => skew = skew - 1,
                _   => ()
            }
            skew_result.push(skew);
        });

    skew_result

}