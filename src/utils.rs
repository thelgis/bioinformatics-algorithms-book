pub mod bio_utils {
    use phf::phf_map;

    pub static DNA_COMPLEMENT: phf::Map<char, char> = phf_map! {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            'a' => 't',
            't' => 'a',
            'g' => 'c',
            'c' => 'g'
        };

}


pub mod file_utils {
    use std::fs;

    /// Splits file content that is specifically two lines in a tuple
    pub fn read_two_line_file(file_path: &str) -> (String, String) {
        let file_contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        let file_contents_vec: Vec<&str> = file_contents.split("\n").collect();
        (file_contents_vec[0].to_string(), file_contents_vec[1].to_string())
    }

}