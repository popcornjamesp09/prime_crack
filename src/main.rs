use rand::{self, Rng};
use std::env;

// Why are you here?

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: prime_crack <prime1> <prime2>");
        println!("       prime1  - The first prime number that you want");
        println!("       prime2  - The second prime number that you want");
        println!("This program finds the two prime numbers provided after multiplying them together.");
    }
    
    let arg_prime_1: u128 = args[1].trim().parse().expect("Enter a number");
    let arg_prime_2: u128 = args[2].trim().parse().expect("Enter a number");

    // Calculates the number that will be cracked
    let prime: u128 = arg_prime_1*arg_prime_2; 
    let guess:u128 = rand::thread_rng().gen_range(1..=prime/2);
    
    let mut power: u32 = 2;
    
    // Raises the random number to a power and keeps going until the remainder is one
    while guess.checked_pow(power).expect("Overflow") % prime != 1{
        power +=1;
    }

    let number_b: u128 = u128::pow(guess, {power/2}.try_into().unwrap())+1;

    find_gcl(prime, prime, number_b);
}

// Finds the GCD using Euclidean's Algorithm 
fn find_gcl(prime: u128, mut number_a: u128, mut number_b: u128) {
    loop {
        println!("Finding the gratest common factor if {} and {}", number_a, number_b);
        if number_a > number_b {
            number_a = number_a % number_b;
            if number_a == 0 {
                get_primes_from_guess(prime, number_b);
            }
        } else if number_b > number_a {
            number_b = number_b % number_a;
            if number_b == 0 {
                get_primes_from_guess(prime, number_a);
            }
        }
    }
}

fn get_primes_from_guess(prime: u128, guess: u128) {
    let prime1: u128 = prime/guess;
    let prime2: u128 = prime/prime1;

    if prime == prime1 {
        main()
    } else if prime == prime2 {
        main()
    }

    println!("The prime numbers are {}, {}", prime1, prime2)
}