fn main() {
    let vstup_txt: Vec<_> = include_str!("input.txt").lines().collect();
    let gazda = spocitaj("shinygold", 1);
    println!("Finalny pocet vrecoch : {}", gazda);
}

fn spocitaj(vrece: &str, pocet_vrecoch: u64) -> u64 {
    let mut pocet_vrecoch = pocet_vrecoch;
    let vstup_txt: Vec<_> = include_str!("sample.txt").lines().collect();
    for i in &vstup_txt {
        let members: Vec<_> = i.split(',').collect();
        if members[0] == vrece && members.len() != 1 {
            println!("prva cast : {}, druha cast : {}", members[1], members[3]);
            pocet_vrecoch = pocet_vrecoch * (members[1].parse::<u64>().unwrap() + members[3].parse::<u64>().unwrap());
            println!("Pocet vrecoch : {}", pocet_vrecoch);
            pocet_vrecoch = pocet_vrecoch + spocitaj(members[2], members[1].parse::<u64>().unwrap());
        }
    }
    return pocet_vrecoch
}