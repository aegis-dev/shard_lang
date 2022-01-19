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

use crate::out_bin::OutBin;

pub struct Glob {
    name: String,
    value: Vec<u8>,
}

impl Glob {
    pub fn new(name: String) -> Glob {
        Glob { name, value: vec![] }
    }

    pub fn new_with_value(name: String, value: Vec<u8>) -> Glob {
        Glob { name, value }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_value(&self) -> &Vec<u8> {
        &self.value
    }

    pub fn encode(&self, bin: &mut OutBin) -> Result<(), String> {
        let glob_address = bin.code.len() as u16;
        bin.code.extend_from_slice(&self.value);
        match bin.address_table.insert(self.name.clone(), glob_address) {
            None => Ok(()),
            Some(_) => Err(String::from(format!("'{}' label already exist", self.name)))
        }
    }
}