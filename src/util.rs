use std::{
    env,
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{Context, Result};

const INPUTS_DIRECTORY: &str = "inputs";

pub fn input_file() -> Result<File> {
    let input_filename = match env::args_os().nth(1) {
        Some(filename) => filename,
        None => {
            let mut path: PathBuf = [
                OsStr::new(INPUTS_DIRECTORY),
                env::current_exe()
                    .context("Couldn't get executable filename")?
                    .file_stem()
                    .context("No executable filename")?,
            ]
            .iter()
            .collect();

            path.set_extension("txt");

            path.into_os_string()
        }
    };

    Ok(File::open(input_filename)?)
}

pub fn input_lines() -> Result<impl Iterator<Item = Result<String>>> {
    Ok(BufReader::new(input_file()?).lines().map(|line| Ok(line?)))
}
