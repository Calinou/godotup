// godotup: Install and manage multiple Godot Engine versions
//
// Copyright © 2018 Hugo Locurcio and contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate zip;

use ansi_term::Colour::{Cyan, Green, Red};
use std::{io, fs};
use clap::{App, AppSettings};
use reqwest::{Client, StatusCode};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        let reference = matches.value_of("version").unwrap();
        println!("Downloading Godot {} source code...", Cyan.bold().paint(reference));

        let client = Client::new();
        let response = client.get(
            &format!("https://github.com/godotengine/godot/archive/{}.zip", reference)
        ).send().unwrap();

        match response.status() {
            StatusCode::Ok => println!("{}", Green.bold().paint("Success!")),
            status => println!("{} {:?}", Red.bold().paint("Error:"), status),
        }
    }
}
