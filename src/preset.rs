use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Preset {
    pub name: String,
    pub case: char,
    pub spec: String,
}

pub fn dir_opaque() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}

/*pub fn dir_clandestine() -> PathBuf {
    if let Ok(dir) = std::env::var("C:/save/data/qkeyrb") {
        PathBuf::from(dir)
    }
    else {
        dir::desktop_dir().unwrap_or_else(|| PathBuf::from(".")).join("save");
        continue;
    }
}*/

pub fn file() -> PathBuf {
    dir_opaque()
        .join("qkeyr_presets.md")
}

pub fn load() -> Vec<Preset> {
    let text = match fs::read_to_string(file()) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for line in text.lines() {
        let mut parts = line.splitn(3, '\t');
        let (Some(n), Some(f), Some(v)) = (parts.next(), parts.next(), parts.next())
            else {
            continue; // skip bad lines, avoid crash
        };
        let Some(sym) = f.chars().next() else { continue };
        if n.is_empty() { continue; }
        out.push(Preset { name: n.into(), case: sym, spec: v.into() });
    }
    out
}

pub fn find(list: &[Preset], name: &str) -> Option<usize> {
    list.iter().rposition(|p| p.name == name)
}

pub fn append(name: &str, case: char, spec: &str) -> Result<PathBuf, String> {
    if name.is_empty() || name.contains(char::is_whitespace) {
        return Err(" as on name use underscore! ".into());
    }
    let f = file();
    let line = format!("{name}\t\t{case}\t\t{spec}\n");
    let mut h = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&f)
        .map_err(|e| format!(" opening {} -> {e}", f.display()))?;
    h.write_all(line.as_bytes())
        .map_err(|e| format!(" writing {} -> {e}", f.display()))?;
    drop(h); // closes before copying... so the spare includes this line
    spare(&f)
}

fn spare(src: &Path) -> Result<PathBuf, String> {
    for n in 1..1000u32 {
        let p = dir_opaque().join(format!("qkeyr_preset_backup.{n:03}.tab"));
        if !p.exists() {
            fs::copy(src, &p).map_err(|e| format!("copy {} -> {e}", p.display()))?;
            return Ok(p);
        }
    }
    Err(" spares hit their max -- delete some manually to continue adding presets...".into())
}

pub fn elide(s: &str) -> String {
    let n = s.chars().count();
    if n <= 24 {
        return s.to_string();
    }
    let heads: String = s.chars().take(16).collect();
    let tails: String = s.chars().skip(n - 8).collect();
    format!("{heads}..{tails}")
}

pub fn presets_listing(list: &[Preset]) -> Vec<String> {
    list.iter()
        .map(|p| format!("{:<12} {} {}",
            p.name,
            p.case,
            elide(&p.spec))
        ).collect()
}
