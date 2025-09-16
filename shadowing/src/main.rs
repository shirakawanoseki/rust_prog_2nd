fn main() {

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("This value of x in the inner scope is: {x}");
    }

    let guess = "42".parse().expect("Not a number!");

    println!("The value of x is:{x}");
}
