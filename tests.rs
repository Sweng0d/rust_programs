error[E0573]: expected type, found module `self`
PS C:\Users\achil\OneDrive\Área de Trabalho\projects\bank> cargo test
   Compiling bank v0.1.0 (C:\Users\achil\OneDrive\Área de Trabalho\projects\bank)
error[E0573]: expected type, found module `self`
  --> src\lib.rs:21:33
   |
21 |     pub fn transfer (from: &mut self, to: &mut self, amount: i32) {
   |                                 ^^^^ help: a self type with a similar name exists (notice the capitalization): `Self`

error[E0573]: expected type, found module `self`
  --> src\lib.rs:21:48
   |
21 |     pub fn transfer (from: &mut self, to: &mut self, amount: i32) {
   |                                                ^^^^ help: a self type with a similar name exists (notice the capitalization): `Self`

For more information about this error, try `rustc --explain E0573`.
error: could not compile `bank` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `bank` (lib test) due to 2 previous errors
PS C:\Users\achil\OneDrive\Área de Trabalho\projects\bank>
