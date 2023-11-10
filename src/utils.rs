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