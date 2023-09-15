use colored::*;
use std::path::Path;

fn visit_dir(dir: &Path, level: usize, last_prefix: &str) -> Result<(), std::io::Error> {
    if level > 2 {
        return Ok(());
    }
    let entries: Vec<_> = std::fs::read_dir(dir)?.collect();
    let total = entries.len();
    for (i, entry) in entries.into_iter().enumerate() {
        let entry = entry?;
        let path = entry.path();
        let is_dir = path.is_dir();
        let is_last_item = i == total - 1;
        let name = path
            .to_str()
            .and_then(|s| s.rsplit_once('/'))
            .and_then(|(_, name)| Some(name))
            .unwrap();
        if name.starts_with(".") {
            continue;
        }
        let name = if is_dir {
            name.bold().white()
        } else {
            name.normal()
        };
        let prefix = if is_last_item { "└─ " } else { "├─ " }.truecolor(80, 80, 80);
        println!("{last_prefix}{prefix}{name}");
        let next_prefix = if is_last_item { "   " } else { "│  " }.truecolor(80, 80, 80);
        if path.is_dir() {
            visit_dir(&path, level + 1, &format!("{last_prefix}{next_prefix}"))?;
        }
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    println!(".");
    visit_dir(Path::new("."), 0, "")?;
    Ok(())
}
