mod preset;

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::thread::sleep;
use std::time::Duration;

const VERSION: &str = " Aug/2026 ''Elate Dawn'' ";
const ATOZ_STRING: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Clone, Copy)]
enum Mode { Vig, Beau, Var }

impl Mode {
    fn tag(self) -> &'static str {
        match self {
            Mode::Vig   => "+ -",       //"Vigenere",
            Mode::Beau  => "- -",       //"Beaufort",
            Mode::Var   => "- +",       //"Variant Beaufort"
            //Mode::Plus => "+ +"
            //Mode::Rot13 => "+13 -13"
        }
    }
    fn next(self) -> Mode {
        match self {
            Mode::Vig   => Mode::Beau,
            //Mode::Vig   => Mode::Var,
            Mode::Beau  => Mode::Var,
            Mode::Var   => Mode::Vig
        }
    }
}

struct Worker {
    atoz: Vec<char>,    // was 'qwer'
    //asdf: String,       // put alpha into string?
    word: String,       // keyword
    text: String,
    hash: String,
    cash: String,
    auto: bool,
    mode: Mode,
    file: bool,
}

impl Worker {
    fn new() -> Self {
        Self {
            atoz: ('A'..='Z').collect(),
            //asdf: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            word: "DEFAULT".into(),
            text: "THISISADEFAULTMESSAGE".into(),
            hash: String::new(),
            cash: String::new(),
            auto: false,
            mode: Mode::Vig,
            file: true,
        }
    }

    fn atoz_str(&self) -> String { self.atoz.iter().collect() }

    fn state_holders(&self, sym: char) -> Option<String> {
        match sym {
            '&' => Some(self.text.clone()),
            '!' => Some(self.word.clone()),
            '@' => Some(self.atoz_str()),
            '#' => Some(self.hash.clone()),
            '$' => Some(self.cash.clone()),
            _ => None,
        }
    }

    fn state_pushed(&mut self, sym: char, v: &str) -> bool {
        match sym {
            '&' => { self.text = clean(v); true }
            '!' => { self.word = clean(v); true }
            /*'@' => {
                let mut s = HashSet::new();
                self.atoz = clean(v).chars().filter(|&c| s.insert(c)).collect();
                true
            }*/
            _ => false,
        }
    }

    fn pass(&self, source: &str, encrypt: bool) -> String {
        let n       = self.atoz.len();
        let index   = |c: char| self.atoz.iter().position(|&x| x == c);
        let k: Vec<usize> = self.word.chars().filter_map(index).collect();
        let x: Vec<usize> = source.chars().filter_map(index).collect();
        if k.is_empty() || x.is_empty() { return String::new(); }

        let mut out: Vec<usize> = Vec::with_capacity(x.len());
        for i in 0..x.len() {
            let p = x[i];
            let k_i = if self.auto && i >= k.len() {
                if encrypt { x[i - k.len()] }
                else { out[i - k.len()] }
            } else {
                k[i % k.len()]
            };
            out.push(match (self.mode, encrypt) {
                (Mode::Vig, true)   => (p + k_i) % n,
                (Mode::Vig, false)  => (p + n - k_i) % n,
                (Mode::Beau, _)     => (k_i + n - p) % n,
                (Mode::Var, true)   => (p + n - k_i) % n,
                (Mode::Var, false)  => (p + k_i) % n,
            });
        }
        out.iter().map(|&i| self.atoz[i]).collect()
    }

