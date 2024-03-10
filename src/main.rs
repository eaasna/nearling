use rand::prelude::*;

struct PublicConstants{
    n : u128,
    g : u32,
}

struct User{
    public_key : u128,
    private_key : u128,
}

fn user_mod_arithmetic( private_key : u128, pub_const : &PublicConstants ) -> u128 {
    println!("User Key pair ({},{})", private_key, u128::from(pub_const.g.pow(private_key.try_into().unwrap())) % pub_const.n);
    return u128::from(pub_const.g.pow(private_key.try_into().unwrap())) % pub_const.n;
}

fn make_user( user_private_key : u128, pub_const : &PublicConstants ) -> User {
    
    return User{ private_key : user_private_key, public_key : user_mod_arithmetic(user_private_key, pub_const) }
}

struct Message{
    secret : u128,
    private_text : String, // data owned by each message
    // private_text : &'static str,    // use a string that has a static lifetime -> the whole runtime of the program
}

impl User{
    
    fn make_shared_secret(&self, other_public_key : u128, pub_const : &PublicConstants) -> u128 {
        let mut secret = other_public_key;
        for i in 1..self.private_key {
            secret = secret * u128::from(pub_const.g) % pub_const.n;
        }
        return secret;
    }

    fn make_message_for_other(&self, half_mod : u128, pub_const : &PublicConstants, text : &str) -> Message {
        println!("Made shared secret {}", self.make_shared_secret(half_mod, &pub_const));
        return Message { secret : self.make_shared_secret(half_mod, &pub_const), 
                         private_text : text.to_string() };
    }
}

fn force_guess(a : u128, b : u128, n : &u128, g : &u32) -> u128 {
    let mut guess : u128 = u128::from(*g);
    for _i in 1..(a*b) {
        guess = guess * u128::from(*g) % n;
    }
    println!("Try keys {} and {}. Guessed {}", a, b, guess);
    return guess;
}   


impl Message{
    fn try_unlock(&self, private_a : u128, private_b : u128, pub_const : &PublicConstants) -> bool {
        if force_guess(private_a, private_b, &pub_const.n, &pub_const.g) == self.secret {
            println!("Unlocked message with keys {} and {}", private_a, private_b);
            println!("{}", self.private_text);
            return true;
        }
        else {
            return false;
        }
    }
}


fn main() {
    let pub_const = PublicConstants{ g: 7, n: 101 };
    
    let mut rng = thread_rng();
    let limit = 5;
    let alice : User = make_user(rng.gen_range(1..limit), &pub_const);
    let bob : User = make_user(rng.gen_range(1..limit), &pub_const);
    let mes = "Hi Bob!".to_string();
    let message_for_bob = alice.make_message_for_other(bob.public_key, &pub_const, &mes);

    'outer: for i in 1..limit {
        for j in i..limit {
            if message_for_bob.try_unlock(i, j, &pub_const) {
                break 'outer;
            }
        }
    }
}
