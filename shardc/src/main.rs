//
// Copyright © 2020-2022  Egidijus Lileika
//
// This file is part of Shard Lang project
//
// Shard Lang is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Shard Lang is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with shard_lang. If not, see <https://www.gnu.org/licenses/>.
//

use std::{env, io};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("libshardc [source_file]");
        return;
    }

    let source_file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to read {}", args[1]);
            return;
        }
    };
    let reader = io::BufReader::new(source_file);

    let mut lines = vec![];
    for line in reader.lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(_) => { }
        };
    }

    let bin = match libshardc::compile_from_asm(lines) {
        Ok(bin) => bin,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    if Path::new("out.bin").exists() {
        std::fs::remove_file("out.bin").unwrap();
    }

    let mut out_bin = File::create("out.bin").unwrap();
    match out_bin.write_all(bin.as_slice()) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    };
}

