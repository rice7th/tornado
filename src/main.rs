/***====-------------------------------------------------------+
 | A simple C Compiler.                                        |
 | Copyright 2023 Giovanni Ricevuto (Rice7th)                  |
 | Licensed under the Apache License, Version 2.0              |
 | (the "License"); you may not use this file except in        |
 | compliance with the License.                                |
 | You may obtain a copy of the License at                     |
 |                                                             |
 |     http://www.apache.org/licenses/LICENSE-2.0              |
 |                                                             |
 | Unless required by applicable law or agreed to in writing,  |
 | software distributed under the License is distributed on    |
 | an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY   |
 | KIND, either express or implied.                            |
 | See the License for the specific language governing         |
 | permissions and limitations under the License.              |
 +-------------------------------------------------------------+
 | ███████████▓▓██████████▓▓▓▓▓▓▓▓▓██████████████████░░░░░░░░░ |
 | ██████████████▓▓▓▓█▓▓▓▓▓██████████████████████████░░░░░░░░░ |
 | ███▓██▓██▓▓██▓████████████████████████████████████░░░▀█▀░░░ |
 | ▓▓▓███████████████████████████████████████████████░░░░█░░░░ |
 | ██████████████████████████████████████████████████░░░░▀░░░░ |
 | █████████████████▓▓█████████████████████▓▓▓▓██████░░░█▀█░░░ |
 | ▓▓▓▓█████████▓▓▓███▓▓▓▓▓█████████████▓▓▓█████▓▓█▓█░░░█░█░░░ |
 | █████▓████▓▓▓██████████▓▓██▓██████▓▓████████████▓▓░░░▀▀▀░░░ |
 | ██████▓█▓▓███████████████████▓▓▓█▓▓▓██████████████░░░█▀▄░░░ |
 | ███████▓███████████████████▓▓████████▓▓▓██████████░░░█▄▀░░░ |
 | ▓▒▓▓████▓█████████████████▓█████████████▓▓▓▓▓▓████░░░▀░▀░░░ |
 | ░░░▒▒▓▓▓▓█▓▓█████████████▓████████████▓▓▓▒▒░░▒▒▓▓▓░░░█▖█░░░ |
 |   ░░░▒▒▓▓███▓███████████████████████▓▓▒▒░░    ░▒▒▓░░░█▝█░░░ |
 |       ░▒▓████▓▓█▓▓▓███▓▓▓▓▒▓▓█████▓▒░             ░░░▀░▀░░░ |
 |        ░▒▓▓▓▒▒  ░▒▒▓▓▓██▓░  ░▒▓▓▒░ ░    ░         ░░░▄▀█░░░ |
 |         ░░  ░     ▒▓███▓▓░             ░          ░░░█▀█░░░ |
 |                    ▒▓▓▓██▒                        ░░░▀░▀░░░ |
 |                     ░▓██▓▓                        ░░░█▀▄░░░ |
 |                      ░▓▓█▓░                       ░░░█░█░░░ |
 |                       ▒▓██▒░▒                     ░░░▀▀▀░░░ |
 |                     ░▒░▒▓█▓▒░░                    ░░░█▀█░░░ |
 |                       ░░▒▓█▓▒░░                   ░░░█░█░░░ |
 | ██████████████████████████████████████████████████░░░▀▀▀░░░ |
 | ██████████████████████████████████████████████████░░░░░░░░░ |
 | ██████████████████████████████████████████████████░░░░░░░░░ |
 +-------------------------------------------------------====***/

use tornado_cc::*;

// TODO: Refactor this whole file.
fn main() {
   // TODO: Actually use clap
   let args = std::env::args().collect::<Vec<String>>();
   let source = match args.get(1) {
      Some(arg) => arg.as_bytes(),
      _ => panic!("A file must be provided.")
   };

   let mut diagnostics = util::diag::Diagnostics::new();
   
   let mut lexer = front::lex::Lexer::new(source, &mut diagnostics);
   lexer.lex();
   let tokens = lexer.get_tokens();
   println!("{:#?}", tokens);
   dbg!(diagnostics);
}