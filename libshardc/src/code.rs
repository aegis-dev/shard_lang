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

use crate::instruction::Instruction;
use crate::out_bin::OutBin;

pub struct Code {
    code: Vec<Instruction>
}

impl Code {
    pub fn new() -> Code {
        Code { code: vec![] }
    }

    pub fn new_with_code(code: Vec<Instruction>) -> Code {
        Code { code }
    }

    pub fn push_instruction(&mut self, instruction: Instruction) {
        self.code.push(instruction);
    }

    pub fn set_code(&mut self, code: Vec<Instruction>) {
        self.code = code;
    }

    pub fn get_code(&self) -> &Vec<Instruction> {
        &self.code
    }

    pub fn get_mut_code(&mut self) -> &mut Vec<Instruction> {
        &mut self.code
    }

    pub fn encode(&self, bin: &mut OutBin) -> Result<(), String> {
        for instruction in self.code.iter() {
            instruction.encode(bin)?;
        }
        Ok(())
    }
}