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
pub mod read_args{
    use std::env;

    fn get_input_file() -> String {
        use std::env;
        let args: Vec<String> = env::args().collect();
        let filename = args.get(1).expect("ERR 10 -- Unable to get filename");
        return filename.to_string();
    }
}