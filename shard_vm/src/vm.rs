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

use std::convert::TryFrom;
use shard_core::opcodes::Opcode;
use crate::memory::{Memory, DefaultMemory};

pub const VM_ADDRESS_SIZE: usize = 2;
pub const VM_OPERAND_SIZE: usize = 1;
pub const VM_VALUE_SIZE: usize = VM_OPERAND_SIZE;
pub const VM_STACK_SIZE: usize = u8::MAX as usize + 1;
pub const VM_CALL_STACK_SIZE: usize = (u8::MAX as usize + 1) * 2;
pub const VM_MAX_IMAGE_SIZE: usize = u16::MAX as usize + 1;

pub struct VM {
    memory: Box<dyn Memory>,
    sp: u8,
    csp: u8,
    pc: u16,
    rmb: u8,
    rlb: u8,
}

pub enum ExecutionStatus {
    Ok,
    Interrupt,
    SysCall
}

impl VM {
    pub fn new(code: Vec<u8>) -> Result<VM, String> {
        let memory = Box::new(DefaultMemory::new(code)?);
        Ok(VM { sp: 0xff, csp: 0xff, pc: 0x00, rmb: 0x00, rlb: 0x00, memory })
    }

    pub fn new_with_custom_memory(memory: Box<dyn Memory>) -> VM {
        VM { sp: 0xff, csp: 0xff, pc: 0x00, rmb: 0x00, rlb: 0x00, memory }
    }

    pub fn peek_memory(&self, address: u16) -> Result<u8, String> {
        self.memory.read_u8(address)
    }

    pub fn dump_memory_range(&self, start: u16, end: u16) -> Vec<u8> {
        self.memory.dump_memory_range(start, end)
    }

    pub fn dump_memory(&self) -> Vec<u8> {
        self.memory.dump_memory()
    }

    pub fn get_memory_mut(&mut self) -> &mut dyn Memory {
        self.memory.as_mut()
    }

    pub fn get_rlb(&self) -> u8 {
        self.rlb
    }

    pub fn get_rmb(&self) -> u8 {
        self.rmb
    }

    pub fn reset(&mut self) {
        self.sp = 0xff;
        self.csp = 0xff;
        self.pc = 0x00;
        self.rmb = 0x00;
        self.rlb = 0x00;
    }

    pub fn execute(&mut self, sys_call_handler: fn(&mut VM)) -> Result<(), String> {
        self.reset();
        self.continue_execution(sys_call_handler)
    }

