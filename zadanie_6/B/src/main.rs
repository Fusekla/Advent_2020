fn main() {
    let vstup_txt: Vec<_> = include_str!("input.txt").split("\r\n\r").collect();

    let mut vsetci_suhlasia = 0;

    for skupina in vstup_txt {
        let individualne:Vec<_> = skupina.split("\r").collect();
        let initial_pismenka:Vec<_> = individualne[0].trim().chars().collect();

            for kazde in initial_pismenka {
                let mut i = 0;
                let mut ciastkove = 0;
                while i < individualne.len() {
                    if individualne[i].contains(kazde) {
                        ciastkove += 1;
                    }
                    i += 1;
                }
                if ciastkove == individualne.len() {
                    vsetci_suhlasia +=1;
            }
        }
    }
    println!("Vsetci suhlasia : {}", vsetci_suhlasia);
}
