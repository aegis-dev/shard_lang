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

use crate::vm::VM;

fn sys_call_handler(_vm: &mut VM) { }

#[test]
fn execution_tests() {
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  nop"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  call test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  jump test"),
                String::from("test:"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x04"),
                String::from("  jump_c"),
                String::from("  push 0xff"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0xff"),
                String::from("  store8 0xaaaa"),
                String::from("  call test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  load8 0xaaaa"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0xff"),
                String::from("  push 0xaa"),
                String::from("  push 0xaa"),
                String::from("  store8_c"),
                String::from("  call test"),
                String::from("  itrpt"),
                String::from("test:"),
                String::from("  push 0xaa"),
                String::from("  push 0xaa"),
                String::from("  load8_c"),
                String::from("  rlb.set"),
                String::from("  return"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }

    //
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xff);
    }
    {
        let code = shard_compiler::compile_from_asm(
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

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x00);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x02"),
                String::from("  push 0x01"),
                String::from("  add"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x03);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x02"),
                String::from("  push 0x01"),
                String::from("  sub"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x01);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x02"),
                String::from("  push 0x02"),
                String::from("  mul"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x04);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x04"),
                String::from("  push 0x02"),
                String::from("  div_s"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x02);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0xC8"),
                String::from("  push 0x02"),
                String::from("  div_u"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x64);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x04"),
                String::from("  push 0x03"),
                String::from("  rem_s"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x01);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0xff"),
                String::from("  push 0xfe"),
                String::from("  div_u"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x01);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x02"),
                String::from("  push 0x02"),
                String::from("  pow"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x04);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0xff"),
                String::from("  abs"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x01);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x03"),
                String::from("  push 0x02"),
                String::from("  and"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x02);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x03"),
                String::from("  push 0x02"),
                String::from("  or"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x03);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0xaa"),
                String::from("  push 0x56"),
                String::from("  xor"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xfc);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x01"),
                String::from("  shl"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x02);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0xf0"),
                String::from("  push 0x01"),
                String::from("  shr_s"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xf8);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0xf0"),
                String::from("  push 0x01"),
                String::from("  shr_u"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x78);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0xf0"),
                String::from("  push 0x01"),
                String::from("  rotl"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0xe1);
    }
    {
        let code = shard_compiler::compile_from_asm(
            vec![
                String::from("  push 0x01"),
                String::from("  push 0x01"),
                String::from("  rotr"),
                String::from("  rlb.set"),
                String::from("  itrpt"),
            ]
        ).unwrap();

        let mut vm = VM::new(code).unwrap();
        vm.execute(sys_call_handler).unwrap();

        assert_eq!(vm.get_rlb(), 0x80);
    }
}