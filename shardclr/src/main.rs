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

mod interrupts;

use std::{env, path::Path, fs::File, io::{BufReader, Read}};

use shard_vm::vm::VM;


fn print_help() {
    println!("shardclr [binary_image_path]\nExample: shardclr image.bin");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let binary_image_path = &args[1];

    if !Path::new(binary_image_path).exists() {
        println!("{} file doesn't exist", binary_image_path);
        return;
    }

    let binary_image = {
        let file = File::open(binary_image_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();
        buffer
    };

    let mut vm = VM::new(binary_image).unwrap();

    if let Err(err) = vm.execute(interrupts::interrupt_handler) {
        println!("shardclr error:\n{}", err);
    };
}
