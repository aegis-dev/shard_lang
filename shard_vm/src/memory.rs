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

pub trait Memory {
    fn write_u8(&mut self, address: u16, value: u8) -> Result<(), String>;
    fn read_u8(&self, address: u16) -> Result<u8, String>;

    fn stack_start_address(&self) -> u16;
    fn dump_memory(&self) -> Vec<u8>;
}
