/*  A program to convert markup scripts to html for viewing.
    Copyright (C) 2022 Sherlock Holmes

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
pub mod read_write {
    pub fn read_input_file(path: &str) -> String {
        use std::fs;
        let input = fs::read_to_string(path).expect("ERR 00 -- Unable to read input");
        return input;
    }
    pub fn write_output_file(output_file: &str, contents: &str) {
        use std::fs;
        fs::write(output_file, contents).expect("ERR 01 -- Unable to write file");
    }
}