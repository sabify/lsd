// Copyright (c) 2017 fd developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#[macro_use]
extern crate clap;
extern crate version_check;

use clap::Shell;
use std::fs;
use std::process::exit;

include!("src/app.rs");

fn main() {
    // rustc version too small or can't figure it out
    if version_check::is_min_version("1.62.0") != Some(true) {
        eprintln!("'lsd' requires rustc >= 1.62.0");
        exit(1);
    }

    let outdir = std::env::var_os("SHELL_COMPLETIONS_DIR")
        .or_else(|| std::env::var_os("OUT_DIR"))
        .unwrap_or_else(|| exit(0));

    fs::create_dir_all(&outdir).unwrap();

    let mut app = build();
    app.gen_completions("lsd", Shell::Bash, &outdir);
    app.gen_completions("lsd", Shell::Fish, &outdir);
    app.gen_completions("lsd", Shell::Zsh, &outdir);
    app.gen_completions("lsd", Shell::PowerShell, &outdir);
}
