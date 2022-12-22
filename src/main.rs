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
pub mod parse;
pub mod template;

use crate::io::read_write;
use crate::args::read_args;
use crate::parse::parse_modules;
use crate::template::boilerplate;

use indicatif::ProgressBar;
use std::{thread, time};
use console::{style, Style};
fn main() {
    let input_filepath = read_args::get_input_file();
    let output_filepath = read_args::get_output_file();
    let input_text = read_write::read_input_file(&input_filepath);
    let mut output_text = boilerplate::basic_html_top();
    let mut paragraph_buffer = String::new();
    let bar = ProgressBar::new(input_text.lines().count() as u64);
    for line in input_text.lines() {
        bar.inc(1);
        if line[0..1].contains("#") {
            if paragraph_buffer.is_empty() == false {
                output_text.push_str(&parse_modules::parse_methods::paragraph(paragraph_buffer.to_string()));
                paragraph_buffer.clear();
            }
            output_text.push_str(&parse_modules::parse_methods::headers(line.to_string()));
        } else if line.contains("{start}") {
            
        } else if line.contains("{end}") {
            if paragraph_buffer.is_empty() == false {
                output_text.push_str(&parse_modules::parse_methods::paragraph(paragraph_buffer.to_string()));
                paragraph_buffer.clear();
            }
        } else if line.contains("{title}") {
        } else {
            paragraph_buffer.push_str(line);
            paragraph_buffer.push_str("\n");
        }
    }
    output_text.push_str(&boilerplate::basic_html_bottom());
    read_write::write_output_file(&output_filepath, &output_text);
    bar.finish();
}
