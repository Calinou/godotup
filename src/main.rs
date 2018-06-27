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
extern crate directories;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate tempfile;
extern crate zip;

use ansi_term::Colour::White;
use clap::{App, AppSettings};
use directories::ProjectDirs;
use reqwest::{Client, StatusCode};
use std::{fs, io, process};

mod logger;

fn main() {
    let logger = logger::init();
    assert!(logger.is_ok());

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let project_dir = ProjectDirs::from("", "", "godotup");
    let _data_dir = project_dir.data_local_dir();

    if let Some(matches) = matches.subcommand_matches("install") {
        let reference = matches.value_of("version").unwrap();
        info!(
            "Downloading Godot {} source code...",
            White.bold().paint(reference)
        );

        let client = Client::new();
        let mut response = client
            .get(&format!(
                "https://github.com/godotengine/godot/archive/{}.zip",
                reference
            ))
            .send()
            .unwrap();

        let mut tmpfile = tempfile::tempfile().unwrap();
        let _file = response.copy_to(&mut tmpfile);

        match response.status() {
            StatusCode::Ok => info!("Download completed."),
            status => {
                error!("Download failed ({:?})", status);
                process::exit(1);
            }
        }

        info!("Extracting ZIP archive...");
        let mut archive = zip::ZipArchive::new(tmpfile).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let out_path = file.sanitized_name();

            if (&*file.name()).ends_with('/') {
                // File is a directory
                fs::create_dir_all(&out_path).unwrap();
            } else {
                // File is a file
                if let Some(p) = out_path.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }

                let mut out_file = fs::File::create(&out_path).unwrap();
                io::copy(&mut file, &mut out_file).unwrap();
            }
        }
    }
}
