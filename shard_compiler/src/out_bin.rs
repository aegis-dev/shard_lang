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

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct OutBin {
    pub code: Vec<u8>,
    pub address_table: HashMap<String, u16>,
    pub addresses_to_update: HashMap<u16, String>,
}

impl OutBin {
    pub fn new() -> OutBin {
        OutBin {
            code: vec![],
            address_table: HashMap::new(),
            addresses_to_update: HashMap::new(),
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.code.clone()
    }

    pub fn write_binary_into_file(&self, file: &mut File) -> Result<(), String> {
        OutBin::write_bytes(file, &self.code)?;
        Ok(())
    }

    fn write_bytes(file: &mut File, bytes: &[u8]) -> Result<(), String> {
        match file.write(bytes) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Failed to write bytes"))
        }
    }
}