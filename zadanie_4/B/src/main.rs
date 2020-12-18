struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

fn main() {
    let vstup_txt: Vec<_> = include_str!("input.txt").split("\r\n\r\n").collect();

    let mut valid_passports = 0;
    let mut traktor = 0;

    println!("pocet pasov : {}", vstup_txt.len());

    for member in vstup_txt {
        traktor += 1;
        // println!("member : {}", member.trim());

        // println!("HOLVAN A KIBASZOTT HUTOTASKA??!?!");
        let aladar: Vec<&str> = member.trim().split_whitespace().collect();
        // println!("aladar : {:?}", aladar);

        let mut exists_counter = 0;
        let mut byr_found: u32 = 0;
        let mut iyr_found: u32 = 0;
        let mut eyr_found: u32 = 0;
        let mut hgt_found: String = "Nincsen".to_string();
        let mut hcl_found: String = "Nincsen".to_string();
        let mut ecl_found: String = "Nincsen".to_string();
        let mut pid_found: String = "Nincsen".to_string();

        for i in aladar {
            // println!("{}",i);

            // let mut byr:u32 = 0;
            if i.starts_with("byr") {
                let byr_in: String = i.split("byr:").collect();
                byr_found = byr_in.trim().parse::<u32>().unwrap_or(0);
                exists_counter += 1;
                continue;
            }

            // let mut iyr:u32 = 0;
            if i.starts_with("iyr") {
                let iyr_in: String = i.split("iyr:").collect();
                iyr_found = iyr_in.trim().parse::<u32>().unwrap_or(0);
                exists_counter += 1;
                continue;
            }

            // let mut eyr:u32 = 0;
            if i.starts_with("eyr") {
                let eyr_in: String = i.split("eyr:").collect();
                eyr_found = eyr_in.trim().parse::<u32>().unwrap_or(0);
                exists_counter += 1;
                continue;
            }

            // let mut hgt:String = "Nincsen".to_string();
            if i.starts_with("hgt") {
                hgt_found = i.split("hgt:").collect();
                exists_counter += 1;
                continue;
            }

            // let mut hcl:String = "Nincsen".to_string();
            if i.starts_with("hcl") {
                hcl_found = i.split("hcl:").collect();
                exists_counter += 1;
                continue;
            }

            // let mut ecl:String = "Nincsen".to_string();
            if i.starts_with("ecl") {
                ecl_found = i.split("ecl:").collect();
                exists_counter += 1;
                continue;
            }

            // let mut pid:u32 = 0;
            if i.starts_with("pid") {
                pid_found = i.split("pid:").collect();
                // pid_found = pid_in.trim().parse::<u64>().unwrap_or(0);
                exists_counter += 1;
                continue;
            }
        }

        if exists_counter == 7 {
            let passport = Passport {
                byr: byr_found,
                iyr: iyr_found,
                eyr: eyr_found,
                hgt: hgt_found,
                hcl: hcl_found,
                ecl: ecl_found,
                pid: pid_found,
            };
            let mut bandurka = 0;
            // println!("byr : {}, iyr : {}, eyr : {}, ecl : {}", passport.byr, passport.iyr, passport.eyr, passport.ecl);
            if passport.byr >= 1920 && passport.byr <= 2002 {
                if passport.iyr >= 2010 && passport.iyr <= 2020 {
                    if passport.eyr >= 2020 && passport.eyr <= 2030 {
                        match passport.ecl.as_str().trim() {
                            "amb" => bandurka += 1,
                            "blu" => bandurka += 1,
                            "brn" => bandurka += 1,
                            "gry" => bandurka += 1,
                            "grn" => bandurka += 1,
                            "hzl" => bandurka += 1,
                            "oth" => bandurka += 1,
                            _ => (),
                        }
                        if passport.hgt.contains("cm") && passport.hgt.trim().len() == 5 {
                            let vyska: u32 = passport.hgt[..3].parse().unwrap_or(0);
                            // println!("Vyska v cm : {}", vyska);
                            if vyska >= 150 && vyska <= 193 {
                                bandurka += 1;
                            } else {
                                // println!("Neplatna vyska v cm : {}", vyska);
                            }
                        }
                        if passport.hgt.contains("in") && passport.hgt.trim().len() == 4 {
                            let vyska: u32 = passport.hgt[..2].parse().unwrap_or(0);
                            if vyska >= 59 && vyska <= 76 {
                                bandurka += 1;
                            } else {
                                // println!("Neplatna vyska v in : {}", vyska);
                            }
                        }
                        if passport.hcl.starts_with('#') && passport.hcl.trim().len() == 7 {
                            bandurka += 1;
                        } else {
                            println!("Neplatne haro : {}", passport.hcl);
                        }

                        if passport.pid.trim().len() == 9 {
                            println!("Platne pid : {}", passport.pid);
                            bandurka += 1;
                        } else {
                            // println!("Neplatne pid : {}", passport.pid);
                        }
                        if bandurka == 4 {
                            valid_passports += 1;
                        }
                    } else {
                        // println!("Neplatne eyr : {}", passport.eyr);
                    }
                } else {
                    // println!("Neplatne iyr : {}", passport.iyr);
                }
            } else {
                // println!("Neplatny byr : {}", passport.byr);
            }
            // println!("Bandurkoch pre pas {} je {}", passport.pid, bandurka);
        }
    }
    println!("Traktoroch mame {}", traktor);
    println!("Platnych pasov je {}", valid_passports);
}
