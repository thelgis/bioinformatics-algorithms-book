use phf::phf_map;


static DNA_COMPLEMENT: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "T",
    "T" => "A",
    "G" => "C",
    "C" => "G",
    "a" => "t",
    "t" => "a",
    "g" => "c",
    "c" => "g"
};


pub fn dna_complement(nucleotide: &str) -> String {
    DNA_COMPLEMENT
        .get(nucleotide)
        .expect("Must use a valid nucleotide [A, T, G, C]. Case is irrelevant.")
        .to_string()
}