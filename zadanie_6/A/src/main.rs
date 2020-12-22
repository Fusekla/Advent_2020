fn main() {
    let vstup_txt: Vec<_> = include_str!("input.txt").split("\r\n\r\n").collect();

    let mut odpovedi_spolu = 0;

    for entry in vstup_txt {
        let mut vstup_chars: Vec<char> = entry.chars().collect();
        vstup_chars.sort();
        vstup_chars.dedup();
        while vstup_chars[0] == '\r' || vstup_chars[0] == '\n' {
            vstup_chars.remove(0);
        }
        odpovedi_spolu = odpovedi_spolu + vstup_chars.len()
    }

    println!("Odpovedi dohromady : {}", odpovedi_spolu);
}
