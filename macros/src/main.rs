//////////////////////
//////////////////////
////////      declarative macros        ////////
//////////////////////
/////////////////////
///

// boiler plate for declarative
// macros_rules! rule_name{
//     // yeah expression matching ki trh hai bracket mein expression ayega and uske respective code run hoga 
//     () => {}
// }
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

macro_rules! double {
    ($x: expr) =>{
        $x * 2
    };
}
fn main() {
    say_hello!();
    println!("double is {}", double!(10));
}



//////////////////////
//////////////////////
////////      procedural macros        ////////
//////////////////////
/////////////////////
///

// we have to make a seperate create for this


// 1) Derive macros
#[derive(Debug, Clone)]
struct User {
    name: String,
}

// which in internal is doing
// Automatically generates:
// Debug implementation
// Clone implementation


// 2) Attribute macros
// We write
// #[log]
// fn login(){
//     println!("user logged in");
// }
// fn login() {
//     println!("Entering function");
//     println!("user logged in");
//     println!("Exiting function");
// }

// // actual impl of log function
// use proc_macro::TokenStream;

// #[proc_macro_attribute]
// pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     // code that modifies your function
// }

// // one more example
// #[timer]
// fn process() {
//     println!("doing work");
// }

// // macros converts it into
// fn process() {
//     let start = now();
//     println!("doing work");
//     println!("Time taken: {}", now() - start);
// }


// 3) Functional macros

// Setup for creating proc macros 
// 1) create new crate for macros only
// cargo new my_macros --lib

// 2) update cargot.toml file here 

// [lib]
// proc-macro = true

// 3) Add dependencies
// [dependencies]
// syn = "2"
// quote = "1"

// 4) Write macros 
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, punctuated::Punctuated, Expr, Token};

// #[proc_macro]
// pub fn my_sum(input: TokenStream) -> TokenStream {
//     // Parse input: 1, 2, 3, 4
//     let exprs = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);

//     // Build output: 1 + 2 + 3 + 4
//     let expanded = quote! {
//         0 #( + #exprs )*
//     };

//     TokenStream::from(expanded)
// }

// 5) use it in another project
// cargo new app

// 6) Add dependency in Cargo.toml
// [dependencies]
// my_macros = { path = "../my_macros" }


// 7) use in code
// use my_macros::my_sum;

// fn main() {
//     let result = my_sum!(1, 2, 3, 4);
//     println!("{}", result);
// }
