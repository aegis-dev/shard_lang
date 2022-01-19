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

pub mod instruction;
pub mod code;
pub mod glob;
pub mod out_bin;
pub mod context;

#[cfg(test)]
mod tests;

use shard_core::opcodes::Opcode;

use crate::context::Context;
use crate::glob::Glob;
use crate::instruction::{Instruction, Literal};

pub fn compile_from_asm(asm_source: Vec<String>) -> Result<Vec<u8>, String> {
    let mut context = Context::new();

    for line_number in 0..asm_source.len() {
        parse_asm_line(&mut context, &asm_source[line_number], line_number + 1)?;
    }

    context.write_binary()
}

pub fn parse_asm_line(context: &mut Context, line: &str, line_number: usize) -> Result<(), String> {
    let line = match line.find(";") {
        None => line,
        Some(delimiter) => line.split_at(delimiter).0
    };

    let mut token_it = line.split_whitespace();

    let keyword = match token_it.next() {
        Some(keyword) => keyword,
        None => return Ok(())
    };

    match Opcode::from_string(keyword) {
        None => {
            // label or global
            if keyword.ends_with(":") {
                let mut label = String::from(keyword);
                label.pop();
                let mut data = vec![];

                for data_str in token_it {
                    let value_str_trimmed = data_str.trim_start_matches("0x");
                    match u8::from_str_radix(value_str_trimmed, 16) {
                        Ok(value) => data.push(value),
                        Err(err) => return Err(String::from(format!("{}: failed to parse value '{}' - {}", line_number, data_str, err)))
                    }
                }

                if data.is_empty() {
                    let literal = Literal::Label(label);
                    context.get_code_mut().push_instruction(Instruction::new_with_literal(Opcode::Label, literal));
                } else {
                    context.add_glob(Glob::new_with_value(label, data))?;
                }
            } else {
                return Err(String::from(format!("{}: Invalid keyword", line_number)))
            }

        }
        Some(opcode) => {
            match Opcode::is_opcode_instruction(opcode) {
                true => {
                    context.get_code_mut().push_instruction(Instruction::new(opcode));
                }
                false => {
                    let value_str = match token_it.next() {
                        Some(value_str) => value_str,
                        None => return Ok(())
                    };

                    match opcode {
                        // label / u16
                        Opcode::Call | Opcode::Jump | Opcode::Load | Opcode::Store | Opcode::Eqz |
                        Opcode::Eq | Opcode::Ne | Opcode::LtS | Opcode::LtU | Opcode::GtS |
                        Opcode::GtU | Opcode::LeS | Opcode::LeU | Opcode::GeS | Opcode::GeU
                        => {
                            match value_str.starts_with("0x") {
                                true => {
                                    let value_str_trimmed = value_str.trim_start_matches("0x");
                                    match u16::from_str_radix(value_str_trimmed, 16) {
                                        Ok(value) => {
                                            let literal = Literal::Address(value);
                                            context.get_code_mut().push_instruction(Instruction::new_with_literal(opcode, literal));
                                        },
                                        Err(err) => {
                                            return Err(String::from(format!("{}: failed to parse value '{}' - {}", line_number, value_str, err)))
                                        }
                                    }
                                }
                                false => {
                                    let literal = Literal::Label(value_str.to_string());
                                    context.get_code_mut().push_instruction(Instruction::new_with_literal(opcode, literal));
                                }
                            };
                        }
                        // u8
                        Opcode::StackGet | Opcode::StackSet | Opcode::Push => {
                            let value = match value_str.starts_with("0x") {
                                true => {
                                    let value_str_trimmed = value_str.trim_start_matches("0x");
                                    match u8::from_str_radix(value_str_trimmed, 16) {
                                        Ok(value) => value,
                                        Err(err) => return Err(String::from(format!("{}: failed to parse value '{}' - {}", line_number, value_str, err)))
                                    }
                                }
                                false => {
                                    return Err(String::from(format!("{}: Value is not a hex number", line_number)))
                                }
                            };
                            let literal = Literal::Const(value);
                            context.get_code_mut().push_instruction(Instruction::new_with_literal(opcode, literal));
                        }
                        _ => {
                            assert!(!Opcode::is_opcode_instruction(opcode));
                        }
                    }
                }
            }
        }
    }

    Ok(())
}


