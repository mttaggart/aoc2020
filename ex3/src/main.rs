use std::fs;
use std::env;

struct Password {
    min: usize,
    max: usize,
    check: char,
    pw: String
}

impl Password {
    pub fn new(s : &str) -> Password {
        let mut sections = s
            .split(" ");
        let min_max : Vec<usize> = sections.nth(0)
            .unwrap()
            .split("-")
            .map(|m| m.parse::<usize>().unwrap_or_default())
            .collect();

        let check = sections.nth(0)
            .unwrap()
            .chars()
            .nth(0)
            .unwrap();

        let pw = String::from(sections.nth(0).unwrap());
        println!("{:?}: {} {}", min_max, check, pw);

        Password {min: min_max[0], max: min_max[1], check: check, pw: pw}
    } 

    fn is_valid(&self) -> bool {
        let check_count = self.pw
            .chars()
            .filter(|&c| c == self.check)
            .count();
        check_count >= self.min && check_count <= self.max
    }
}


fn main() {
    
    // Read in file  *
    // Return product of terms
    // Build Policies
    let filename = env::args().nth(1).unwrap();
    let file_string = String::from_utf8(fs::read(filename).unwrap())
        .unwrap();
    let passwords = file_string
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .map(|p| Password::new(p));

  
    let valid : usize = passwords.filter(|p| p.is_valid()).count();
    println!("Valid password: {}", valid);

}