    fn tabula_recta(&self, draw: &str) {
        let chars: Vec<char> = draw.chars().collect();
        let d = chars.len();

        for shift in 0..d {
            let row: String = (0..d)
                .map(|i| chars[(i + shift) % d].to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", row);
            gently_print();
        }
    }

    fn both(&mut self) {
        let h = self.pass(&self.text, true);
        let c = self.pass(&self.text, false);
        self.hash = h;
        self.cash = c;
    }

    fn full_state(&self) {
        println!("<a..z> @ {}", self.atoz_str());
        println!("<word> ! {}", self.word);
        //println!("<mode> ~ {}", self.mode.tag());
        //println!("<auto> - {}", self.auto);
        println!("<text> & {}", self.text);
        println!("<hash> # {}", self.hash);
        println!("<cash> $ {}", self.cash);
    }
    /*fn full_state(&self) {
        println!("<a..z> @ {}", self.atoz_str());
        gently_print();
        println!("<word> ! {}", self.word);
        gently_print();
        //println!("<mode> ~ {}", self.mode.tag());
        //println!("<auto> - {}", self.auto);
        println!("<text> & {}", self.text);
        gently_print();
        println!("<hash> # {}", self.hash);
        gently_print();
        println!("<cash> $ {}", self.cash);
    }*/

    fn caeser_salad(&self, feed: &str, alpha: &[char]) {
        for s in 0..26u8 {
            let dish: String = feed.chars()
                .map(|c| {
                    if let Some(pos) = alpha.iter().position(|&x| x == c) {
                        let new_pos = (pos + s as usize) % 26;
                        alpha[new_pos]
                    } else {
                        c
                    }
                }).collect();
            println!("<..{s:02}> % {dish}");
            gently_print();
        }
    }

    /*fn old_caeser_salad(&self, feed: &str) {
        for s in 0..26u8 {
            let dish: String = feed.chars()
                .map(|c| if c.is_ascii_uppercase() {
                    (b'A' + (c as u8 - b'A' + s) % 26) as char
                } else { c }).collect();
            println!("<..{s:02}> % {dish}");
            gently_print();
        }
    }*/

    fn atbash(&self, s: &str) -> String {
        let n = self.atoz.len();
        s.chars().map(|c| match self.atoz.iter().position(|&x| x == c) {
            Some(i) => self.atoz[n - 1 - i],
            None => c,
        }).collect()
    }
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn keyed_alphabet(seed: &str) -> Vec<char> {
    let mut seen    = HashSet::new();
    let mut out     = Vec::new();
    for c in seed.to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()) {
        if seen.insert(c) { out.push(c); }
    }
    for c in 'A'..='Z' {
        if seen.insert(c) {
            out.push(c);
        }
    }
    out
}

fn clean(s: &str) -> String {
    s.to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()).collect()
}

fn main() {
    let mut w = Worker::new();
    w.both();

    let sin = io::stdin();
    let mut lines = sin.lock().lines();

    //println!("")
    show_gap();
    //show_application();
    show_header();
    //show_guide();
    show_hint();

    loop {
        let atoz_vec: Vec<char> = ('A'..='Z').collect();
        print!("    >> ");
        io::stdout().flush().unwrap();
        //let mut input = String::new();
        //io::stdin().read_line(&mut input).unwrap();

        let line = match lines.next() {
            Some(Ok(l)) => l,
            _ => break,
        };

        let r = line.trim();
        if r.is_empty() { continue; }
        let bye = "HPPECZF!";

        match r {
            "_q!" | "_Q!"   =>     {
                w.caeser_salad(bye, &atoz_vec);
                paused_exits();
                break; }
            //"_q!"   =>     { println!("<....> ~ bye bye!\n"); break; }
            "_h" | "_H"    =>      { show_new_guide(); continue; }
            "_v" | "_V"  =>      { println!("{VERSION}"); continue; }
            "_:" | "_;" => {
                let ps = preset::load();
                if ps.is_empty() { println!("<void> ? there are no presets"); }
                for l in preset::presets_listing(&ps) { println!("{l}"); }
                continue;
            }
            _ => {}
        }

        let mut handle = r.chars();
        let sig = handle.next().unwrap();
        let arg = handle.as_str().trim();

        let mut called = true;
        let mut served = true;
        //let mut t: Vec<char>;


        match sig {
            '.' => w.atoz = keyed_alphabet(arg),
            '!' => w.word = clean(arg),
            '&' => w.text = clean(arg),
            '@' => {
                let mut s = HashSet::new();
                w.atoz = clean(arg)
                    .chars()
                    .filter(|&c| s.insert(c))
                    .collect();
            },
            '^' => match arg {
                "!" => w.word = reverse(&w.word),
                "@" => w.atoz.reverse(),
                "&" => w.text = reverse(&w.text),
                s => {
                    println!("<swap> ^{}", reverse(s));
                    served = false;
                }
            },
            '`' => match arg {
                "@" => w.atoz.reverse(),
                "!" => { let t = w.atbash(&w.word); w.word = t; }
                "&" => { let t = w.atbash(&w.text); w.text = t; }
                s => {
                    //let mut t = w.atbash(&w.atoz_str()); s = t;
                    let sc = s;
                    w.atbash(&sc);
                    println!("<swap> `{}", w.atbash(&sc));
                    served = false;
                }
            },
            ';' => {
                let mut a = arg.split_whitespace();
                match (a.next(), a.next().and_then(|f| f.chars().next())) {
                    (Some(name), Some(sym)) => match w.state_holders(sym) {
                        Some(v) => match preset::append(name, sym, &v) {
                            Ok(tab) => println!("<save> + {name} {sym} -> {}",
                                tab.file_name().unwrap_or_default().to_string_lossy()),
                            Err(e) => println!("<eror> ? {e}"),
                        },
                        None => println!("<eror> ? {sym} unrecognizable"),
                    },
                    _ => {
                        println!("<help> - ';hello hello' or ';asdf #'");
                        println!("<help> - ':hello' or ':asdf !'");
                    }
                }
                served = false;
            }
            ':' => {
                let mut a = arg.split_whitespace();
                match a.next() {
                    Some(name) => {
                        let ps = preset::load();
                        match preset::find(&ps, name) {
                            Some(i) => {
                                let sym = a.next().and_then(|s| s.chars().next())
                                    .unwrap_or(ps[i].case);
                                if w.state_pushed(sym, &ps[i].spec) {
                                    println!("<load> - '{name}' -> '{sym}'");
                                } else {
                                    println!("<eror> ? '{sym}' unavailable");
                                    served = false;
                                }
                            }
                            None => {
                                println!("<eror> ? cannot find '{name}'");
                                served = false;
                            }
                        }
                    } None => {
                        println!("<help> - ':name {}'", w.text);
                        served = false;
                    }
                }
            }
            _ => {
                called = false;
                served = false;
            }
        }

        if called {
            if served {
                w.both();
            }
            continue;
        }

        if r.len() != 1 { println!("<void> ? ..."); continue; }
        match r {
            "q" | "Q" => { w.full_state(); }
            "z" | "Z" => {
                println!("<hash> # {}", w.hash);
                println!("<cash> $ {}", w.cash);
            }
            "w" | "W" => println!("<word> ! {}", w.word),
            "a" | "A" => println!("<a..z> @ {}", w.atoz_str()),
            "x" | "X" => println!("<text> & {}", w.text),
            "e" | "E" => println!("<hash> # {}", w.hash),
            "d" | "D" => println!("<cash> $ {}", w.cash),
            "f" | "F" => {
                w.auto = !w.auto;
                w.both();
                println!("<auto> - {}", w.auto);
            }
            "m" | "M" => {
                println!("<mode> ~ {}", w.mode.tag());
                println!("<auto> - {}", w.auto);
            }
            "v" | "V" => {
                w.mode = w.mode.next();
                w.both();
                println!("<mode> ~ {}", w.mode.tag());
            }
            /*"b" | "B" => {
                w.mode = w.mode.beau();
                w.both();
                println!("<mode> ~ {}", w.mode.tag());
            }*/
            "t" | "T" => w.tabula_recta(&w.atoz_str()),
            "y" | "Y" => w.caeser_salad(&w.cash, &w.atoz),
            "u" | "U" => w.caeser_salad(&w.hash, &w.atoz),
            "i" | "I" => w.caeser_salad(&w.text, &w.atoz),
            "o" | "O" => w.caeser_salad(&w.atoz_str(), &w.atoz),
            "p" | "P" => w.caeser_salad(&w.word, &w.atoz),
            _ => println!("<void> ? ..."),

        }
    }
}

