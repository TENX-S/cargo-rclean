use std::fs;
use clap::{App, Arg};
use std::process::Command;
use anyhow::{Result, bail};
use std::path::{Path, PathBuf};
use walkdir::{WalkDir, DirEntry};
use ansi_term::ANSIGenericString;
use ansi_term::Colour::{Cyan, Red, Green};
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

static RELEASE: AtomicBool = AtomicBool::new(false);
static DOC: AtomicBool = AtomicBool::new(false);
static ALL: AtomicBool = AtomicBool::new(false);

fn main() {

    WalkDir::new({
        let args = App::new("cargo-rclean")
            .version("1.0.0")
            .author("Ares <coldswind@pm.me>")
            .about("Clean your rust project, recursively")
            .arg(Arg::with_name("INPUT")
                .help("Cleans up all rust projects in the specified directory")
                .value_name("PATH")
                .required(true)
                .index(1)
            )
            .arg(Arg::with_name("release")
                .help("Whether or not to clean release artifacts")
                .short("r")
                .long("release")
                .takes_value(false)
            )
            .arg(Arg::with_name("doc")
                .help("Whether or not to clean just the documentation directory")
                .short("d")
                .long("doc")
                .takes_value(false)
            )
            .arg(Arg::with_name("all")
                .help("Clean the rust projects no matter it has a target folder or not")
                .short("a")
                .long("all")
                .takes_value(false)
            )
            .get_matches();

        if args.is_present("release") {
            RELEASE.store(true, SeqCst)
        }

        if args.is_present("doc") {
            DOC.store(true, SeqCst)
        }

        if args.is_present("all") {
            ALL.store(true, SeqCst)
        }

        args.value_of("INPUT").unwrap().to_string()

    })
        .into_iter()
        .filter_entry(|e| strategy(e))
        .map(|p| -> Result<DirEntry> {
            if let Ok(entry) = p {
                Ok(entry)
            } else {
                eprintln!("{}: {}", ANSI_err(), Cyan.bold().paint(p.unwrap_err().to_string()));
                bail!("")
            }
        })
        .for_each(|x: Result<DirEntry>| {
            if x.is_ok() {
                let x = x.unwrap();
                let ret = fs::read_dir(x.path());
                if let Ok(read_dir) = ret {
                    read_dir
                        .map(|x| -> Result<PathBuf> {
                            if let Ok(entry) = x.as_ref() {
                                Ok(entry.path())
                            } else {
                                bail!("{}", Red.bold().paint(x.unwrap_err().to_string()));
                            }
                        })
                        .map(Result::unwrap)
                        .filter(|x| {
                            x.is_dir()
                        })
                        .filter(|x| {
                            x.join("Cargo.toml").exists()
                        })
                        .filter(|x| {
                            let detect_target = x.join("target").is_dir();
                            if ALL.load(SeqCst) {
                                !detect_target
                            } else {
                                detect_target
                            }
                        })
                        .for_each(|x| {
                            cargo_clean(x).unwrap_or_else(|e| eprintln!("{}! {}", ANSI_err(), e));
                        })
                }
            }
        });
}

#[inline(always)]
fn cargo_clean(path: impl AsRef<Path>) -> Result<()> {

    let mut args = vec!["clean"];

    if RELEASE.load(SeqCst) { args.push("--release"); }

    if DOC.load(SeqCst) { args.push("--doc"); }

    if Command::new("cargo")
        .args(&args)
        .current_dir(path.as_ref())
        .output()?.status.success() {
        println!("{} at {}", ANSI_ok(), path_display(path_to_str(path)));
    } else {
        println!("{}! In: {}", ANSI_err(), path_display(path_to_str(path)));
    }

    Ok(())

}

#[inline(always)]
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

#[inline(always)]
fn is_symlink(entry: &DirEntry) -> bool {
    entry.path_is_symlink()
}

#[inline(always)]
fn path_display<'a>(path: impl AsRef<str> + 'a) -> ANSIGenericString<'a, str> {
    Cyan.bold().underline().paint(path.as_ref().to_string())
}

#[inline(always)]
#[allow(non_snake_case)]
fn ANSI_ok() -> ANSIGenericString<'static, str> {
    Green.bold().paint("OK")
}

#[inline(always)]
#[allow(non_snake_case)]
fn ANSI_err() -> ANSIGenericString<'static, str> {
    Red.bold().paint("Error")
}

#[inline(always)]
fn path_to_str(path: impl AsRef<Path>) -> String {
    path.as_ref().to_str().unwrap().to_string()
}

#[inline(always)]
fn strategy(entry: &DirEntry) -> bool {
    if cfg!(target_os = "macos") {
        !is_symlink(entry) && !is_hidden(entry) && entry.path().is_dir() && !(
            if let Some(ext) = entry.path().extension() {
                ext == "app"
            } else {
                false
            }
        )
    } else {
        !is_symlink(entry) && !is_hidden(entry) && entry.path().is_dir()
    }
}
