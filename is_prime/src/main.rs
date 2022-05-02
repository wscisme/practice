// use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::mem;
// use std::vec;

fn is_prime() {
    
    println!("Determin whether an integer (< 1.8e19) is a prime number");

    'input_loop: loop {
        print!("\nPlease input your number: ");
        io::stdout().flush().unwrap();

        let mut userin = String::new();

        io::stdin().read_line(&mut userin)
            .expect("Failed to read line");

        if userin.trim() == "exit" {
            return;
        }

        let num: u64 = match userin.trim().parse() {
            Ok(res) => res,
            Err(_) => continue,
        };

        if num % 2 == 0 {
            println!("Your input: {} is not a prime number! It can be divided by 2", num);
            continue;
        }

        let mut i: u64 = 3;

        while i*i <= num {
            if num % i == 0 {
                println!("Your input: {} is not a prime number! It can be divided by {}", num, i);
                continue 'input_loop;
            }
            i += 2;
        }

        println!("Your input: {} is a prime number!", num);
    }

}

fn find_primes() {
    
    loop {

        print!("\nPlease input the ranges: ");
        io::stdout().flush().unwrap();
        let mut userin = String::new();

        io::stdin().read_line(&mut userin)
            .expect("Failed to read line");

        if userin.trim() == "exit" {
            return;
        }

        let mut uin: Vec<&str> = userin.trim().split(' ').collect();
        if userin.contains('-') {
            uin = userin.trim().split('-').collect();
        }

        if userin.len() < 2 { continue; }

        let mut min: u64 = match uin[0].trim().parse() {
            Ok(res) => res,
            Err(_) => continue,
        };

        let mut max: u64 = match uin[1].trim().parse() {
            Ok(res) => res,
            Err(_) => continue,
        };

        if max < min {
            mem::swap(&mut min, &mut max);
        }

        let mut res: Vec<u64> = Vec::new();
        res.reserve(13);

        let mut num: u64 = min;
        if num % 2 == 0 {
            num += 1;
        }

        while num <= max {
            let mut is_prime = true;
            let mut i: u64 = 3;
            while i*i <= num {
                if num % i == 0 {
                    is_prime = false;
                    break;
                }
                i += 2;
            }
            if is_prime { res.push(num); }
            num += 2;
        }

        if res.len() == 0 {
            println!("There are no primes in range {} to {}.", min, max)
        } else {
            let percent: f32 = 100.0 * (res.len() as f32) / ((max - min + 1) as f32);
            if res.len() < 30 {
                println!("There are {} primes within the range ({:.2}%), they are:", res.len(), percent);
                for num in res {
                    println!("{}", num);
                }
            } else {
                print!("There are {} primes within the range ({:.2}%), show? [Y/N]: ", res.len(), percent);
                io::stdout().flush().unwrap();
                let mut uans = String::new();
                io::stdin().read_line(&mut uans).expect("");
                if uans.trim().to_lowercase() == "y" {
                    for num in res {
                        println!("{}", num);
                    }
                }
            }
        }
    }
}


fn main() {
    loop {
        print!("Input the function to be called [check|find]: ");
        io::stdout().flush().unwrap();

        let mut userin = String::new();
        io::stdin().read_line(&mut userin)
            .expect("Failed to read line");

        if userin.trim() == "check" {
            is_prime();
        } else if userin.trim() == "find" {
            find_primes();
        } else {
            println!("Your input {} does not match any known function of this program.", userin);
        }
    }
}
