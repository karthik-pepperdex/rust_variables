pub fn data_types() {
    /* data types in rust are a certain type of a variable. the data type of a certain variable is needed to be known by the compiler in order
        to work accordingly with it.

        variables in rust are broadly divided into 2 types: scalar and compound.

        scalar is further divided into: integer types, floating point types, boolean types and character types.
    */

    // integer type - integer type in rust is a number without a decimal part.
    let int1 = 1; // if you dont specify the data type for an integer number, rust by default assumes it as an i32.

    /* integer types in turn are divided into 2 types 'u' and 'i'. a number can either be represented with a 'u' or an 'i'.  
        integers whose data types start with a 'u', are unsigned integers. they can only be positive.
        whereas integers whose data types start with an 'i', are signed integers. they can be positive or negative.
    */
    let int2: u16 = 3; // as the datatype is unsigned the value can only be positive
    let int3: i16 = -3; // as the datatype is signed the value can be positive or negative
    println!("integers: {} {} {}", int1, int2, int3);

    // floating point type - floating point type in rust is a number with a decimal part.
    let flt1 = 3.4; // if the data type in a floating point number is not mentioned, rust defaults to a f64 datatype.
    println!("floating point numbers: {}", flt1);

    // boolean type - boolean type in rust is a boolean(true / false) value.
    let bool1 = true;
    print!("boolean: {}", bool1);

    // character type - character type in rust is a single character and is the most primitive data type in rust.
    let char1 = 'a';
    println!("character: {}", char1);

    /* compound types - these types can group multiple values of different types into a variable */
    // tuple type - tuple type in rust is a group of multiple values of different types.
    let tup = (17, 3.1, true); // we can also specify the type of each value in a tuple.
    println!("tuples in rust: {:?}", tup);

    // specifying the type
    let tup1: (i32, f64, bool) = (35, 3.4, false);
    println!("tuples in rust by specifying the type of each variable: {:?}", tup1);
    // accessing the elements of a tuple
    let tupx = tup1.0;
    println!("accessing the elements of a tuple: {}", tupx);

    // array type - array type in rust is a group of multiple values of the same type.
    let arr = [1,2,3]; // when we dont specify the type rust infers the type and length and represents them in this manner [type; length].
    println!("array in rust: {:?}", arr);

    // specifying the type
    let arr1: [i32; 4] = [3,4,6,8];
    println!("array in rust by specifying the type of each variable: {:?}", arr1);
    // accessing the elements of an array
    let arrx = arr1[0];
    println!("accessing the elements of an array: {}", arrx);
}