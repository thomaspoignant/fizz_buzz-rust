mod fizz_buzz;

fn main() {
    let fizz_buzz = fizz_buzz::fizz_buzz(1, 100);

    if fizz_buzz.is_err() {
        println!("Error: {:?}", fizz_buzz.err().unwrap());
    } else {
        println!("FizzBuzz results :{:?}", fizz_buzz.ok().unwrap());
    }
}

