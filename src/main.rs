//use std::io;
use rand::{self, Rng};

fn main() {
    let prime: u128 = 727*521;
    let random_number:u128 = rand::thread_rng().gen_range(1..=prime/2);

    let mut working_guess: u128 = random_number.clone();
    let guess: u128 = working_guess.clone();
    let mut power: u32 = 2;
    while working_guess % prime != 1{
        working_guess = u128::pow(working_guess,power);
        power +=1;
    }

    let mut number_a: u128 = prime.clone();
    let mut number_b: u128 = u128::pow(guess, power/2)+1;
    let mut total: bool = true;

    while total {
        if number_a > number_b {
            number_a = number_a % number_b;
            if number_a == 0 {
                get_primes_from_guess(prime, number_b);
                total = false;
            }
        } else if number_b > number_a {
            number_b = number_b % number_a;
            if number_b == 0 {
                get_primes_from_guess(prime, number_a);
                total = false;
            }
        }
    }
}

fn get_primes_from_guess(prime: u128,guess: u128) {
    let prime1: u128 = prime/guess;
    let prime2: u128 = prime/prime1;

    if prime == prime1 {
        main()
    } else if prime == prime2 {
        main()
    }

    println!("The prime numbers are {}, {}", prime1, prime2)
}