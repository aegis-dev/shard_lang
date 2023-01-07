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

use crate::vm::{VM_STACK_SIZE, VM_MAX_IMAGE_SIZE};

pub trait Memory {
    fn write_u8(&mut self, address: u16, value: u8) -> Result<(), String>;
    fn read_u8(&self, address: u16) -> Result<u8, String>;

    fn stack_start_address(&self) -> u16;
    fn call_stack_start_address(&self) -> u16;

    fn dump_memory(&self) -> Vec<u8>;
    fn dump_memory_range(&self, start: u16, end: u16) -> Vec<u8>;
}

pub struct DefaultMemory {
    memory: Vec<u8>,
    stack_start_address: u16,
    call_stack_start_address: u16,
    ram_start_address: u16,
}

impl DefaultMemory {
    pub fn new(code: Vec<u8>) -> Result<DefaultMemory, String> {
        if code.len() > u16::MAX as usize {
            return Err(String::from(format!(
                    "Code size {} exceeding {} limit",
                code.len(), u16::MAX
            )));
        }

        let mut memory = vec![];
        let mut code_temp = code;
        memory.append(&mut code_temp);

        let stack_start_address = memory.len() as u16;
        let mut stack = vec![0 as u8; VM_STACK_SIZE];
        memory.append(&mut stack);

        let call_stack_start_address = memory.len() as u16;
        let mut call_stack = vec![0 as u8; VM_STACK_SIZE];
        memory.append(&mut call_stack);

        let ram_start_address = memory.len() as u16;
        let mut ram = vec![0 as u8; VM_MAX_IMAGE_SIZE - memory.len()];
        memory.append(&mut ram);

        assert_eq!(memory.len(), VM_MAX_IMAGE_SIZE);

        Ok(DefaultMemory { memory, stack_start_address, call_stack_start_address, ram_start_address })
    }
}

impl Memory for DefaultMemory {
    fn write_u8(&mut self, address: u16, value: u8) -> Result<(), String> {
        self.memory[address as usize] = value;
        Ok(())
    }

    fn read_u8(&self, address: u16) -> Result<u8, String> {
        let value = self.memory[address as usize];
        Ok(value)
    }

    fn stack_start_address(&self) -> u16 {
        self.stack_start_address
    }

    fn call_stack_start_address(&self) -> u16 {
        self.call_stack_start_address
    }

    fn dump_memory(&self) -> Vec<u8> {
        self.memory.clone()
    }

    fn dump_memory_range(&self, start: u16, end: u16) -> Vec<u8> {
        self.memory[start as usize..end as usize].to_vec()
    }
}
