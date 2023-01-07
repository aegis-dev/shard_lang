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

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use shard_vm::vm::VM;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum Syscall {
    Read = 0x00,
    Write = 0x01,
}

pub fn syscall_handler(vm: &mut VM) {
    let syscall_id = vm.stack_pop().unwrap();
    let syscall = Syscall::try_from(syscall_id).unwrap();

    match syscall {
        Syscall::Read => {

        },
        Syscall::Write => {
            let size = vm.stack_pop().unwrap();
            let data_address = vm.stack_pop_address().unwrap();
            let _output_index = vm.stack_pop().unwrap();

            let mut data = vec![];
            for offset in 0..size as u16 {
                data.push(vm.peek_memory(data_address + offset).unwrap());
            }

            // TODO: use output_index
            print!("{}", String::from_utf8(data).unwrap());
        },
    }
}