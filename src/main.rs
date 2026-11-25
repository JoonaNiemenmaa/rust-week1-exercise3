use std::io;

fn main() {
    println!("By how much do you want to increment the number?");

    let mut count: i32 = 0;

    while count < i32::from(i16::MAX) {
        println!("Current: {}. Increment by: ", count);

        let mut buf = String::new();

        io::stdin()
            .read_line(&mut buf)
            .expect("Virhe luetessa komentokehotetta");

        let syöte: i16 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parsing failed. Was the number too long for a 16-bit variable?");
                continue;
            }
        };

        if syöte < 0 {
            println!("The given value is lower than 0.");
            continue;
        }

        if syöte == 0 {
            println!("The given value is 0. Ending the program.");
            return;
        }

        count += i32::from(syöte);
    }
    println!("Enough incrementations.");
}
