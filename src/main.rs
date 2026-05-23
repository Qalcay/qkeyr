use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufWriter, Write};

struct Worker {
    qwer: Vec<char>, 
    asdf: String, 
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
            asdf: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), 
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

    show_gap();
    //show_application();
    show_greeting();
    show_commands();
    //show_hint();

    loop {
        print!("    >> ");
        io::stdout().flush().unwrap(); 
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 

        let read = input.trim();
        if read.is_empty() { continue; }

        let com_use = read.chars().next().unwrap();
        let ival = read[com_use.len_utf8()..].trim();

        if read == "~q!" {
            println!("\t .. Program exiting . . .\n\n");
            break;
        }

        if read == "~h" {
            show_commands();
        }

        else if read == "xc" {
            println!("<qwer> {}", w.qwer.iter().collect::<String>());
            println!("<word> {}", w.word); 
            println!("<text> {}", w.text);
        }

        else if read.starts_with('.') {
            let mut seer = HashSet::new();
            let mut new_az = Vec::new();
            for c in ival.to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()) {
                if seer.insert(c) { new_az.push(c); }
            }
            for c in 'A'..='Z' { if seer.insert(c) { new_az.push(c); } }
            w.qwer = new_az;
        }
        else if read.starts_with('!') {
            w.word = ival.to_uppercase();
        }
        else if read.starts_with('@') {
            let mut seer = HashSet::new(); 
            w.qwer = ival.to_uppercase().chars().filter(|&c| seer.insert(c)).collect();
        }
        else if read.starts_with('&') {
            w.text = ival.to_uppercase(); 
        }

        else if read.starts_with('q') {
            w._runner(true); w._runner2(false); 
            println!("<qwer> @ {}", w.qwer.iter().collect::<String>());
            println!("<word> ! {}", w.word);
            println!("<auto> - {}", w.auto);
            println!("<text> & {}", w.text);
            println!("<hash> # {}", w.hash);
            println!("<cash> $ {}", w.cash);
        }
        else if read.starts_with('w') {
            println!("<word> ! {}", w.word);
        }
        else if read.starts_with('e') {
            println!("<hash> # {}", w.hash);
        }
        else if read.starts_with('r') {
            println!("");
        }
        else if read.starts_with('a') {
            println!("<qwer> @ {}", w.qwer.iter().collect::<String>());
        }
        else if read.starts_with('s') {
            println!("");
        }
        else if read.starts_with('d') {
            println!("<cash> $ {}", w.cash);
        }
        else if read.starts_with('f') {
            w.auto = !w.auto; 
            println!("<auto> - {}", w.auto);
        }
        else if read.starts_with('z') {
            w._runner(true); w._runner2(false);
            println!("<hash> # {}", w.hash);
            println!("<cash> $ {}", w.cash);
        }
        else if read.starts_with('x') {
            println!("<text> & {}", w.text);
        }
        else if read.starts_with('c') {
            println!("<text> & {}", w.text); 
            println!("<hash> # {}", w.hash); 
        }
        else if read.starts_with('v') {
            println!("");
        }
        /*else if {

            if {

            }
            else {

            }
        }*/
        else {
            print!("");
        }




    }
}

//fn _brute_method



fn show_application() {
    print!("\n [QKEYR APPLICATION FOR MULTIPLE USE CASE CIPHERING]\n");
    print!("\t .. Program started . . .\n\n\n");
}

fn show_gap() {
    print!("\t\n");
}

fn show_greeting() {
    print!("\t. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .   \n");
    print!("\t.                                                                               .   \n");
    print!("\t.      ~XXXXXXXX        ~XX    ~XX     ~XXXXXXX    ~XX      ~XX    ~XXXXXXX     .   \n");
    print!("\t.   ~XX        ~XX      ~X    ~X       ~X           ~XX     ~X     ~X     ~X    .   \n");
    print!("\t.  ~XX          ~XX     ~X  ~X         ~X            ~XX   ~X      ~X     ~X    .   \n");
    print!("\t.  ~X            ~X     ~XXXX          ~XXXXXXX        ~XXX        ~XXXXXXX     .   \n");
    print!("\t.  ~XX          ~XX     ~X  ~XX        ~X               ~X         ~X   ~X      .   \n");
    print!("\t.   ~XX     ~SS XX      ~X    ~X       ~X               ~X         ~X    ~X     .   \n");
    print!("\t.     ~XXXXXXX~SS       ~XX    ~XX     ~XXXXXXX        ~XXX        ~XX    ~XX   .   \n");
    print!("\t.              ~SS                                                              .   \n");
    print!("\t. . . . . . . .  ~S~S  . . . . . . . . . . . . . . . . . . . . . . . . . . . .  .   \n\n\n");
}

fn show_hint() {
    print!("\t .. type '~h' for more information . . . \n\n\n\n");
}

fn show_commands() {
    //print!("\t\n");
    //print!("\t .. Commands are as follows: \n");
    print!("\t\n");
    print!("\t i.\t '~q!', '~h', '~p'"); print!("\t -\tquit, help, print?\n");
    print!("\t ii.\t  '~v', '~g', '~l'"); print!("\t -\tversion, g?, l?\n");
    print!("\t iii.\t'~', '!', '@', '&'"); print!("\t -\tcommand key, set key, set a-z, set text\n");
    print!("\t iv.\t'.', ':', '^', '*'"); print!("\t - \tset keyed a-z, presets, reverse, mod(n)\n");
    print!("\t v.\t'q', 'w', 'e', 'r'"); print!("\t -\tquery, word, encrypt, r?\n");
    print!("\t vi.\t'a', 's', 'd', 'f'"); print!("\t -\talphabet, sha?, decrypt, autokey on/off\n");
    print!("\t vii.\t'z', 'x', 'c', 'v'"); print!("\t -\tencrypt+decrypt, text, cipher, ?\n");
    print!("\t\n");
    print!("\t\n");
}