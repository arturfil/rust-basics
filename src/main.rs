use collections::vectors::vectors_main;
use enums::options::options_main;
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
    vectors_main();
    // match_main();

}
