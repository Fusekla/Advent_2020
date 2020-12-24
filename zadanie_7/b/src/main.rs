fn main() {
    let gazda = spocitaj("shinygold", 1);
    println!("Finalny pocet vrecoch : {}", gazda);
}

fn spocitaj(vrece: &str, pocet_vrecoch: u64) -> u64 {
    let mut pocet_vrecoch = pocet_vrecoch;
    let vstup_txt: Vec<_> = include_str!("sample.txt").lines().collect();
    for i in &vstup_txt {
        let members: Vec<_> = i.split(',').collect();
        if members[0] == vrece && members.len() != 1 {
            let mut j = 0;
            while j < members.len() -3 {
                println!("prva cast : {}, druha cast : {}", members[j+1], members[j+3]);
                pocet_vrecoch = pocet_vrecoch * (members[j+1].parse::<u64>().unwrap() + members[j+3].parse::<u64>().unwrap());
                println!("Pocet vrecoch : {}", pocet_vrecoch);
                pocet_vrecoch = pocet_vrecoch + spocitaj(members[j+2], members[j+1].parse::<u64>().unwrap());
                // pocet_vrecoch = pocet_vrecoch + spocitaj(members[j+4], members[j+3].parse::<u64>().unwrap());
                j += 2;
            }
        }
    }
    return pocet_vrecoch;
}