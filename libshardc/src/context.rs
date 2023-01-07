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

use std;
use std::collections::HashSet;

use crate::code::Code;
use crate::glob::Glob;
use crate::out_bin::OutBin;

pub struct Context {
    code: Code,
    globs: Vec<Glob>,
    unique_labels: HashSet<String>,
}

impl Context {
    pub fn new() -> Context {
        Context { code: Code::new(), globs: vec![], unique_labels: HashSet::new() }
    }

    pub fn get_code(&self) -> &Code {
        &self.code
    }

    pub fn get_code_mut(&mut self) -> &mut Code {
        &mut self.code
    }

    pub fn add_glob(&mut self, glob: Glob) -> Result<(), String> {
        if self.unique_labels.contains(glob.get_name()) {
            return Err(String::from(format!("Label with name '{}' already exist", glob.get_name())))
        }
        self.globs.push(glob);
        Ok(())
    }

    pub fn get_globs(&self) -> &Vec<Glob> {
        &self.globs
    }

    pub fn write_binary(&self) -> Result<Vec<u8>, String> {
        let mut bin = OutBin::new();

        self.code.encode(&mut bin)?;

        for glob in &self.globs {
            glob.encode(&mut bin)?;
        }

        for (offset, label_dest) in bin.addresses_to_update.iter() {
            match bin.address_table.get(label_dest) {
                Some(dest_offset) => {
                    let bytes = dest_offset.to_le_bytes();
                    let offset_as_idx = *offset as usize;
                    bin.code[offset_as_idx + 0] = bytes[0];
                    bin.code[offset_as_idx + 1] = bytes[1];
                }
                None => {
                    return Err(String::from(format!("Unknown label '{0}'", label_dest)));
                }
            }
        }

        Ok(bin.get_bytes())
    }

    // // TODO finish implementing binary loading
    // pub fn load_binary(&mut self, file_name: &str) -> Result<(), String> {
    //     Ok(())
    // }
}