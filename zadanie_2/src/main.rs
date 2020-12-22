fn main() {
    let vstup_txt: Vec<_> = include_str!("input.txt").lines().collect();

    // println!("{}", vstup_txt[0]);

    let members = vstup_txt.len();
    let mut dobrych_hesiel = 0;

    for i in 0..members {
        let lojzo: Vec<_> = vstup_txt[i].split(|c| c == '-' || c == ' ').collect();

        // println!{"{:?}", lojzo};

        let min: i32 = lojzo[0].parse::<i32>().unwrap();
        let max: i32 = lojzo[1].parse::<i32>().unwrap();

        let pismenko: char = lojzo[2].chars().next().unwrap();
        let heslo = lojzo[3];

        // println!("minimum : {}, maximum : {}, hladane pismenko : {}, heslo : {}", min, max, pismenko, heslo);

        // let mut pocet = 0;

        // for i in heslo.chars() {
        //     // println!("{}", i);
        //     if i == pismenko {
        //         pocet += 1
        //     }
        

        // println!("{}", pocet);

        // if pocet >= min && pocet <= max {
        //     dobrych_hesiel += 1;
        // }
        // else {
        //     println!("Cislo NENI v range, min : {}, max : {}, actual count : {}", min, max, pocet);
        // }


        let rozsekane:Vec<char> = heslo.chars().collect();

        let spodok:usize = (min-1) as usize;
        let vrch:usize = (max-1) as usize;

        if rozsekane[spodok] == pismenko {
            if rozsekane[vrch] != pismenko {
                println!("spodok : {}, vrch : {}", spodok, vrch);
                println!("spodok : {}, vrch : {}, pismenko : {}, heslo : {}", rozsekane[spodok], rozsekane[vrch], pismenko, heslo);
                dobrych_hesiel += 1
            }
        };   
          
        if rozsekane[vrch] == pismenko {
            if rozsekane[spodok] != pismenko {
                println!("spodok : {}, vrch : {}", spodok, vrch);
                println!("spodok : {}, vrch : {}, pismenko : {}, heslo : {}", rozsekane[spodok], rozsekane[vrch], pismenko, heslo);
                dobrych_hesiel += 1
            }
        };    
    }
    

    println!("Dobrych hesiel : {}", dobrych_hesiel);

}
