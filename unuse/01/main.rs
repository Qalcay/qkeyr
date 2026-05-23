use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufWriter, Write};

struct Worker {
    qwer: Vec<char>, 
    //qwer: String, 
    word: String, 
    text: String, 
    hash: String, 
    cash: String, 
    auto: bool, 
    file: bool, 
}

impl Worker {
    fn new() -> Self {
        Self {
            qwer: ('A'..='Z').collect(),
            //qwer: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), 
            word: String::from("DEFAULT"), 
            text: String::from("THISISADEFAULTMESSAGE"), 
            hash: String::new(), 
            cash: String::new(), 
            auto: false, 
            file: true, 
        }
    }

    fn _nexter(&self, s: &mut Vec<usize>) -> bool {
        let n = self.qwer.len(); 
        for i in (0..s.len()).rev() {
            if s[i] + 1 < n {
                s[i] += 1;
                return true;
            }
            s[i] = 0;
        }
        false
    }

    fn _runner(&mut self, encry: bool) {
        let az = &self.qwer;
        let n = az.len(); 
        let char_to_indx = |c: char| az.iter().position(|&x| x == c);
        //let char_to_indx = |c: char| az.chars().position(|x| x == c);

        let k_indc: Vec<usize> = self.word.chars().filter_map(char_to_indx).collect();
        //let s_text: = if encry { &self.text } else { &self.hash };
        let x_indc: Vec<usize> = self.text.chars().filter_map(char_to_indx).collect();

        if k_indc.is_empty() || x_indc.is_empty() { return; }

        let mut r_indc = Vec::new();

        for i in 0..x_indc.len() {
            let p_indx = x_indc[i];

            // autokey on?
            let k_indx = if self.auto && i >= k_indc.len() {
                if encry {
                    x_indc[i - k_indc.len()] // plain?
                }
                else {
                    r_indc[i - k_indc.len()] // decry
                }
            }
            else {
                k_indc[i % k_indc.len()]
            };


            //let k_indx = k_indc[i % k_indx.len()];
            let f_indx = if encry { 
                (p_indx + k_indx) % n 
            } 
            else { 
                (p_indx + n - k_indx) % n // +n neg res is handled safely
            };
            r_indc.push(f_indx);
        }

        //let result: String = r_indc.iter().map(|&indx| az[indx]).collect();
        //if encry { self.hash = result; } else { self.text = result; }
        self.hash = r_indc.iter().map(|&indx| az[indx]).collect();
    }

    fn _runner2(&mut self, encry: bool) {
        let az = &self.qwer;
        let n = az.len(); 
        let char_to_indx = |c: char| az.iter().position(|&x| x == c);
        //let char_to_indx = |c: char| az.chars().position(|x| x == c);

        let k_indc: Vec<usize> = self.word.chars().filter_map(char_to_indx).collect();
        //let s_text: = if encry { &self.text } else { &self.hash };
        let x_indc: Vec<usize> = self.text.chars().filter_map(char_to_indx).collect();

        if k_indc.is_empty() || x_indc.is_empty() { return; }

        let mut r_indc = Vec::new();

        for i in 0..x_indc.len() {
            let p_indx = x_indc[i];

            // autokey on?
            let k_indx = if self.auto && i >= k_indc.len() {
                if encry {
                    x_indc[i - k_indc.len()] // plain?
                }
                else {
                    r_indc[i - k_indc.len()] // decry
                }
            }
            else {
                k_indc[i % k_indc.len()]
            };


            //let k_indx = k_indc[i % k_indx.len()];
            let f_indx = if encry { 
                (p_indx + k_indx) % n 
            } 
            else { 
                (p_indx + n - k_indx) % n // +n neg res is handled safely
            };
            r_indc.push(f_indx);
        }

        //let result: String = r_indc.iter().map(|&indx| az[indx]).collect();
        //if encry { self.hash = result; } else { self.text = result; }
        self.cash = r_indc.iter().map(|&indx| az[indx]).collect();
    }
}

