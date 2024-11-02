// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod entity;

use entity::prelude::*;
fn main() {
    tauri_to_do_lib::run()
}
