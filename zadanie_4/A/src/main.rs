fn main() {
    let vstup_txt: Vec<_> = include_str!("input.txt").split("\n\r\n").collect();

    let mut pass_counter = 0;
    let mut valid_passports = 0;

    for i in vstup_txt {
        pass_counter += 1;
        println!("Passport #{}", pass_counter);
        let passport_details = i.chars();
        let mut pocet_dvojbodiek: i32 = 0;
        for j in passport_details {
            if j == ':' {
                pocet_dvojbodiek += 1;
            }
        }

        if pocet_dvojbodiek == 8 {
            valid_passports += 1;
        }
        if pocet_dvojbodiek == 7 {
            if i.contains("cid") {
                ()
            } else {
                valid_passports += 1;
            }
        }

        println! {"Passport {} ma {} dvojbodiek.", pass_counter, pocet_dvojbodiek};
    }

    println!("Mame {} valid passportov.", valid_passports);
}
