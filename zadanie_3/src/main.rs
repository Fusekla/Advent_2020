fn main() {
    let vstup_txt: Vec<_> = include_str!("input.txt").lines().collect();
    let members = vstup_txt.len();
    let mut riadok = 1;
    let mut pahylov = 0;

    for i in 0..members {
        if i %2 == 0 {
            let mut lojzo: Vec<char> = vstup_txt[i].chars().collect();

            let mut j = 1;

            while j < riadok {
                let mut traktor: Vec<char> = lojzo.drain(0..1).collect();

                lojzo.append(&mut traktor);
                j += 1;
                if j == riadok {
                    if lojzo[0] == '#' {
                        pahylov += 1
                    }
                }
            }
            println!("Spracovali sme riadok {}", riadok);
            riadok += 1;
        }
    }
    println!("Narafali sme do {} pahylov.", pahylov);
}