//fn _brute_method

fn paused_exits() {
    sleep(Duration::from_millis(2222));
}

fn gently_print() {
    sleep(Duration::from_micros(2222));
}

fn show_application() {
    print!("\n [QKEYR APPLICATION FOR MULTIPLE USE CASE CIPHERING]\n");
    print!("\t .. Program started . . .\n\n\n");
}

fn show_gap() {
    print!("\t\n");
}

fn show_new_header() {
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

fn show_header() {
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
    print!("\t .. type '_h' for more information . . . \n\n\n\n");
}

fn show_guide() {
    //print!("\t\n");
    //print!("\t .. Commands are as follows: \n");
    print!("\t\n");
    print!("\t i.\t _q! _h _p"); print!("\t_quit _help _print\n");
    print!("\t ii.\t _v _g _l"); print!("\t_version g l\n");
    print!("\t iii.\t ~ ! @ &"); print!("\t_command key _set key _set a-z _set text\n");
    print!("\t iv.\t . : ^ *"); print!("\t_set keyed a-z _preset _reverse _mod(n)\n");
    print!("\t v.\tq w e r"); print!("\t\t_query _word _encrypt r\n");
    print!("\t vi.\t a s d f"); print!("\t_alphabet _sha _decrypt _autokey\n");
    print!("\t vii.\t  z x c v"); print!("\t_e+d _text _cipher _vary\n");
    print!("\t\n");
    print!("\t\n");
}

fn show_new_guide() {
    //print!("\t\n");
    //print!("\t .. Commands are as follows: \n");
    print!("\t
        _q! | quit
         _h | help
         _p | _print ");
    print!("\t
         _v | version
         _g |
         _l | ");
    print!("\t
          ~ | command key
          ! | set key
          @ | set a-z
          & | set text ");
    print!("\t
          . | set keyed a-z
          : | preset
          ^ | reverse
          % | mod(n) ");
    print!("\t
          q | query
          w | word
          e | encrypt
          r | ");
    print!("\t
          a | alphabet
          s | sha
          d | decrypt
          f | autokey ");
    print!("\t
          z | e and d
          x | text
          c | cipher
          v | mode ");
    print!("\t\n\n");
}
