use crate::models::makefile::Makefile;
use crate::usecase::{fzf_make_old::fuzzy_finder, usecase_main::Usecase};
use anyhow::Result;
use colored::*;
use std::process;

pub struct FzfMakeOld;

impl FzfMakeOld {
    pub fn new() -> Self {
        Self {}
    }
}

impl Usecase for FzfMakeOld {
    fn command_str(&self) -> Vec<&'static str> {
        vec!["--o", "-o", "old"]
    }

    fn run(&self) -> Result<()> {
        let makefile = match Makefile::create_makefile() {
            Err(e) => {
                return Err(e);
            }
            Ok(f) => f,
        };

        let target = fuzzy_finder::run(makefile);

        // Make output color configurable via config file https://github.com/kyu08/fzf-make/issues/67
        println!("{}", ("make ".to_string() + &target).blue());
        process::Command::new("make")
            .stdin(process::Stdio::inherit())
            .arg(target)
            .spawn()
            .expect("Failed to execute process")
            .wait()
            .expect("Failed to execute process");

        Ok(())
    }
}
