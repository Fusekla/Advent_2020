// osiris

fn main() {
    let vstup_txt:Vec<_> = include_str!("input.txt").lines().collect();
    let mut vstup = Vec::new();

    for i in vstup_txt {
        vstup.push(i.parse::<i32>().unwrap());
    };
    for j in &vstup {
        for k in &vstup {
            for l in &vstup {
                if j + k + l == 2020 {
                    println!("Hladane cisla su {}, {} a {}1", j, k, l);
                    println!("Ich sucin je {}", j * k * l);
                }
            }
        }
    };
}

// lojzo
