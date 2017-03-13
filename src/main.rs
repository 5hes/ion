#![allow(unknown_lints)]
#![allow(while_let_on_iterator)]
#![feature(box_syntax)]
#![feature(plugin)]
#![plugin(peg_syntax_ext)]
extern crate glob;
extern crate liner;

pub mod completer;
pub mod pipe;
pub mod directory_stack;
pub mod to_num;
pub mod variables;
pub mod status;
pub mod flow_control;
mod builtins;
mod parser;
mod shell;

use std::io::{stderr, Write, ErrorKind};

use shell::Shell;

fn main() {
    let mut shell = Shell::default();
    shell.evaluate_init_file();

    if "1" == shell.variables.get_var_or_empty("HISTORY_FILE_ENABLED") {
        match shell.context.history.load_history() {
            Ok(()) => {
                // pass
            }
            Err(ref err) if err.kind() == ErrorKind::NotFound => {
                let history_filename = shell.variables.get_var_or_empty("HISTORY_FILE");
                let _ = writeln!(stderr(), "ion: failed to find history file {}: {}", history_filename, err);
            },
            Err(err) => {
                let _ = writeln!(stderr(), "ion: failed to load history: {}", err);
            }
        }
    }
    shell.execute();
}
