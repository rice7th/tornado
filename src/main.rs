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

use tornado_cc::{util::scan::Scanner, *};

// TODO: Refactor this whole file.
fn main() {
   // TODO: Actually use clap
   //let args = std::env::args().collect::<Vec<String>>();
   //let source = match args.get(1) {
   //   Some(arg) => std::fs::read_to_string(arg).unwrap(),
   //   _ => panic!("A file must be provided.")
   //};

   let source = String::from("float f = 10");

   let mut diagnostics = util::diag::Diagnostics::new();
   
   let mut lexer = front::lex::Lexer::new(source.as_bytes(), &mut diagnostics);
   lexer.lex();
   let tokens = lexer.get_tokens().clone();
   println!("{:#?}", tokens);

   let mut parser = front::par::Parser::new(&tokens, &mut diagnostics);

   let my_expr = parser.expression();
   dbg!(my_expr);

   /*
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
   */


  // TODO: Make a better compiler driver api
  // like
  // ```rust
  // let mut cc = tornado::CC::new(); // return compiler driver
  // cc.src("main.c").output("main")
  //    .include("./include/")
  //    .define("MY_MACRO")
  //    .opt(3) // opt_level 3 or custom
  //    .link("m") // math lib
  //    .static()
  //    .compile(); // compile
  // ```
  // So one can like actually compile more stuff in parallel by just using
  // rust thread.
}
