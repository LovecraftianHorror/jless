// I don't like this rule because it changes the semantic
// structure of the code.
#![allow(clippy::collapsible_else_if)]

extern crate lazy_static;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use structopt::StructOpt;
use termion::cursor::HideCursor;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

pub mod app;
pub mod flatjson;
mod highlighting;
pub mod input;
pub mod jsonparser;
mod jsontokenizer;
mod lineprinter;
pub mod options;
mod screenwriter;
mod search;
mod terminal;
mod truncatedstrview;
mod types;
mod viewer;

use app::App;
use options::Opt;
