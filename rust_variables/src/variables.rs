pub fn variables() {
    /* 
        INFO: Variables in rust by default are immutable. But we can make variables in rust mutable by adding the keyword mut in front of the variable name.
              Constants in rust are very similar to variables in rust. but the major difference is, variables are immutable by default(means they can be changed if we want to). Whereas, constants are always immutable and can't be changed.
              Constants can only be set to values that are NOT inferred during runtime. 
    */

    // how immutability works in rust
    let var_a = 5;
    println!("var_a = {}", var_a);
    // var_a = 6;
    // println!("var_a = {}", var_a); // code snippet will return an error

    // to make variables mutable do this
    let mut var_b = 30;
    println!("var_b = {}", var_b);
    var_b = 31;
    println!("var_b = {}", var_b); // whereas this code snippet will print the statements with the respective values

    // constants in rust
    const RUST_IS_POG: bool = true;
    println!("RUST_IS_POG = {}", RUST_IS_POG);
}