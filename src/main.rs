fn main() {
    //Rust has three kinds of loops: loop, while, and for. Let’s try each one.
    //The loop keyword tells Rust to execute a block of code over and over again 
    //forever or until you explicitly tell it to stop.
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    // The result is 20


    // loop label example

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // the loop label allows us to specify which loop we want to break out of  

    // conditional loop with while
    let mut number = 3;
    while number !=0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // write this in a for loop instead
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // here we use the range operator to generate all numbers in sequence starting from one number and ending before another number.
    // and then we use the rev method to reverse the collection
    // the rev method is defined on the Iterator trait, which is implemented for the range type


    // This construct eliminates a lot of nesting that would be necessary if
    // you used loop, if, else, and break, and it’s clearer. 
    //While a condition holds true, the code runs; otherwise, it exits the loop.

    // loop through a collection with for

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // The for loop is more concise and readable than a while loop in this situation.
    // The for loop takes ownership of a collection, such as an array, and iterates over
    // each element. In the example, we use a for loop to print the values in the array.
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
