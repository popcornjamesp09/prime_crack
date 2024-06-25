extern crate num_bigint;
extern crate num_traits;

use num_traits::{Zero, One};
use rand::{self, Rng};
use std::env;

use num_bigint::BigInt;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: prime_crack <prime1> <prime2>");
        println!("       prime1  - The first prime number that you want");
        println!("       prime2  - The second prime number that you want");
        println!("This program finds the two prime numbers provided after multiplying them together.");
        panic!("Read usage")
    }
    
    let arg_prime_1: u128 = args[1].trim().parse().expect("Enter a number");
    let arg_prime_2: u128 = args[2].trim().parse().expect("Enter a number");

    // Calculates the number that will be cracked
    let range: u128 = arg_prime_1*arg_prime_2; 
    let guess: BigInt = BigInt::from(rand::thread_rng().gen_range(1..=(&range/2)));

    let prime: BigInt = BigInt::from(range);

    
    let mut power: u32 = 2;
    
    // Raises the random number to a power and keeps going until the remainder is one
    while BigInt::pow(&guess, power) % &prime != BigInt::one() {
        power +=1;
    }

    let number_b: BigInt = BigInt::pow(&guess, power/2)+1;

    find_gcl(prime.clone(), prime, number_b);
}

// Finds the GCD using Euclidean's Algorithm 
fn find_gcl(prime: BigInt, mut number_a: BigInt, mut number_b: BigInt) {
    loop {
        if number_a > number_b {
            number_a = number_a % number_b.clone();
            if number_a == BigInt::zero() {
                get_primes_from_guess(prime, number_b);
                break
            }
        } else if number_b > number_a {
            number_b = number_b % number_a.clone();
            if number_b == BigInt::zero() {
                get_primes_from_guess(prime, number_a);
                break
            }
        }
    }
    
}

fn get_primes_from_guess(prime: BigInt, guess: BigInt) {
    let prime1: BigInt = &prime/guess;
    let prime2: BigInt = &prime/&prime1;

    if &prime == &prime1 {
        main()
    } else if &prime == &prime2 {
        main()
    }

    println!("The prime numbers are {}, {}", prime1, prime2)
}