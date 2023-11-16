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
   //let args = std::env::args().collect::<Vec<String>>();
   //let source = match args.get(1) {
   //   Some(arg) => std::fs::read_to_string(arg).unwrap(),
   //   _ => panic!("A file must be provided.")
   //};

   let source = String::from("int main() {}");

   let mut diagnostics = util::diag::Diagnostics::new();
   
   let mut lexer = front::lex::Lexer::new(source.as_bytes(), &mut diagnostics);
   lexer.lex();
   let tokens = lexer.get_tokens();
   println!("{:#?}", tokens);

   // DEBUG TEST
   // TODO: Move this atrocity into a separate test
   // grab second token (in this case main)
   let a = match tokens[1].tokentype {
      front::lex::TokenType::IDENTIFIER(ref ident) => Some(ident),
      _ => None
   }.unwrap();

   // This is not going to look good at the pearly gates
   // get the identifier and convert &[u8] to str
   // pretty sure there's an easier way to do this
   println!("{:#?}", source.as_bytes().get(a.start..a.start + a.size)
                                      .unwrap()
                                      .iter()
                                      .map(|f| *f as char)
                                      .collect::<Vec<char>>());

   let mut numpar = util::num::NumberParser::new(b"0xDEADBEEF", &mut diagnostics);
   numpar.num();
   let mynum = numpar.get_num();

   // Number parser convalidation
   dbg!(mynum);
   println!("{}", 0xDEADBEEF);

   dbg!(diagnostics);
}