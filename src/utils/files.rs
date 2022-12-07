extern crate glob;

use std::{env, fs};
use std::path::Path;

use glob::{glob, GlobResult};
use log::log;

pub fn get_files() {
    let current_dir = env::current_dir();
    log::info!("{:?}", current_dir)
}
