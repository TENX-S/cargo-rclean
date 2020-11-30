use std::fs;
use anyhow::Result;
use std::path::Path;
use clap::{App, Arg};
use rayon::prelude::*;
use std::process::Command;
use walkdir::{WalkDir, DirEntry, Error};
use ansi_term::Colour::{Cyan, Red, Green,};
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

static RELEASE: AtomicBool = AtomicBool::new(false);
static DOC: AtomicBool = AtomicBool::new(false);

fn main() {

    WalkDir::new({
        let app = App::new("cargo-rclean")
            .version("1.0.0")
            .author("Ares.T <coldswind@pm.me>")
            .about("Clean your rust project, recursively")
            .arg(Arg::with_name("INPUT")
                .value_name("PATH")
                .required(true)
                .index(1))
            .arg(Arg::with_name("release")
                .short("r")
                .long("release")
                .takes_value(false)
            )
            .arg(Arg::with_name("doc")
                .short("d")
                .long("doc")
                .takes_value(false)
            ).get_matches();

        if app.is_present("release") {
            RELEASE.store(true, SeqCst)
        }

        if app.is_present("doc") {
            DOC.store(true, SeqCst)
        }

        app.value_of("INPUT").unwrap().to_string()
    })
        .into_iter()
        .filter_entry(|e| strategy(e))
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(|x: Result<DirEntry, Error>|
            fs::read_dir(x.unwrap().path())
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

    let mut args = vec!["clean"];

    if RELEASE.load(SeqCst) {
        args.push("--release");
    }

    if DOC.load(SeqCst) {
        args.push("--doc");
    }

    if Command::new("cargo")
        .args(&args)
        .current_dir(path.as_ref())
        .output()?.status.success() {
        println!("{} at {}", Green.bold().paint("OK"), Cyan.bold().underline().paint(path_to_str(path)));
    } else {
        println!("{}! In: {}", Red.bold().paint("Error"), Cyan.bold().underline().paint(path_to_str(path)));
    }

    Ok(())
}

#[inline(always)]
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
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
        !is_hidden(entry) && entry.path().is_dir() && !(
            if let Some(ext) = entry.path().extension() {
                ext == "app" || ext == "rtfd"
            } else {
                false
            }
        )
    } else {
        !is_hidden(entry) && entry.path().is_dir()
    }
}
