# Password Generator (Rust)

A simple command-line password generator written in Rust. It creates strong, randomized passwords using uppercase letters, lowercase letters, numbers, and special characters.

## Features

- Generates secure random passwords
- Includes:
    - Uppercase letters (A–Z)
    - Lowercase letters (a–z)
    - Numbers (0–9)
    - Special characters (!@#$%^&* etc.)
- Ensures at least one character from each category
- Shuffles characters for better randomness

## Requirements

- Rust (latest stable recommended)
- Cargo (comes with Rust)

Install Rust via:

```
https://www.rust-lang.org/tools/install
```

## How to Run

Clone or download the project, then run:

```
cargo run
```

## How It Works

1. The program defines character sets:
    - Uppercase letters
    - Lowercase letters
    - Numbers
    - Special characters
2. It guarantees the password contains at least one character from each set.
3. It fills the remaining length with randomly selected characters.
4. Finally, it shuffles the password to avoid predictable patterns.

## Example Output

```
Your Password: A9!kP2@xL0z#QmT1
```

## Notes

- The password length is currently fixed at **16 characters** in the code.
- The program reads input using `stdin`, but the input is not used to change password length (can be improved).

## Possible Improvements

- Allow user-defined password length
- Add CLI arguments (e.g., `-length 20`)
- Let users enable/disable character types
- Improve input handling (currently overwrites password variable)

## License

This project is free to use and modify.
