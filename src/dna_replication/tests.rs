use std::collections::{HashMap, HashSet};
use std::fs;
use crate::dna_replication::{dna_reverse_complement, frequent_words, pattern_count, pattern_matching};
use crate::utils::file_utils::read_two_line_file;


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
        let (text, pattern) = read_two_line_file(
            &format!("resources/dna_replication/pattern_count/inputs/input_{iter}.txt")
        );
        assert_eq!(pattern_count(&text, &pattern), correct_answers_per_file[&iter])
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

#[test]
fn dna_reverse_complement_test() {

    for iter in 1..4 {

        let input_path =
            format!("resources/dna_replication/reverse_complement/inputs/input_{iter}.txt");
        let output_path =
            format!("resources/dna_replication/reverse_complement/outputs/output_{iter}.txt");

        let dna = fs::read_to_string(input_path)
            .expect("Should have been able to read the file");
        let reverse_complement = fs::read_to_string(output_path)
            .expect("Should have been able to read the file");

        assert_eq!(dna_reverse_complement(&dna), reverse_complement)
    }

}


#[test]
fn pattern_matching_test() {

    for iter in 1..7 {

        let (pattern, text) = read_two_line_file(
            &format!("resources/dna_replication/pattern_matching/inputs/input_{iter}.txt")
        );

        let correct_offsets: Vec<i128> =
            fs::read_to_string(format!("resources/dna_replication/pattern_matching/outputs/output_{iter}.txt"))
                .expect("Should have been able to read the file")
                .split(" ")
                .map(|x| x.parse::<i128>().unwrap())
                .collect();

        assert_eq!(pattern_matching(&pattern, &text), correct_offsets)

    }

    // Also test a real example: Genome of 'Vibrio Cholerae'
    let vibrio_cholerae_genome =
        fs::read_to_string("resources/dna_replication/pattern_matching/inputs/Vibrio_cholerae.txt")
            .expect("Should have been able to read the file");

    let vibrio_cholerae_pattern = "ATGATCAAG";

    let calculated_positions = pattern_matching(&vibrio_cholerae_pattern, &vibrio_cholerae_genome);
    let expected_positions = [
        116556, 149355, 151913, 152013, 152394, 186189, 194276, 200076, 224527, 307692, 479770,
        610980, 653338, 679985, 768828, 878903, 985368
    ];

    assert_eq!(calculated_positions, expected_positions)

}
