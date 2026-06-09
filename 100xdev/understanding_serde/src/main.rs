use serde::{Deserialize, Serialize};
use serde_json::Result;

// added Debug macro to print struct
#[derive(Serialize, Deserialize, Debug)]
struct Address{
    city:String,
    state:String,
    #[serde(rename = "pinCode")] //Attribute type macro
    pincode:String
}
#[derive(Serialize, Deserialize, Debug)]
struct SignupResponse{
    message:String,
    address:Address //using struct as a field of struct
}

fn main() {
    //Giving value of Address struct
    let a:Address = Address{
        city:String::from("Varansi"),
        state:String::from("UP"),
        pincode:String::from("221005")
    };

    //Giving value to struct which has to be converted to JSON
    let s:SignupResponse = SignupResponse {
        message:String::from("You are not signed in"),
        address:a
    };

    //converting struct to JSON (serialize)
    let json_str = serde_json::to_string(&s).unwrap();
    println!("{}" , json_str);

    //converting JSON to Struct (deserialize)
    let s2:Result<SignupResponse> = serde_json::from_str(&json_str);
    print!("{:?}",s2.unwrap())
}
