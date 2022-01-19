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

use shard_core::opcodes::Opcode;

use crate::out_bin::OutBin;

#[derive(Debug, PartialEq)]
pub enum Literal {
    None(),
    Const(u8),
    Address(u16),
    Label(String),
}

pub struct Instruction {
    opcode: Opcode,
    literal: Literal,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode, literal: Literal::None() }
    }

    pub fn new_with_literal(opcode: Opcode, literal: Literal) -> Instruction {
        Instruction { opcode, literal }
    }

    pub fn get_opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn get_literal(&self) -> &Literal {
        &self.literal
    }

    pub fn get_mut_literal(&mut self) -> &mut Literal {
        &mut self.literal
    }

    pub fn encode(&self, bin: &mut OutBin) -> Result<(), String> {
        match &self.opcode {
            Opcode::Label => {
                match &self.literal {
                    Literal::Label(label) => {
                        // Pseudo instruction - nothing to encode since this is jump destination.
                        // Putting destination into the map.
                        match bin.address_table.insert(label.clone(), bin.code.len() as u16) {
                            None => return Ok(()),
                            Some(_) => return Err(String::from(format!("'{}' label already exist", label)))
                        };
                    }
                    _ => return Err(String::from(format!("Invalid literal type with Label")))
                }
            }
            _ => {}
        }

        bin.code.push(self.opcode as u8);

        if Opcode::is_opcode_instruction(self.opcode) {
            // No literal - return
            return Ok(());
        }

        match &self.literal {
            Literal::None() => {
                assert!(Opcode::is_opcode_instruction(self.opcode));
            }
            Literal::Const(value) => {
                bin.code.push(*value);
            }
            Literal::Address(value) => {
                let bytes = value.to_le_bytes();
                bin.code.extend_from_slice(&bytes);
            }
            Literal::Label(value) => {
                bin.addresses_to_update.insert(bin.code.len() as u16, value.clone());
                // Push temporary address
                bin.code.push(0x00);
                bin.code.push(0x00);
            }
        }

        Ok(())
    }
}