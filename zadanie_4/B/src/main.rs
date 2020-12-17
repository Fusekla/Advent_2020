struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: u64,
}

fn main() {
    let vstup_txt: Vec<_> = include_str!("sample.txt").split("\n\r\n").collect();

    for member in vstup_txt{
        println!("HOLVAN A KIBASZOTT HUTOTASKA??!?!");
        let aladar: Vec<&str> = member.split(|x| (x == ' ') || (x == '\n') ).collect();
        
        let mut exists_counter = 0;  
        let mut byr:u32 = 0;   
        let mut iyr:u32 = 0;  
        let mut eyr:u32 = 0; 
        let mut hgt:String = "Nincsen".to_string();
        let mut hcl:String = "Nincsen".to_string();
        let mut ecl:String = "Nincsen".to_string();
        let mut pid:u32 = 0;
        
        for i in aladar {

            // let mut byr:u32 = 0;
            if i.starts_with("byr") {
                let byr_in:String = i.split("byr:").collect();
                let byr_num:u32 = byr_in.trim().parse::<u32>().unwrap();
                byr = byr_num;
                exists_counter += 1;
                continue
            }

            // let mut iyr:u32 = 0;
            if i.starts_with("iyr") {
                let iyr_in:String = i.split("iyr:").collect();
                let iyr_num:u32 = iyr_in.trim().parse::<u32>().unwrap();
                iyr = iyr_num;
                exists_counter += 1;
                continue
            }

            // let mut eyr:u32 = 0;
            if i.starts_with("eyr") {
                let eyr_in:String = i.split("eyr:").collect();
                let eyr_num:u32 = eyr_in.trim().parse::<u32>().unwrap();
                eyr = eyr_num;
                exists_counter += 1;
                continue
            }

            // let mut hgt:String = "Nincsen".to_string();
            if i.starts_with("hgt") {
                hgt = i.split("hgt:").collect();
                exists_counter += 1;
                continue
            }

            // let mut hcl:String = "Nincsen".to_string();
            if i.starts_with("hcl") {
                hcl = i.split("hcl:").collect();
                exists_counter += 1;
                continue
            }

            // let mut ecl:String = "Nincsen".to_string();
            if i.starts_with("ecl") {
                ecl = i.split("ecl:").collect();
                exists_counter += 1;
                continue
            }

            // let mut pid:u32 = 0;
            if i.starts_with("pid") {
                let pid_in:String = i.split("pid:").collect();
                let pid_num:u32 = pid_in.trim().parse::<u32>().unwrap();
                pid = pid_num;
                exists_counter += 1;
                continue
            }
        }

        println!("mame byr : {}", byr);
        println!("mame iyr : {}", iyr);
        println!("mame eyr : {}", eyr);
        println!("mame hgt : {}", hgt);
        println!("mame hcl : {}", hcl);
        println!("mame ecl : {}", ecl);
        println!("mame pid : {}", pid);
        println!("existing fields counter : {}", exists_counter);

    }

}
