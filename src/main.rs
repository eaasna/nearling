use std::io::{stdout, Write};

struct PublicConstants{
    n : u32,
    g : u32,
}

struct User{
    public_key : u32,
    private_key : u32,
}

fn user_mod_arithmetic( private_key : u32, pub_const : &PublicConstants ) -> u32 {
    return pub_const.g.pow(private_key) % pub_const.n;
}

fn make_user( user_key : u32, pub_const : &PublicConstants ) -> User {
    
    return User{ private_key : user_key, public_key : user_mod_arithmetic(user_key, pub_const) }
}

struct Message{
    secret : u32,
    private_text : String, // data owned by each message
    // private_text : &'static str,    // use a string that has a static lifetime -> the whole runtime of the program
}

impl User{
    
    fn make_shared_secret(&self, other_half_mod : u32, n : u32) -> u32 {
        return other_half_mod * self.private_key % n;
    }

    fn make_message_for_other(&self, half_mod : u32, pub_n : u32, text : &str) -> Message {
        return Message { secret : self.make_shared_secret(half_mod, pub_n), 
                         private_text : text.to_string() };
    }
}

fn force_guess(a : u32, b : u32, n : &u32, g : &u32) -> u32 {
    return g.pow(a * b) % n;
}   


impl Message{
    fn try_unlock(&self, private_a : u32, private_b : u32, pub_const : &PublicConstants) {
        if force_guess(private_a, private_b, &pub_const.n, &pub_const.g) == self.secret {
            println!(self.message);
        }
    }
}


fn main() {
    let pub_const = PublicConstants{ g: 7, n: 1001 };
    
    let alice : User = make_user(10, &pub_const);
    let bob : User = make_user(13, &pub_const);
    let mes = "Hi Bob!".to_string();
    let message_for_bob = alice.make_message_for_other(bob.public_key, pub_const.n, &mes);

    println!("{}", mes);

    for i in 1..pub_const.n {
        for j in 1..pub_const.n {
            message_for_bob.try_unlock(i, j, &pub_const);
        }
    }
}
