use std::{fs, io, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    from: String,
    to: String,
    dir: PathBuf,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    process_dir(cli.dir, &cli.from, &cli.to)?;

    Ok(())
}

fn process_dir(dir: PathBuf, from: &str, to: &str) -> io::Result<()> {

    let mut git_exists = false;
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.is_dir() {
            process_dir(entry.path(), from, to)?;
        }

        if entry.file_name() == ".git" && meta.is_dir() {
            git_exists = true;
            break;
        }
    }

    if !git_exists {
        return Ok(());
    }

    println!("processing {}", dir.display());

    let out = std::process::Command::new("git")
        .args(["remote", "-v"])
        .current_dir(&dir)
        .output()?;

    if !out.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&out.stdout);

    if stdout.is_empty() {
        return Ok(());
    }

    let Some(url) = stdout.lines().next().unwrap().split(' ').next() else {
        eprintln!("failed to get remote url");
        return Ok(());
    };

    let url = url.split_once('\t').unwrap().1;

    url.strip_prefix("https://github.com/")
        .and_then(|url| {
            url.strip_suffix(".git")
        })
        .map(|url| {
            let (old_name, repo_name) = url.split_once('/').unwrap();
            if old_name == from {
                let new_url = format!("https://github.com/{}/{}.git", to, repo_name);
                std::process::Command::new("git")
                    .args(["remote", "set-url", "origin", &new_url])
                    .current_dir(&dir)
                    .output().unwrap();
            }

            Some(())
        });

    Ok(())
}

