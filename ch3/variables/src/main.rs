fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    println!("Hello, world!");

    let value = 5;
    let value = value + 1;
    {
        let value = value * 2;
        println!("The value of value in the inner scope is: {}", value);
    }

    println!("The value of value in the outer scope is: {}", value);

    another_function(5);
    println!("The value of five function: {}", five());
    println!("The value of plus_one function: {}", plus_one(5));

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The element is {}", element);
    }
}

fn another_function(x: i32) {
    println!("The value of x is {} in another function", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
