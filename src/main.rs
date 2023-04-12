use collections::{vectors::vectors_main, strings::strings_main, hash_maps::maps_main};
use enums::options::options_main;
use error_handling::errors::main_errors;
use generics::generics_main;
use slices::slices_main;
use structs::{struct_main, example::main_structs_example};

mod data_types;
mod functions;
mod control_flow;
mod ownership;
mod references;
mod slices;
mod structs;
mod enums;
mod collections;
mod error_handling;
mod generics;

fn main() {
    
    // ownership_ie(); 
    // ownership_n_funcs();
    // return_values_n_scope();
    // take_n_return();
    // take_n_return();
    // dangle_example();
    // let mut s = String::from("hello word");
    // let word = first_words(&s);
    // slices_main();
    // struct_main();
    // main_structs_example();
    // enums_main();
    // options_main();
    // vectors_main();
    // match_main();
    // strings_main();  
    // maps_main()
    // main_errors(); 
    generics_main();
}
