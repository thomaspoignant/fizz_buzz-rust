#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod fizz_buzz;

use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

#[get("/fizzbuzz?<string1>&<string2>&<int1>&<int2>&<limit>")]
fn buzz(string1: String, string2: String, int1: i32, int2: i32, limit: i32) -> JsonValue {
    let fizz_buzz = fizz_buzz::fizz_buzz(string1, string2, int1, int2, limit);

    if fizz_buzz.is_err() {
        let error = fizz_buzz.err().unwrap();
        return json!({
            "error": error
        })
    }

    let res = fizz_buzz.ok().unwrap();
    return json!({
        "values": res
    })
}


fn main() {
    rocket::ignite().mount("/v1", routes![buzz]).launch();

}
