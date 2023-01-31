#![allow(non_snake_case, unused_variables, dead_code)]

use modinverse::egcd;
use num::{integer::lcm, ToPrimitive};
use num_primes::Generator;

#[derive(Debug)]
struct RSA {
    p: u32,
    q: u32,
    n: u32,
    totient: u32,
    e: u32,
    d: u32,
}

impl RSA {
    pub fn new() -> RSA {
        let p: u32 = 73;//RSA::rand_prime();
        let q: u32 = 103;//RSA::rand_prime();
        let n: u32 = p * q;
        let totient: u32 = RSA::prime_carmichaels_totient(&p, &q);
        let e: u32 = 29;//RSA::compute_e(&totient);
        let d: u32 = RSA::compute_private_key(&e, &totient);
        RSA { 
            p: (p), 
            q: (q), 
            n: (n), 
            totient: (totient),
            e: (e), 
            d: (d),
        }
    }

    fn compute_private_key(e: &u32, totient: &u32) -> u32 {
        let (_, e, _) = egcd(*e, *totient);
        return e % totient; 
    }

    fn compute_e(totient: &u32) -> u32 {
        let e = &RSA::rand_prime();
        while (totient % e == 0) | (e > totient) {
            let e = &RSA::rand_prime();
        }
        return *e;
    }

    fn prime_carmichaels_totient(p: &u32, q: &u32) -> u32 {
        lcm(p - 1, q - 1)
    }

    fn rand_prime() -> u32 {
        match Generator::new_prime(8).to_u32() {
            Some(prime) => prime,
            None => panic!()
        }
    }

    pub fn encrypt(m: &u32, (n, e): &(u32, u32)) -> u32 {
        (m.pow(*e)) % n
    }

    pub fn decrypt(&self, c: &u32) -> u32 {
        (c.pow(self.d)) % self.n
    }

    pub fn get_public_key(&self) -> (u32, u32) {
        (self.n, self.e)
    }
}

fn main() {
    let some_client: RSA = RSA::new();
    let public_key: (u32, u32) = some_client.get_public_key();
    let message: u32 = 65;
    let encrypted = RSA::encrypt(&message, &public_key);
    let decrypted = some_client.decrypt(&encrypted);
    
    dbg!(&some_client);
    println!("Message: {:?},  Encrypted: {:?}, Decrypted: {:?}", &message, &encrypted, &decrypted);
    assert_eq!(&message, &decrypted);
}