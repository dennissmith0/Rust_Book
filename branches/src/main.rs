fn main() {
    let number = 6;

    if number % 4 == 0 {
	    println!("number is divisible by 4")
    } else if number % 3 == 0 {
	    println!("number is divisible by 3")
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
	    println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("the value of number is: {number}");

    let x = 1;
    let y = if x > 0 { 0 } else { 1 };
    println!("{y}");
}
