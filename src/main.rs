use rand::distributions::DistString;
use std::os::unix::ffi::OsStrExt;
use std::{
    ffi::{OsStr, OsString},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[arg(long, env = "TODO_FILE_PATH")]
    file_path: PathBuf,
    #[arg(long)]
    open: bool,

    #[arg(trailing_var_arg = true)]
    text: Vec<OsString>,
}

pub fn store_to_file_with<F>(path: &Path, f: F) -> io::Result<()>
where
    F: FnOnce(&mut dyn io::Write) -> io::Result<()>,
{
    fs::create_dir_all(path.parent().expect("Not a root path"))?;
    let tmp_path = path.with_extension(format!(
        "{}.tmp",
        rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
    ));
    let mut file = fs::File::create(&tmp_path)?;
    f(&mut file)?;
    file.flush()?;
    file.sync_data()?;
    drop(file);
    fs::rename(tmp_path, path)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let new_text = opts.text.join(OsStr::new(" "));

    if new_text.is_empty() {
        if !opts.open {
            io::copy(
                &mut fs::File::open(&opts.file_path)?,
                &mut std::io::stdout(),
            )?;
        }
    } else {
        prepend_new_text(&opts, new_text)?;
    }

    if opts.open {
        spawn_xdg_open(opts)?;
    } else {
    }

    Ok(())
}

fn spawn_xdg_open(opts: Opts) -> Result<(), anyhow::Error> {
    std::process::Command::new("xdg-open")
        .arg(opts.file_path)
        .spawn()?;
    Ok(())
}

fn prepend_new_text(opts: &Opts, new_text: OsString) -> Result<(), anyhow::Error> {
    let mut prev = if opts.file_path.exists() {
        Some(fs::File::open(&opts.file_path)?)
    } else {
        None
    };

    store_to_file_with(&opts.file_path, |w| {
        w.write_all(b"* ")?;
        w.write_all(new_text.as_bytes())?;
        w.write_all(b"\n")?;
        if let Some(prev) = prev.as_mut() {
            io::copy(prev, w)?;
        }

        Ok(())
    })?;

    Ok(())
}
