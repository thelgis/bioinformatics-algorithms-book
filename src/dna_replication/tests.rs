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
    let ori_region_of_vibrio_cholerae =
        "\
            atcaatgatcaacgtaagcttctaagcatgatcaaggtgctcacacagtttatccacaac\
            ctgagtggatgacatcaagataggtcgttgtatctccttcctctcgtactctcatgacca\
            cggaaagatgatcaagagaggatgatttcttggccatatcgcaatgaatacttgtgactt\
            gtgcttccaattgacatcttcagcgccatattgcgctggccaaggtgacggagcgggatt\
            acgaaagcatgatcatggctgttgttctgtttatcttgttttgactgagacttgttagga\
            tagacggtttttcatcactgactagccaaagccttactctgcctgacatcgaccgtaaat\
            tgataatgaatttacatgcttccgcgacgatttacctcttgatcatcgatccgattgaag\
            atcttcaattgttaattctcttgcctcgactcatagccatgatgagctcttgatcatgtt\
            tccttaaccctctattttttacggaagaatgatcaagctgctgctcttgatcatcgtttc\
            ";

    // Experiments have revealed that bacterial DnaA boxes are usually 9 nucleotides long
    // The probability that there exists a 9-mer appearing three or more times in a randomly
    // generated DNA string of length 500 is approximately 1/1300
    assert_eq!(
        frequent_words(ori_region_of_vibrio_cholerae, 9),
        HashSet::from([
            "atgatcaag".to_string(), // All of these are found 3 times in the ori
            "cttgatcat".to_string(),
            "tcttgatca".to_string(),
            "ctcttgatc".to_string()])
    );

    // Test edge-case of empty text
    assert_eq!(
        frequent_words("", 1),
        HashSet::new()
    );
}