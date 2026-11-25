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

        let syöte: i32 = buf.trim().parse().expect("Virhe jäsentäessä syötettä");

        count += syöte;
    }
    println!("Enough incrementations.");
}
