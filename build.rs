use ignore::Walk;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::Command,
};
use zip::{ZipWriter, write::SimpleFileOptions};

fn watch_homeworks() {
    for entry in Walk::new("homeworks").flatten() {
        let path = entry.path();
        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src/syllabus.typ");

    // We can't just use "cargo:rerun-if-changed=homeworks" because we'd be
    // recursively rebuilding over and over due to Cargo.lock and handin.zip
    watch_homeworks();

    // Create public dir if it doesn't exist
    std::fs::create_dir_all("public").ok();

    let status = Command::new("typst")
        .args(["compile", "src/syllabus.typ", "public/syllabus.pdf"])
        .status()
        .expect("failed to run typst");

    if !status.success() {
        panic!("typst compilation failed");
    }

    // Build homework handouts
    let homeworks = [
        ("homeworks/week1/primerlab", "primerlab"),
        ("homeworks/week2/getownedlab", "getownedlab"),
        ("homeworks/week3/cardlab", "cardlab"),
        ("homeworks/week4/multilab", "multilab"),
        ("homeworks/week5/pokerlab", "pokerlab"),
        ("homeworks/week5-ec/summarylab", "summarylab"),
        ("homeworks/week6/greplab", "greplab"),
        ("homeworks/week8/iterlab", "iterlab"),
        ("homeworks/week9/splitlab", "splitlab"),
        ("homeworks/week10/filterlab", "filterlab"),
        ("homeworks/week12/rowlab", "rowlab"),
    ];

    for (path, slug) in homeworks {
        if Path::new(path).exists() {
            let out_dir = format!("public/hw/{slug}");
            fs::create_dir_all(&out_dir).ok();

            // Create zip handout
            let zip_path = format!("{out_dir}/{slug}.zip");
            if let Err(e) = create_zip(path, &zip_path, slug) {
                println!("cargo:warning=Failed to zip {slug}: {e}");
            }

            // Generate docs
            let manifest = format!("{path}/Cargo.toml");
            let status = Command::new("cargo")
                .args([
                    "doc",
                    "--no-deps",
                    "--manifest-path",
                    &manifest,
                    "--target-dir",
                    &out_dir,
                ])
                .status();

            match status {
                Ok(s) if s.success() => {}
                Ok(_) => println!("cargo:warning=cargo doc failed for {slug}"),
                Err(e) => println!("cargo:warning=Failed to run cargo doc for {slug}: {e}"),
            }
        }
    }
}

fn create_zip(src_dir: &str, zip_path: &str, root_name: &str) -> io::Result<()> {
    let file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let src_path = Path::new(src_dir);
    add_dir_to_zip(&mut zip, src_path, root_name, &options)?;

    zip.finish()?;
    Ok(())
}

fn add_dir_to_zip(
    zip: &mut ZipWriter<File>,
    dir: &Path,
    prefix: &str,
    options: &SimpleFileOptions,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        // Skip target directory and hidden files
        if name == "target" || name.starts_with('.') {
            continue;
        }

        let zip_path = format!("{prefix}/{name}");

        if path.is_dir() {
            zip.add_directory(&zip_path, *options)?;
            add_dir_to_zip(zip, &path, &zip_path, options)?;
        } else {
            zip.start_file(&zip_path, *options)?;
            let contents = fs::read(&path)?;
            zip.write_all(&contents)?;
        }
    }
    Ok(())
}
