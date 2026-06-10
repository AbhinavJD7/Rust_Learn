fn main() {
    //println!("{}" , longest_str(String::from("Abhinav"), String::from("Rai")));

    let str1:String = String::from("Abhinav");
    let str2:String = String::from("Rai");
    let longest = longest_str(&str1 , &str2);
    println!("{}" , longest) 
}

fn longest_str<'a>( first: & 'a String , second: & 'a String) -> &'a String{
    if first.len() > second.len() {
        first
    } else {
        second
    }
}


//ownership & Borrowing rules understanding
// fn main(){
//     let str1:String = String::from("Abhinav"); //str1 owns Abhinav on heap
//     let str2:String = &str1 // 1 immutable reference
//     let str3:String = return_reference(&str1) // 2nd immutable reference
// }

// fn return_reference(s:&String) -> String{
//     return s;
// }