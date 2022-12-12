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
pub mod boilerplate {
    use std::fs;
    pub fn basic_html_top() -> String {
        let mut return_value = fs::read_to_string("boilerplate/htmltop.txt").unwrap();
        return return_value;
    }
    pub fn basic_html_bottom() -> String {
        let mut return_value = fs::read_to_string("boilerplate/htmlbottom.txt").unwrap();
        return return_value;
    }
}