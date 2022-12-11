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
pub mod io;
pub mod args;
use crate::io::read_write;
use crate::args::read_args;
fn main() {
    let filepath = read_args::get_input_file();
    let input_text = read_write::read_input_file(&filepath);
    println!("{}", input_text);
}
