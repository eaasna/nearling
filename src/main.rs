use std::io::{stdout, Write};

struct public_constants{
    n : u32,
    g : u32,
}

struct user{
    public_key : u32,
    private_key : u32,
}

fn user_mod_arithmetic( private_key : u32, pub_const : public_constants ) -> u32 {
    return pub_const.g.pow(private_key) % pub_const.n;
}

fn make_user( user_key : u32, pub_const : public_constants ) -> user {
    
    return user{ private_key : user_key, public_key : user_mod_arithmetic(user_key, pub_const) }
}

struct message{
    secret : u32,
    private_text : str,
}

impl user{
    
    fn make_shared_secret(&self, other_half_mod : u32, n : u32) -> u32 {
        return other_half_mod * self.private_key % n;
    }

    fn make_message_for_other(&self, other_half_mod : u32, n : u32, text : &str) -> message {
        return message { secret : self.make_shared_secret(other_half_mod = other_half_mod, n = n), 
                         private_text : text };
    }
}

fn force_guess(a : u32, b : u32, n : u32, g : u32) -> u32 {
    return g.pow(a * b) % n;
}   


//impl message{
//    fn try_unlock(&self, private_a : u32, private_b : u32, pub_const : public_constants) {
//        if (force_guess(private_a, private_b, pub_const.n, pub_const.g) == self.secret) {
//            println!(&self.text);
//        }
//    }
//}


fn main() {
    let pub_const = public_constants{ g: 7, n: 1001 };
    
    let alice : user = make_user(10, pub_const);
    let bob : user = make_user(13, pub_const);
    let mut mes = "Hi Bob!".to_string();
    let message_for_bob = alice.make_message_for_other(bob.public_key, pub_const.n, &mes);

    println!("{}", mes);

    //for i in 1..n {
    //    for j in 1..n {
    //        message_for_bob.try_unlock(private_a = i, private_b = j, pub_const);
    //    }
    //}
}
