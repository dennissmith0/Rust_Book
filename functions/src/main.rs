fn main() {
    another_function(5);

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("the value of y is: {y}");
}

fn another_function(x: i32) {
    println!("the value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}
