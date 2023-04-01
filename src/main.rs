use slices::slices_main;
use structs::{struct_main, example::main_structs_example};

mod data_types;
mod functions;
mod control_flow;
mod ownership;
mod references;
mod slices;
mod structs;

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
    main_structs_example();
   
}
