fn main() {
    let vstup_txt: Vec<_> = include_str!("input.txt").lines().collect();

    let mut hladame: Vec<_> = Vec::new();

    for i in &vstup_txt {
        let members: Vec<_> = i.split(',').collect();
        if members.contains(&"shinygold") {
            hladame.push(members[0]);
        }
    }
    let mut novota: Vec<_> = Vec::new();

    for j in &vstup_txt {
        for k in &hladame {
            let members: Vec<_> = j.split(',').collect();
            if members.contains(k) {
                novota.push(members[0]);
            }
        }
    }
    novota.sort();
    novota.dedup();
    println!("Novota : {:?}", novota.len()-1);

    let mut novota2: Vec<_> = Vec::new();

    for j in &vstup_txt {
        for k in &novota {
            let members: Vec<_> = j.split(',').collect();
            if members.contains(k) {
                novota2.push(members[0]);
            }
        }
    }
    println!("Novota 2 pred upravou : {}", novota2.len()-1);
    novota2.sort();
    novota2.dedup();
    println!("Novota 2 po uprave : {:?}", novota2.len()-1);

    let mut novota3: Vec<_> = Vec::new();

    for j in &vstup_txt {
        for k in &novota2 {
            let members: Vec<_> = j.split(',').collect();
            if members.contains(k) {
                novota3.push(members[0]);
            }
        }
    }
    println!("Novota 3 pred upravou : {}", novota3.len()-1);
    novota3.sort();
    novota3.dedup();
    println!("Novota 3 : {:?}", novota3.len()-1);
    let mut novota4: Vec<_> = Vec::new();

    for j in &vstup_txt {
        for k in &novota3 {
            let members: Vec<_> = j.split(',').collect();
            if members.contains(k) {
                novota4.push(members[0]);
            }
        }
    }
    println!("Novota 4 pred upravou : {}", novota4.len()-1);
    novota4.sort();
    novota4.dedup();
    println!("Novota 4 : {:?}", novota4.len()-1);

    let mut novota5: Vec<_> = Vec::new();

    for j in &vstup_txt {
        for k in &novota4 {
            let members: Vec<_> = j.split(',').collect();
            if members.contains(k) {
                novota5.push(members[0]);
            }
        }
    }
    println!("Novota 5 pred upravou : {}", novota5.len()-1);
    novota5.sort();
    novota5.dedup();
    println!("Novota 5 : {:?}", novota5.len()-1);

    let mut novota6: Vec<_> = Vec::new();

    for j in &vstup_txt {
        for k in &novota5 {
            let members: Vec<_> = j.split(',').collect();
            if members.contains(k) {
                novota6.push(members[0]);
            }
        }
    }
    println!("Novota 6 pred upravou : {}", novota6.len()-1);
    novota6.sort();
    novota6.dedup();
    println!("Novota 6 : {:?}", novota6.len()-1);

    let mut novota7: Vec<_> = Vec::new();

    for j in &vstup_txt {
        for k in &novota6 {
            let members: Vec<_> = j.split(',').collect();
            if members.contains(k) {
                novota7.push(members[0]);
            }
        }
    }
    println!("Novota 7 pred upravou : {}", novota7.len()-1);
    novota7.sort();
    novota7.dedup();
    println!("Novota 7 : {:?}", novota7.len()-1);

    let mut novota8: Vec<_> = Vec::new();

    for j in &vstup_txt {
        for k in &novota7 {
            let members: Vec<_> = j.split(',').collect();
            if members.contains(k) {
                novota8.push(members[0]);
            }
        }
    }
    println!("Novota 8 pred upravou : {}", novota8.len()-1);
    novota8.sort();
    novota8.dedup();
    println!("Novota 8 : {:?}", novota8.len()-1);

    let mut novota9: Vec<_> = Vec::new();

    for j in &vstup_txt {
        for k in &novota8 {
            let members: Vec<_> = j.split(',').collect();
            if members.contains(k) {
                novota9.push(members[0]);
            }
        }
    }
    println!("Novota 9 pred upravou : {}", novota9.len()-1);
    novota9.sort();
    novota9.dedup();
    println!("Novota 9 : {:?}", novota9.len()-1);

    let mut novota10: Vec<_> = Vec::new();

    for j in &vstup_txt {
        for k in &novota9 {
            let members: Vec<_> = j.split(',').collect();
            if members.contains(k) {
                novota10.push(members[0]);
            }
        }
    }
    println!("Novota 10 pred upravou : {}", novota10.len()-1);
    novota10.sort();
    novota10.dedup();
    println!("Novota 10 : {:?}", novota10.len()-1);




}