fn main() {
    let mut w = Worker::new();

    show_application();
    show_greeting();
    show_commands();

    loop {
        print!("    >> ");
        io::stdout().flush().unwrap(); 
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 

        let read = input.trim();
        if read.is_empty() { continue; }

        if read == "~q!" {
            println!("");
            break;
        }
        else if read == "xc" {
            print!("10 xc press");
        }
        /*else if read == "x" {
            print!("5 x press");
        }*/
        /*else if {

            if {

            }
            else {

            }
        }*/

        let com_use = read.chars().next().unwrap();
        let ival = read[com_use.len_utf8()..].trim();

        match com_use {
            '.' => {
                let mut seer = HashSet::new();
                let mut new_az = Vec::new();
                for c in ival.to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()) {
                    if seer.insert(c) { new_az.push(c); }
                }
                for c in 'A'..='Z' { if seer.insert(c) { new_az.push(c); } }
                w.qwer = new_az;
            }
            '!' => w.word = ival.to_uppercase(), 
            '@' => {
                let mut seer = HashSet::new(); 
                //let mut new_az = Vec::new();
                //for c in ival.
                w.qwer = ival.to_uppercase().chars().filter(|&c| seer.insert(c)).collect();
            } 
            '&' => w.text = ival.to_uppercase(), 
            'q' => { w._runner(true); w._runner2(false); 
                println!("<qwer> {}", w.qwer.iter().collect::<String>());
                println!("<word> {}", w.word);
                println!("<auto> {}", w.auto);
                println!("<text> {}", w.text);
                println!("<hash> {}", w.hash);
                println!("<cash> {}", w.cash);
            }
            'w' => println!("<word> {}", w.word), 
            'e' => { w._runner(true); 
                println!("<hash> {}", w.hash);
            } 
            'r' => println!(""), 
            'a' => println!("<qwer> {}", w.qwer.iter().collect::<String>()), 
            's' => println!(""), 
            'd' => { w._runner2(false); 
                println!("<cash> {}", w.cash);
            } 
            'f' => { w.auto = !w.auto; println!("<auto> {}", w.auto) }
            'z' => { w._runner(true); w._runner2(false);
                println!("<hash> {}", w.hash);
                println!("<cash> {}", w.cash);
            }
            'x' => println!("<text> {}", w.text),
            'c' => { w._runner(true);
                println!("<text> {}", w.text); 
                println!("<hash> {}", w.hash); 
            }
            'v' => println!(""), 

            _ => println!(""),
        }
    }
}

//fn _brute_method



fn show_application() {
    print!("\n [QKEYR APPLICATION FOR MULTIPLE USE CASE CIPHERING]\n");
    print!("\t .. Program started . . .\n\n");
}

fn show_greeting() {
    print!("\t                                                                                  \n\n");
    print!("\t      -oOOOOOOo        -oO    -Oo     -oOOOOOo    -oO      -oO    -oOOOOOo        \n");
    print!("\t   -oO        -Oo      -O    -O       -Oo          -oO    -oO     -Oo     -O      \n");
    print!("\t  -oO          -Oo     -O  -O         -O            -oO   -O      -O      -O      \n");
    print!("\t  -O            -O     -OOOo          -OOOOOOo        -ooo        -oOOOOOo        \n");
    print!("\t  -oO          -Oo     -O  -Oo        -Oo              -O         -O  -Oo         \n");
    print!("\t   -oO     -sS Oo      -O    -O       -O               -O         -O    -O        \n");
    print!("\t     -oOOOOOo-sS       -oO    -Oo     -oOOOOOo        -oOo        -oO    -Oo      \n");
    print!("\t              -sS                                                                  \n");
    print!("\t   . . . . . .  -S-s  . . . . . . . . . . . . . . . . . . . . . . . . . . . .     \n\n\n");
}

fn show_commands() {
    print!("\t\n");
    print!("\t .. <commands are as follows> \n");
    print!("\t      . - - - - - - - - - .  \n");
    print!("\t     |   '~q!', '~h', '~p' | \n");
    print!("\t     |    '~v', '~g', '~l' | \n");
    print!("\t     |  '~', '!', '@', '&' | \n");
    print!("\t     |  'q', 'w', 'e', 'r' | \n");
    print!("\t     |  'a', 's', 'd', 'f' | \n");
    print!("\t     |  'z', 'x', 'c', 'v' | \n");
    print!("\t      ' - - - - - - - - - '  \n");
    print!("\t\n\n");
}