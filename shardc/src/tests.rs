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

use std::collections::{HashSet, HashMap};

#[test]
fn test_module_import() {
    {
        let main_module = String::from("\
            #import std/malloc\n
            #import std/malloc\n
        ");
        let module_name = String::from("main");
        let mock_dir = String::from("");

        let mut included_modules = HashSet::new();
        let mut standard_modules = HashMap::new();
        standard_modules.insert(String::from("std/malloc"), String::from(include_str!("../../standard_modules/std/malloc.srd")));

        let lines = crate::load_module_from_string(&main_module, &module_name, &mock_dir, &mut included_modules, &standard_modules).unwrap();

        let mut count = 0;
        for line in lines {
            if line.contains("malloc:") {
                count += 1;
            }
        }

        assert_eq!(count, 1);
    }
    {
        let main_module = String::from("#import std/malloc\n#import std/bad_name\n");
        let module_name = String::from("main");
        let mock_dir = String::from("");

        let mut included_modules = HashSet::new();
        let mut standard_modules = HashMap::new();
        standard_modules.insert(String::from("std/malloc"), String::from(include_str!("../../standard_modules/std/malloc.srd")));

        match crate::load_module_from_string(&main_module, &module_name, &mock_dir, &mut included_modules, &standard_modules) {
            Ok(_) => {
                assert!(false);
            }
            Err(err) => {
                assert_eq!("Failed to read /std/bad_name", err);
            }
        };
    }
}