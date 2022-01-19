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
use crate::{Context, Literal, parse_asm_line};

#[test]
fn compile_from_string() {
    {
        let mut context = Context::new();

        parse_asm_line(&mut context, "main:", 1).unwrap();

        let code = context.get_code().get_code();
        assert_eq!(code.len(), 1);

        let literal = Literal::Label(String::from("main"));
        assert_eq!(code[0].get_opcode(), Opcode::Label);
        assert_eq!(code[0].get_literal(), &literal);
    }
    {
        let mut context = Context::new();

        parse_asm_line(&mut context, "push 0x02", 1).unwrap();

        let code = context.get_code().get_code();
        assert_eq!(code.len(), 1);

        let literal = Literal::Const(0x02);
        assert_eq!(code[0].get_opcode(), Opcode::Push);
        assert_eq!(code[0].get_literal(), &literal);
    }
    {
        let mut context = Context::new();

        parse_asm_line(&mut context, "add", 1).unwrap();

        let code = context.get_code().get_code();
        assert_eq!(code.len(), 1);

        assert_eq!(code[0].get_opcode(), Opcode::Add);
    }
    {
        let mut context = Context::new();

        parse_asm_line(&mut context, "glob: 0x00 0x01 0x02", 1).unwrap();

        let globs = context.get_globs();
        assert_eq!(globs.len(), 1);

        let value = vec![0x00, 0x01, 0x02];
        assert_eq!(globs[0].get_name(), "glob");
        assert_eq!(globs[0].get_value(), &value);
    }
    {
        let mut context = Context::new();

        parse_asm_line(&mut context, "jump away", 1).unwrap();

        let code = context.get_code().get_code();
        assert_eq!(code.len(), 1);

        let literal = Literal::Label(String::from("away"));
        assert_eq!(code[0].get_opcode(), Opcode::Jump);
        assert_eq!(code[0].get_literal(), &literal);
    }
    {
        let mut context = Context::new();

        parse_asm_line(&mut context, "jump 0x1234", 1).unwrap();

        let code = context.get_code().get_code();
        assert_eq!(code.len(), 1);

        let literal = Literal::Address(0x1234);
        assert_eq!(code[0].get_opcode(), Opcode::Jump);
        assert_eq!(code[0].get_literal(), &literal);
    }
    {
        let mut context = Context::new();

        parse_asm_line(&mut context, "push 0x01 ; comments should be ignored ", 1).unwrap();

        let code = context.get_code().get_code();
        assert_eq!(code.len(), 1);

        let literal = Literal::Const(0x01);
        assert_eq!(code[0].get_opcode(), Opcode::Push);
        assert_eq!(code[0].get_literal(), &literal);
    }
    {
        let mut context = Context::new();

        parse_asm_line(&mut context, " ; just a comment line with extra space ", 1).unwrap();

        let code = context.get_code().get_code();
        assert_eq!(code.len(), 0);
    }
    {
        let mut context = Context::new();

        parse_asm_line(&mut context, "glob: 0x00 0x01 0x02 ; also this comment should be ignored", 1).unwrap();

        let globs = context.get_globs();
        assert_eq!(globs.len(), 1);

        let value = vec![0x00, 0x01, 0x02];
        assert_eq!(globs[0].get_name(), "glob");
        assert_eq!(globs[0].get_value(), &value);
    }
    {
        let mut context = Context::new();

        match parse_asm_line(&mut context, "jump 0x12346", 1) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {}
        };
        match parse_asm_line(&mut context, "push 0x123", 1) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {}
        };
        match parse_asm_line(&mut context, "push asdasd", 1) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {}
        };
        match parse_asm_line(&mut context, "another_glob: 0x01, 0x001", 1) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {}
        };
        match parse_asm_line(&mut context, "another_glob: 0x01 0x100", 1) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {}
        };
        match parse_asm_line(&mut context, "another_glob: 0x01 100", 1) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {}
        };
        match parse_asm_line(&mut context, "another_glob: 0x01 booooom", 1) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {}
        };
    }
}