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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use crate::dna_replication::pattern_count;

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
}