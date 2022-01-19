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

use crate::memory::Memory;
use crate::vm::VM;

struct SimpleMemory {
    memory: Vec<u8>,
}

impl SimpleMemory {
    pub fn new(code: Vec<u8>) -> Result<SimpleMemory, String> {
        if code.len() > u16::MAX as usize {
            return Err(String::from(format!(
                "Code size {} exceeding {} limit",
                code.len(), u16::MAX
            )));
        }

        let mut memory = vec![];
        let mut code_temp = code;
        memory.append(&mut code_temp);

        let mut padding = vec![0 as u8; u16::MAX as usize - memory.len()];
        memory.append(&mut padding);
        assert_eq!(memory.len(), u16::MAX as usize);

        Ok(SimpleMemory { memory })
    }
}

impl Memory for SimpleMemory {
    fn write_u8(&mut self, address: u16, value: u8) -> Result<(), String> {
        self.memory[address as usize] = value;
        Ok(())
    }

    fn read_u8(&self, address: u16) -> Result<u8, String> {
        let value = self.memory[address as usize];
        Ok(value)
    }

    // Stack at the very end
    fn stack_start_address(&self) -> u16 {
        0xfeff
    }

    fn dump_memory(&self) -> Vec<u8> {
        self.memory.clone()
    }
}

#[test]
fn execution_tests() {
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  nop"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  call test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  jump test"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x04"),
                String::from("  jump_c"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  call test"),
                String::from("  push 0xff"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  rmb.set"),
                String::from("  rlb.set"),
                String::from("  rlb.get"),
                String::from("  rmb.get"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x03);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xff"),
                String::from("  store 0xaaaa"),
                String::from("  call test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  load 0xaaaa"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xff"),
                String::from("  push 0xaa"),
                String::from("  push 0xaa"),
                String::from("  store_c"),
                String::from("  call test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xaa"),
                String::from("  push 0xaa"),
                String::from("  load_c"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x00"),
                String::from("  eqz test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xaa"),
                String::from("  push 0xaa"),
                String::from("  eq test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xaf"),
                String::from("  push 0xaa"),
                String::from("  ne test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x00"),
                String::from("  push 0x01"),
                String::from("  lt_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xf0"),
                String::from("  push 0xff"),
                String::from("  lt_u test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xf0"),
                String::from("  push 0x0"),
                String::from("  lt_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xf0"),
                String::from("  push 0x0"),
                String::from("  lt_u test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x00"),
                String::from("  gt_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x01"),
                String::from("  gt_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x00"),
                String::from("  push 0xf0"),
                String::from("  gt_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x00"),
                String::from("  gt_u test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x00"),
                String::from("  push 0xf0"),
                String::from("  gt_u test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x01"),
                String::from("  le_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x02"),
                String::from("  push 0x01"),
                String::from("  le_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x00"),
                String::from("  push 0xf0"),
                String::from("  le_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x01"),
                String::from("  le_u test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x00"),
                String::from("  push 0xf0"),
                String::from("  le_u test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }

    //
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x01"),
                String::from("  ge_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x02"),
                String::from("  push 0x01"),
                String::from("  ge_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x00"),
                String::from("  push 0xf0"),
                String::from("  ge_s test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x01"),
                String::from("  ge_u test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x00"),
                String::from("  push 0xf0"),
                String::from("  ge_u test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x02"),
                String::from("  push 0x01"),
                String::from("  add"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x03);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x02"),
                String::from("  push 0x01"),
                String::from("  sub"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x01);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x02"),
                String::from("  push 0x02"),
                String::from("  mul"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x04);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x04"),
                String::from("  push 0x02"),
                String::from("  div_s"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x02);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xC8"),
                String::from("  push 0x02"),
                String::from("  div_u"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x64);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x04"),
                String::from("  push 0x03"),
                String::from("  rem_s"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x01);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xff"),
                String::from("  push 0xfe"),
                String::from("  div_u"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x01);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x02"),
                String::from("  push 0x02"),
                String::from("  pow"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x04);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xff"),
                String::from("  abs"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x01);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x03"),
                String::from("  push 0x02"),
                String::from("  and"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x02);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x03"),
                String::from("  push 0x02"),
                String::from("  or"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x03);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xaa"),
                String::from("  push 0x56"),
                String::from("  xor"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xfc);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x01"),
                String::from("  shl"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x02);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xf0"),
                String::from("  push 0x01"),
                String::from("  shr_s"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xf8);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xf0"),
                String::from("  push 0x01"),
                String::from("  shr_u"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x78);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0xf0"),
                String::from("  push 0x01"),
                String::from("  rotl"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0xe1);
    }
    {
        let code = libshardc::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x01"),
                String::from("  rotr"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let memory = Box::new(SimpleMemory::new(code).unwrap());
        let mut vm = VM::new(memory);
        vm.execute().unwrap();

        assert_eq!(vm.get_rlb(), 0x80);
    }
}