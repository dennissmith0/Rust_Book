fn main() {
    let mut counter = 0;

    let result = loop {
	counter += 1;

	if counter == 11 {
	    break counter * 2;
	}
    };
    
    println!("the result is {result}");
}
