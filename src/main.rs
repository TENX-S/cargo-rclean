use walkdir::{WalkDir, DirEntry};
use anyhow::Result;
use rayon::prelude::*;
use std::fs;
use std::env;
use std::process::Command;
use std::path::Path;
use ansi_term::Color::{Red, Green};
use ansi_term::Colour::Cyan;

fn main() {
    WalkDir::new(env::args().skip(1).last().unwrap())
        .into_iter()
        .filter_entry(|e| strategy(e))
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(|x: DirEntry|
            fs::read_dir(x.path())
                .unwrap()
                .map(|x| x.as_ref().unwrap().path())
                .filter(|x| {
                    x.is_dir() && x.join("Cargo.toml").exists() && x.join("target").is_dir()
                })
                .for_each(|x| {
                    cargo_clean(x).unwrap();
                })
        );
}

#[inline(always)]
fn cargo_clean(path: impl AsRef<Path>) -> Result<()> {

    if Command::new("cargo")
        .args(&["clean"])
        .current_dir(path.as_ref())
        .output()?.status.success() {
        println!("{} at {}", Green.paint("OK"), Cyan.bold().underline().paint(path_to_str(path)));
    } else {
        println!("{}! In: {}", Red.paint("Error"), Cyan.bold().underline().paint(path_to_str(path)));
    }

    Ok(())
}

#[inline(always)]
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

#[inline(always)]
fn path_to_str(path: impl AsRef<Path>) -> String {
    path.as_ref().to_str().unwrap().to_string()
}

#[inline]
fn strategy(entry: &DirEntry) -> bool {
    if cfg!(target_os = "macos") {
        !is_hidden(entry) && entry.path().is_dir() &&
            !(
                if let Some(ext) = entry.path().extension() {
                    ext == "app"
                } else {
                    false
                }
            )
    } else {
        !is_hidden(entry) && entry.path().is_dir()
    }
}
