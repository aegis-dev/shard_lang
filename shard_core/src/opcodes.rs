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

use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum Opcode {
    Interrupt = 0x00,
    Return = 0x01,
    Call = 0x02,
    Jump = 0x03,
    JumpC = 0x04,
    Push = 0x05,
    Pop = 0x06,
    Label = 0x07, // pseudo opcode / jump target
    Nop = 0x08,
    Sys = 0x09,
    StackGet = 0x10,
    StackSet = 0x11,
    RegMsbGet = 0x12,
    RegLsbGet = 0x13,
    RegMsbSet = 0x14,
    RegLsbSet = 0x15,
    Load = 0x20,
    LoadC = 0x21,
    Store = 0x2c,
    StoreC = 0x2d,
    Eqz = 0x40,
    Eq = 0x41,
    Ne = 0x42,
    LtS = 0x43,
    LtU = 0x44,
    GtS = 0x45,
    GtU = 0x46,
    LeS = 0x47,
    LeU = 0x48,
    GeS = 0x49,
    GeU = 0x4a,
    Add = 0x60,
    Sub = 0x61,
    Mul = 0x62,
    DivS = 0x63,
    DivU = 0x64,
    RemS = 0x65,
    RemU = 0x66,
    Pow = 0x67,
    Abs = 0x68,
    And = 0x69,
    Or = 0x6a,
    Xor = 0x6b,
    Shl = 0x6c,
    ShrS = 0x6d,
    ShrU = 0x6e,
    Rotl = 0x6f,
    Rotr = 0x70,
}

impl Opcode {
    pub fn from_string(value: &str) -> Option<Opcode> {
        return match value {
            "itrpt" => Some(Opcode::Interrupt),
            "return" => Some(Opcode::Return),
            "call" => Some(Opcode::Call),
            "jump" => Some(Opcode::Jump),
            "jump_c" => Some(Opcode::JumpC),
            "push" => Some(Opcode::Push),
            "pop" => Some(Opcode::Pop),
            "label" => Some(Opcode::Label),
            "nop" => Some(Opcode::Nop),
            "sys" => Some(Opcode::Sys),
            "stack.get" => Some(Opcode::StackGet),
            "stack.set" => Some(Opcode::StackSet),
            "rmb.get" => Some(Opcode::RegMsbGet),
            "rlb.get" => Some(Opcode::RegLsbGet),
            "rmb.set" => Some(Opcode::RegMsbSet),
            "rlb.set" => Some(Opcode::RegLsbSet),
            "load" => Some(Opcode::Load),
            "load_c" => Some(Opcode::LoadC),
            "store" => Some(Opcode::Store),
            "store_c" => Some(Opcode::StoreC),
            "eqz" => Some(Opcode::Eqz),
            "eq" => Some(Opcode::Eq),
            "ne" => Some(Opcode::Ne),
            "lt_s" => Some(Opcode::LtS),
            "lt_u" => Some(Opcode::LtU),
            "gt_s" => Some(Opcode::GtS),
            "gt_u" => Some(Opcode::GtU),
            "le_s" => Some(Opcode::LeS),
            "le_u" => Some(Opcode::LeU),
            "ge_s" => Some(Opcode::GeS),
            "ge_u" => Some(Opcode::GeU),
            "add" => Some(Opcode::Add),
            "sub" => Some(Opcode::Sub),
            "mul" => Some(Opcode::Mul),
            "div_s" => Some(Opcode::DivS),
            "div_u" => Some(Opcode::DivU),
            "rem_s" => Some(Opcode::RemS),
            "rem_u" => Some(Opcode::RemU),
            "pow" => Some(Opcode::Pow),
            "abs" => Some(Opcode::Abs),
            "and" => Some(Opcode::And),
            "or" => Some(Opcode::Or),
            "xor" => Some(Opcode::Xor),
            "shl" => Some(Opcode::Shl),
            "shr_s" => Some(Opcode::ShrS),
            "shr_u" => Some(Opcode::ShrU),
            "rotl" => Some(Opcode::Rotl),
            "rotr" => Some(Opcode::Rotr),
            &_ => None
        }
    }

    pub fn to_string(&self) ->  &'static str {
        return match self {
            Opcode::Interrupt => "itrpt",
            Opcode::Return => "return",
            Opcode::Call => "call",
            Opcode::Jump => "jump",
            Opcode::JumpC => "jump_c",
            Opcode::Push => "push",
            Opcode::Pop => "pop",
            Opcode::Label => "label",
            Opcode::Nop => "nop",
            Opcode::Sys => "sys",
            Opcode::StackGet => "stack.get",
            Opcode::StackSet => "stack.set",
            Opcode::RegMsbGet => "rmb.get",
            Opcode::RegLsbGet => "rlb.get",
            Opcode::RegMsbSet => "rmb.set",
            Opcode::RegLsbSet => "rlb.set",
            Opcode::Load => "load",
            Opcode::LoadC => "load_c",
            Opcode::Store => "store",
            Opcode::StoreC => "store_c",
            Opcode::Eqz => "eqz",
            Opcode::Eq => "eq",
            Opcode::Ne => "ne",
            Opcode::LtS => "lt_s",
            Opcode::LtU => "lt_u",
            Opcode::GtS => "gt_s",
            Opcode::GtU => "gt_u",
            Opcode::LeS => "le_s",
            Opcode::LeU => "le_u",
            Opcode::GeS => "ge_s",
            Opcode::GeU => "ge_u",
            Opcode::Add => "add",
            Opcode::Sub => "sub",
            Opcode::Mul => "mul",
            Opcode::DivS => "div_s",
            Opcode::DivU => "div_u",
            Opcode::RemS => "rem_s",
            Opcode::RemU => "rem_u",
            Opcode::Pow => "pow",
            Opcode::Abs => "abs",
            Opcode::And => "and",
            Opcode::Or => "or",
            Opcode::Xor => "xor",
            Opcode::Shl => "shl",
            Opcode::ShrS => "shr_s",
            Opcode::ShrU => "shr_u",
            Opcode::Rotl => "rotl",
            Opcode::Rotr => "rotr",
        }
    }

    // Opcodes that have no additional arguments and act on their own
    pub fn is_opcode_instruction(opcode: Opcode) -> bool {
        return match opcode {
            Opcode::Nop => true,
            Opcode::Return => true,
            Opcode::Interrupt => true,
            Opcode::Sys => true,
            Opcode::JumpC => true,
            Opcode::Pop => true,
            Opcode::RegMsbGet => true,
            Opcode::RegLsbGet => true,
            Opcode::RegMsbSet => true,
            Opcode::RegLsbSet => true,
            Opcode::LoadC => true,
            Opcode::StoreC => true,
            Opcode::Add => true,
            Opcode::Sub => true,
            Opcode::Mul => true,
            Opcode::DivS => true,
            Opcode::DivU => true,
            Opcode::RemS => true,
            Opcode::RemU => true,
            Opcode::Pow => true,
            Opcode::Abs => true,
            Opcode::And => true,
            Opcode::Or => true,
            Opcode::Xor => true,
            Opcode::Shl => true,
            Opcode::ShrS => true,
            Opcode::ShrU => true,
            Opcode::Rotl => true,
            Opcode::Rotr => true,
            _ => false
        }
    }
}
