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

 pub mod parse_modules {
    pub mod parse_methods {
        pub fn headers(input: String) -> String {
            let mut return_value = input.clone();
            let mut heading_level = 0;
            let mut pre_header = String::from("  <h");
            let mut post_header = String::from("</h");
            for character in input.chars() {
                if character == '#' {
                    heading_level += 1;
                } else {
                    break;
                }
            }
            for i in 1..=heading_level {
                return_value.remove(0);
            }
            pre_header.push_str(&heading_level.to_string());
            pre_header.push_str(">");
            post_header.push_str(&heading_level.to_string());
            post_header.push_str(">");
            return_value.insert_str(0, &pre_header);
            return_value.push_str(&post_header);
            return_value.push_str("\n");
            return return_value;
        }
        pub fn paragraph(input: String) -> String {
            let pre_header = String::from("  <p>");
            let post_header = String::from("</p>\n");
            let mut return_value = input.clone();
            return_value = return_value.trim().replace("\n", "<br>");
            return_value.insert_str(0,&pre_header);
            return_value.push_str(&post_header);
            return return_value;
        }
    }
 }