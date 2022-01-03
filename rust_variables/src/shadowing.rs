pub fn shadowing() {
    // shadowing in rust
    let a = 17;
    println!("a = {}", a); // must be 17

    let a = a*2;
    println!("a = {}", a); /*here we shadow the initial variable a and its value 17 with the new variable a and its value of 
                                "a*2"(here a represents the value of the initial variable a that is 17)*/ // output must be 34
    {
        let a = a+1; // now this third a shadows the previous 2 a's and a here in a+1 indicates the value of second a variable that is 17*2 = 34. and once this inner scope ends, the a value returns to being 34;
        println!("a = {}", a); // output must be 35
    }
    println!("a = {}", a); // here the value of a will be 34 as the inner scope has ended. output must be 34
}