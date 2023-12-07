const ANSWER_TO_LIFE_THE_UNIVERSE_AND_EVERYTHING: i32 = 42;

fn main() {
    let x = ANSWER_TO_LIFE_THE_UNIVERSE_AND_EVERYTHING;
    let x = x + 1;
    {
        let x = 6;
        println!("The value of x is {x}");
    }
    println!("The value of x is {x}");

    /* My own curiousity:
     * Does this shadowing work in a loop like this?
     * No, it does not.
     * The new x only exists inside the body of the loop.
     * It does NOT exist as part of the loop condition,
     * "x < 44". Therefore, this is an infinite loop.
    while x < 44 {
        let x = x + 1;
    }
    println!("The value of x is {x}");
    */

    /*
     * While the y here is mutable as we would expect.
     */
    let mut y = 0;
    while y < 2 {
        println!("The value of y is {y}");
        y = y + 1;
    }
        
}
