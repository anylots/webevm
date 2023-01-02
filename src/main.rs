use rand::{thread_rng, Rng};
use std::fs;
use std::io::prelude::*;
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;

use std::fs::File;

use std::io::BufWriter;

use std::cell::RefCell;
use std::env;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

use webevm::service::execute;

fn main() {
    println!("execute starting!");
    execute::get_last();
}

