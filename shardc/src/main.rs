//
// Copyright © 2020-2023  Egidijus Lileika
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

#[cfg(test)]
mod tests;

use std::{env, io};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::collections::{HashSet, HashMap};


fn print_help() {
    println!("shardc [source_file]");
    println!("shardc --help");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    if args[1] == "--help" {
        print_help();
        return;
    } else if args[1].starts_with("-") {
        print_help();
        return;
    }

    let main_module_name = String::from("main");
    let mut included_modules = HashSet::new();
    let mut standard_modules = HashMap::new();
    standard_modules.insert(String::from("std/malloc"), String::from(include_str!("../../standard_modules/std/malloc.srd")));

    let lines = match load_module_from_file(&args[1], &main_module_name, &mut included_modules, &standard_modules) {
        Ok(lines) => lines,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let bin = match shard_compiler::compile_from_asm(lines) {
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

fn load_source_from_file(module_path: &String) -> Result<Vec<String>, String> {
    let source_file = match File::open(module_path) {
        Ok(file) => file,
        Err(_) => {
            return Err(String::from(format!("Failed to read {}", module_path)));
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

    Ok(lines)
}

fn load_source_from_string(module_string: &str) -> Result<Vec<String>, String> {
    let module_string =  module_string.replace("\r\n", "\n");
    let module_src_lines = module_string.split("\n");

    let mut lines = vec![];
    for line in module_src_lines {
        lines.push(String::from(line));
    }

    Ok(lines)
}

pub fn load_module_from_file(module_path: &String, module_name: &String, included_modules: &mut HashSet<String>, standard_modules: &HashMap<String, String>) -> Result<Vec<String>, String> {
    if included_modules.contains(module_name) {
        return Ok(vec![])
    }
    included_modules.insert(module_name.clone());

    let mut lines = load_source_from_file(module_path)?;
    let current_module_dir = String::from(Path::new(module_path).parent().expect("Unexpected error occurred").to_str().unwrap());

    preprocess_source(&mut lines, &current_module_dir, included_modules, standard_modules)?;

    Ok(lines)
}

pub fn load_module_from_string(module_string: &String, module_name: &String, current_module_dir: &String, included_modules: &mut HashSet<String>, standard_modules: &HashMap<String, String>) -> Result<Vec<String>, String> {
    if included_modules.contains(module_name) {
        return Ok(vec![])
    }
    included_modules.insert(module_name.clone());

    let mut lines = load_source_from_string(module_string)?;

    preprocess_source(&mut lines, current_module_dir, included_modules, standard_modules)?;

    Ok(lines)
}

fn preprocess_source(asm_source: &mut Vec<String>, current_module_dir: &String, included_modules: &mut HashSet<String>, standard_modules: &HashMap<String, String>) -> Result<(), String> {
    let mut sources_to_add = vec![];

    let mut lines_to_remove = vec![];

    for line_number in 0..asm_source.len() {
        let mut line = asm_source[line_number].clone();

        line = match line.find(";") {
            None => line,
            Some(delimiter) => String::from(line.split_at(delimiter).0)
        };

        let mut token_it = line.split_whitespace();

        let keyword = match token_it.next() {
            Some(keyword) => keyword,
            None => continue
        };

        if keyword == "#import" {
            let module_name = match token_it.next() {
                Some(module) => String::from(module),
                None => return Err(String::from(format!("{}: invalid import - module is missing", line_number + 1)))
            };

            match standard_modules.get(&module_name) {
                None => {
                    let mut full_module_path = current_module_dir.clone();
                    full_module_path.push_str("/");
                    full_module_path.push_str(&module_name);

                    sources_to_add.push(load_module_from_file(&full_module_path, &module_name, included_modules, standard_modules)?);
                }
                Some(sys_module_source) => {
                    let mock_sys_dir = String::from("");
                    sources_to_add.push(load_module_from_string(sys_module_source, &module_name, &mock_sys_dir, included_modules, standard_modules)?);
                }
            }

            lines_to_remove.insert(0, line_number);
        }
    }

    for line_number in lines_to_remove {
        asm_source.remove(line_number);
    }

    for source_to_add in sources_to_add {
        asm_source.extend_from_slice(source_to_add.as_slice());
    }

    Ok(())
}



