use std::io;

fn main() {
    println!("Hello, world JM here!");
    let mut x: i32 = -7;
    println!("{x}");
    x = 9;
    println!("{x}");
    const JM: &str = "me";
    println!("{JM}");

    let test_tup = (400, 32, 81);
    
    let (x, y, z) = test_tup;
    println!("This is tuple x {x}");
    println!("This is tuple y {y}");
    println!("This is tuple z {z}");

    let tup1 = test_tup.0;
    println!("Access through period {tup1}");
    println!("Access through period in tuple {}", {test_tup.1});

    let months_arr = ["Jan", "Feb"];
    println!("Months of the year {}", months_arr[0]);

    let repeated_months = ["March"; 4];
    println!("{}", repeated_months[0]);
    println!("{}", repeated_months[1]);
    println!("{}", repeated_months.len());

    let data = another_function("JM");
    println!("{}", data);

    println!("{}", convert_farenheight_to_celcius(44.5));

    println!("Please enter the sequence you want to get from fibonacci:");
    let mut input = String::new();
    let fibonacci_seq = match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Successfully read input, now parse it as a number
            match input.trim().parse::<u32>() {
                Ok(number) => {
                    number
                }
                Err(_) => {
                    return;
                }
            }
        }
        Err(_error) => {
            return;
        }
    };
    println!("Fibonacci value from sequence {fibonacci_seq} is {}", generate_fibonacci_sequence(fibonacci_seq));

    print_12_days_of_xmas_lyrics();
}

fn another_function(x: &str) -> i32 {
    println!("Another function. {}", x);
    let y = 5 + 9;

    y
}

// Function to convert fahrenheight to celcius
fn convert_farenheight_to_celcius(fahrenheight_temp: f64) -> f64 {
    let conversion_farenheight_to_celcius = (fahrenheight_temp - 32.0) * 5.0 / 9.0;

    conversion_farenheight_to_celcius
}

// Function to return the number from a fibonacci sequence specified
fn generate_fibonacci_sequence(sequence: u32) -> u64 {
    let mut current = 0;
    let mut next = 1;
    let mut result = 0;
    let mut number = sequence - 1;

    while number != 0 {
        number -= 1;
        result = current + next;
        current = next;
        next = result;
    };

    result
}

// Print the 12 days of xmas lyrics
fn print_12_days_of_xmas_lyrics() {
    const FIRST_DAY: &str = "A partridge in a pear tree.";
    const SECOND_DAY: &str = "Two turtle doves, And";
    const THIRD_DAY: &str = "Three French hens,";
    const FOURTH_DAY: &str = "Four calling birds,";
    const FIFTH_DAY: &str = "Five golden rings,";
    const SIXTH_DAY: &str = "Six geese a-laying,";
    const SEVENTH_DAY: &str = "Seven swans a-swimming,";
    const EIGTH_DAY: &str = "Eight maids a-milking,";
    const NINTH_DAY: &str = "Nine ladies dancing,";
    const TENTH_DAY: &str = "Ten lords a-leaping,";
    const ELEVENTH_DAY: &str = "Eleven pipers piping,";
    const TWELFTH_DAY: &str = "Twelve drummers drumming,";
    const LYRICS_ARR: [&str; 12] = [FIRST_DAY, SECOND_DAY, THIRD_DAY, FOURTH_DAY, FIFTH_DAY, SIXTH_DAY, SEVENTH_DAY, EIGTH_DAY, NINTH_DAY, TENTH_DAY, ELEVENTH_DAY, TWELFTH_DAY];

    for (index, _item) in LYRICS_ARR.iter().enumerate() {
        let day = index + 1;
        println!("On the {day} day of Christmas, my true love gave to me");
        for i in (0..=index).rev() {
            println!("{}", LYRICS_ARR[i]);
        }
    }
}