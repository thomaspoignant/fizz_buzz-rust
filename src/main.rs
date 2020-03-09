#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod fizz_buzz;

use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use rocket::response::status;
use rocket::http::Status;

#[get("/fizzbuzz?<string1>&<string2>&<int1>&<int2>&<limit>")]
fn buzz(string1: String, string2: String, int1: i32, int2: i32, limit: i32)
    -> Result<status::Custom<JsonValue>, Status> {
    let fizz_buzz = fizz_buzz::fizz_buzz(string1, string2, int1, int2, limit);

    if fizz_buzz.is_err() {
        let error = fizz_buzz.err().unwrap();
        return Ok(status::Custom(Status::BadRequest, json!({
            "error": error
        })));
    }

    let res = fizz_buzz.ok().unwrap();
    return Ok(status::Custom(Status::Ok, json!({
            "values": res
        })));
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/v1", routes![buzz])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn valid_fizz_buzz() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response =
            client.get("/v1/fizzbuzz?string1=fizz&string2=buzz&int1=3&int2=5&limit=15")
                .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("{\"values\":[\"1\",\"2\",\"Fizz\",\"4\",\"Buzz\",\"Fizz\",\"7\",\"8\",\"Fizz\",\"Buzz\",\"11\",\"Fizz\",\"13\",\"14\",\"FizzBuzz\"]}".into()));
    }

    #[test]
    fn error_fizz_buzz() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response =
            client.get("/v1/fizzbuzz?string1=fizz&string2=buzz&int1=3&int2=5&limit=0")
                .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.body_string(), Some("{\"error\":\"InvalidParameters: Limit should be greater than 1.\"}".into()));
    }
}
