// Copyright lowRISC contributors.
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs::FileType;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use clap::Parser;
use indicatif::{MultiProgress, ProgressBar};
use rayon::prelude::*;

#[derive(Parser)]
struct Options {
    src: PathBuf,
    dest: PathBuf,
}

static OPTIONS: LazyLock<Options> = LazyLock::new(|| Options::parse());

static PROGRESS: LazyLock<MultiProgress> = LazyLock::new(|| MultiProgress::new());
static OVERALL_PROGRESS: LazyLock<ProgressBar> =
    LazyLock::new(|| PROGRESS.add(ProgressBar::new(0)));

thread_local! {
    static LOCAL_PROGRESS: ProgressBar = {
        PROGRESS.insert_before(&OVERALL_PROGRESS, ProgressBar::new_spinner())
    };
}

fn copy(path: &Path, file_type: FileType) -> std::io::Result<()> {
    LOCAL_PROGRESS.with(|p| {
        p.tick();
        p.set_message(path.display().to_string());
    });
    OVERALL_PROGRESS.inc(1);

    let src = OPTIONS.src.join(&path);
    let dst = OPTIONS.dest.join(&path);

    if !file_type.is_dir() {
        if file_type.is_symlink() {
            let link = std::fs::read_link(src)?;
            std::os::unix::fs::symlink(link, dst)?;
        } else if file_type.is_file() {
            std::fs::copy(src, dst)?;
        } else {
            PROGRESS.println(format!(
                "Ignore file {} with type {:?}",
                path.display(),
                file_type
            ))?;
        }

        return Ok(());
    }

    std::fs::create_dir_all(dst)?;

    let entries = std::fs::read_dir(&src)?.collect::<std::io::Result<Vec<_>>>()?;
    OVERALL_PROGRESS.inc_length(entries.len() as _);

    entries.par_iter().try_for_each(|entry| {
        let file_type = entry.file_type()?;
        copy(&path.join(entry.file_name()), file_type)
    })
}

fn main() {
    LazyLock::force(&OPTIONS);

    let file_type = std::fs::metadata(&OPTIONS.src).unwrap().file_type();
    copy("".as_ref(), file_type).unwrap();
}
