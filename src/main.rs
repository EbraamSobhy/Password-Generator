use rand::Rng;
use rand::seq::SliceRandom;
use std::io;

fn main() {
    fn random_char(chars: &[u8], rng: &mut impl Rng) -> char {
        chars[rng.random_range(0..chars.len())] as char
    }

    let uppercase = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let lowercase = b"abcdefghijklmnopqrstuvwxyz";

    let numbers = b"0123456789";

    let special = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

    let mut all = Vec::new();

    all.extend_from_slice(uppercase);
    all.extend_from_slice(lowercase);
    all.extend_from_slice(numbers);
    all.extend_from_slice(special);

    let length = 16;

    let mut rng = rand::rng();

    let mut password = vec![
        random_char(uppercase, &mut rng),
        random_char(lowercase, &mut rng),
        random_char(numbers, &mut rng),
        random_char(special, &mut rng),
    ];

    for _ in 4..length {
        password.push(random_char(&all, &mut rng));
    }

    password.shuffle(&mut rng);

    let mut password: String = password.into_iter().collect();

    println!("Password Generator Enter the length of the password");

    io::stdin().read_line(&mut password).unwrap();

    println!("Your Password: {}", password.trim());
}