    pub fn continue_execution(&mut self, sys_call_handler: fn(&mut VM)) -> Result<(), String> {
        loop {
            match self.execute_instruction() {
                Ok(status) => {
                    match status {
                        ExecutionStatus::Ok => continue,
                        ExecutionStatus::Interrupt => return Ok(()),
                        ExecutionStatus::SysCall => {
                            sys_call_handler(self);
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }

    pub fn execute_instruction(&mut self) -> Result<ExecutionStatus, String> {
        let opcode_byte = match self.memory.read_u8(self.pc) {
            Ok(opcode_byte) => opcode_byte,
            Err(err) => return Err(err)
        };

        let opcode = match Opcode::try_from(opcode_byte) {
            Ok(opcode) => opcode,
            Err(_) => {
                return Err(String::from(format!("Unknown opcode byte: {}", opcode_byte)));
            }
        };

        self.pc = self.pc.wrapping_add(1);

        match opcode {
            Opcode::Interrupt => {
                // Push return address
                self.push_address_to_call_stack(self.pc)?;
                return Ok(ExecutionStatus::Interrupt);
            }
            Opcode::Return => {
                let address = self.address_from_call_stack()?;
                self.pc = address;
            }
            Opcode::JumpC => {
                let address = self.address_from_stack()?;
                self.pc = address;
            }
            Opcode::Call => {
                let address = self.operand_address()?;
                // Push return address
                self.push_address_to_call_stack(self.pc)?;
                self.pc = address;
            }
            Opcode::Jump => {
                let address = self.operand_address()?;
                self.pc = address;
            }
            Opcode::Push => {
                let value = self.operand_value()?;
                self.stack_push(value)?;
            }
            Opcode::Pop => {
                self.stack_pop()?;
            }
            Opcode::Nop => { }
            Opcode::Sys => {
                return Ok(ExecutionStatus::SysCall);
            }
            Opcode::StackGet => {
                let offset = self.operand_value()?;
                self.stack_peek(offset)?;
            }
            Opcode::StackSet => {
                let offset = self.operand_value()?;
                self.stack_set(offset)?;
            }
            Opcode::Load8 => {
                let address = self.operand_address()?;
                let value = self.memory.read_u8(address)?;
                self.stack_push(value)?;
            }
            Opcode::Load8C => {
                let address = self.address_from_stack()?;
                let value = self.memory.read_u8(address)?;
                self.stack_push(value)?;
            }
            Opcode::Load16 => {
                let address = self.operand_address()?;
                let msb = self.memory.read_u8(address)?;
                let lsb = self.memory.read_u8(address + 1)?;
                self.stack_push(lsb)?;
                self.stack_push(msb)?;
            }
            Opcode::Load16C => {
                let address = self.address_from_stack()?;
                let msb = self.memory.read_u8(address)?;
                let lsb = self.memory.read_u8(address + 1)?;
                self.stack_push(lsb)?;
                self.stack_push(msb)?;
            }
            Opcode::Store8 => {
                let address = self.operand_address()?;
                let value = self.stack_pop()?;
                self.memory.write_u8(address, value)?;
            }
            Opcode::Store8C => {
                let address = self.address_from_stack()?;
                let value = self.stack_pop()?;
                self.memory.write_u8(address, value)?;
            }
            Opcode::Store16 => {
                let address = self.operand_address()?;
                let msb = self.stack_pop()?;
                let lsb = self.stack_pop()?;
                self.memory.write_u8(address, msb)?;
                self.memory.write_u8(address + 1, lsb)?;
            }
            Opcode::Store16C => {
                let address = self.address_from_stack()?;
                let msb = self.stack_pop()?;
                let lsb = self.stack_pop()?;
                self.memory.write_u8(address, msb)?;
                self.memory.write_u8(address + 1, lsb)?;
            }
            Opcode::Eqz => {
                let value = self.stack_pop()?;
                let address = self.operand_address()?;
                if value == 0 {
                    self.pc = address;
                }
            }
            Opcode::Eq => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                let address = self.operand_address()?;
                if lhs == rhs {
                    self.pc = address;
                }
            }
            Opcode::Ne => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                let address = self.operand_address()?;
                if lhs != rhs {
                    self.pc = address;
                }
            }
            Opcode::LtS => {
                let rhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let lhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let address = self.operand_address()?;
                if lhs < rhs {
                    self.pc = address;
                }
            }
            Opcode::LtU => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                let address = self.operand_address()?;
                if lhs < rhs {
                    self.pc = address;
                }
            }
            Opcode::GtS => {
                let rhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let lhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let address = self.operand_address()?;
                if lhs > rhs {
                    self.pc = address;
                }
            }
            Opcode::GtU => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                let address = self.operand_address()?;
                if lhs > rhs {
                    self.pc = address;
                }
            }
            Opcode::LeS => {
                let rhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let lhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let address = self.operand_address()?;
                if lhs <= rhs {
                    self.pc = address;
                }
            }
            Opcode::LeU => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                let address = self.operand_address()?;
                if lhs <= rhs {
                    self.pc = address;
                }
            }
            Opcode::GeS => {
                let rhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let lhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let address = self.operand_address()?;
                if lhs >= rhs {
                    self.pc = address;
                }
            }
            Opcode::GeU => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                let address = self.operand_address()?;
                if lhs >= rhs {
                    self.pc = address;
                }
            }
            Opcode::Add => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(lhs.wrapping_add(rhs))?;
            }
            Opcode::Sub => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(lhs.wrapping_sub(rhs))?;
            }
            Opcode::Mul => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(lhs.wrapping_mul(rhs))?;
            }
            Opcode::DivS => {
                let rhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let lhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                self.stack_push(u8::from_le_bytes(lhs.wrapping_div(rhs).to_le_bytes()))?;
            }
            Opcode::DivU => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(lhs.wrapping_div(rhs))?;
            }
            Opcode::RemS => {
                let rhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let lhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                self.stack_push(u8::from_le_bytes(lhs.wrapping_rem(rhs).to_le_bytes()))?;
            }
            Opcode::RemU => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(lhs.wrapping_rem(rhs))?;
            }
            Opcode::Pow => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(lhs.wrapping_pow(rhs as u32))?;
            }
            Opcode::Abs => {
                let value = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                self.stack_push(u8::from_le_bytes(value.abs().to_le_bytes()))?;
            }
            Opcode::And => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(rhs & lhs)?;
            }
            Opcode::Or => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(rhs | lhs)?;
            }
            Opcode::Xor => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(rhs ^ lhs)?;
            }
            Opcode::Shl => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(lhs << rhs)?;
            }
            Opcode::ShrS => {
                let rhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                let lhs = i8::from_le_bytes(self.stack_pop()?.to_le_bytes());
                self.stack_push(u8::from_le_bytes((lhs >> rhs).to_le_bytes()))?;
            }
            Opcode::ShrU => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(lhs >> rhs)?;
            }
            Opcode::Rotl => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(lhs.rotate_left(rhs as u32))?;
            }
            Opcode::Rotr => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;
                self.stack_push(lhs.rotate_right(rhs as u32))?;
            }
            Opcode::RegMsbGet => {
                self.stack_push(self.rmb)?;
            }
            Opcode::RegLsbGet => {
                self.stack_push(self.rlb)?;
            }
            Opcode::RegMsbSet => {
                let value = self.stack_pop()?;
                self.rmb = value;
            }
            Opcode::RegLsbSet => {
                let value = self.stack_pop()?;
                self.rlb = value;
            }
            _ => {
                return Err(String::from("Opcode has no implementation"));
            }
        }

        Ok(ExecutionStatus::Ok)
    }

    #[inline(always)]
    pub fn stack_push(&mut self, value: u8) -> Result<(), String> {
        if self.sp <= 0 {
            return Err(String::from("Stack overflow"));
        }

        let address = self.memory.stack_start_address().wrapping_add(self.sp as u16);
        assert!(address >= self.memory.stack_start_address());
        self.sp = self.sp.wrapping_sub(1);
        self.memory.write_u8(address, value)?;
        Ok(())
    }

    #[inline(always)]
    pub fn stack_pop(&mut self) -> Result<u8, String> {
        if self.sp >= 0xff {
            return Err(String::from("Stack is empty"));
        }
        self.sp = self.sp.wrapping_add(1);
        let address = self.memory.stack_start_address().wrapping_add(self.sp as u16);
        assert!(address >= self.memory.stack_start_address());
        self.memory.read_u8(address)
    }

    #[inline(always)]
    pub fn call_stack_push(&mut self, value: u8) -> Result<(), String> {
        if self.csp <= 0 {
            return Err(String::from("Call stack overflow"));
        }

        let address = self.memory.call_stack_start_address().wrapping_add(self.csp as u16);
        assert!(address >= self.memory.call_stack_start_address());
        self.csp = self.csp.wrapping_sub(1);
        self.memory.write_u8(address, value)?;
        Ok(())
    }

    #[inline(always)]
    pub fn call_stack_pop(&mut self) -> Result<u8, String> {
        if self.csp >= 0xff {
            return Err(String::from("Call stack is empty"));
        }
        self.csp = self.csp.wrapping_add(1);
        let address = self.memory.call_stack_start_address().wrapping_add(self.csp as u16);
        assert!(address >= self.memory.call_stack_start_address());
        self.memory.read_u8(address)
    }

    #[inline(always)]
    fn stack_peek(&mut self, offset: u8) -> Result<(), String> {
        let stack_offset = self.sp as u16 + offset as u16;
        if stack_offset > 0xff {
            return Err(String::from("Stack offset out of range"));
        }

        let address = self.memory.stack_start_address().wrapping_add(stack_offset);
        assert!(address >= self.memory.stack_start_address());
        let value = self.memory.read_u8(address)?;
        self.stack_push(value)?;

        Ok(())
    }

    #[inline(always)]
    fn stack_set(&mut self, offset: u8) -> Result<(), String> {
        let stack_offset = self.sp as u16 + offset as u16;
        if stack_offset > 0xff {
            return Err(String::from("Stack offset out of range"));
        }

        let value = self.stack_pop()?;
        let address = self.memory.stack_start_address().wrapping_add(stack_offset);
        assert!(address >= self.memory.stack_start_address());
        self.memory.write_u8(address, value)?;

        Ok(())
    }

    #[inline(always)]
    fn address_from_bytes(msb: u8, lsb: u8) -> u16 {
        ((msb as u16) << 8) | lsb as u16
    }

    #[inline(always)]
    fn operand_address(&mut self) -> Result<u16, String> {
        let address = VM::address_from_bytes(
            self.memory.read_u8(self.pc.wrapping_add(1))?,
            self.memory.read_u8(self.pc)?
        );
        self.pc = self.pc.wrapping_add(2);

        Ok(address)
    }

    #[inline(always)]
    fn operand_value(&mut self) -> Result<u8, String> {
        let value = self.memory.read_u8(self.pc)?;
        self.pc = self.pc.wrapping_add(1);

        Ok(value)
    }

    #[inline(always)]
    fn address_from_stack(&mut self) -> Result<u16, String> {
        let msb = self.stack_pop()?;
        let lsb = self.stack_pop()?;
        Ok(VM::address_from_bytes(msb, lsb))
    }

    #[inline(always)]
    fn address_from_call_stack(&mut self) -> Result<u16, String> {
        let msb = self.call_stack_pop()?;
        let lsb = self.call_stack_pop()?;
        Ok(VM::address_from_bytes(msb, lsb))
    }

    #[inline(always)]
    fn push_address_to_call_stack(&mut self, address: u16) -> Result<(), String> {
        self.call_stack_push((address & 0x00ff) as u8)?;
        self.call_stack_push((address >> 8) as u8)?;
        Ok(())
    }
}