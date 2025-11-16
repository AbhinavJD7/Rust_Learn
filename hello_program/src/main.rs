fn main() {
    println!("Hello, world!");

    let num:u8 = 5;
    println!("The value stored in num is {}",num);
    //String - Dynamic Length String
    // &str - Fixed Length String - read-only data segment
    let mut string_literal:String  = String::from("Hi, Rust!");
    string_literal.push_str("What's up?");
    println!("This is string {}",string_literal);

    //Tupple
    let emp_info:(&str,u8) = ("Ramesh",50);
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;
    println!("The Employee name is {} and his age is {}",emp_name , emp_age);

    //destructuring

    let (employee_name , employee_age) = emp_info;
    println!("The Employee name is {} and his age is {}",employee_name , employee_age);

    print_value(5);
    let num1:u8 = 10;
    let num2:u8 = 20;
    add(num1,num2);
    println!("The product of {} and {} is {} " , num1 , num2 , mult(num1,num2));



//Functions

fn print_value(a:u8){
    println!("Sample Number = {}",a)
}

fn add(a:u8 , b:u8){
    println!("The sum of {} and {} is {}",a , b , a+b )
}

fn mult(a:u8 , b:u8)->u8{
    return a*b;
}


// Memory Management :
// Control First Approach :- C , C++ (in the hand of programmer)
// Safety First Approach :- Java , Python (Garbage Collector)





//signed Interger : i8, i16, i32, i64, i128, isize
//unsigned Interger : u8, u16, u32, u64, u128, usize
//floating point : f32, f64
//boolean : bool
//character : char
//compound types : tuples, arrays
//variable : immutable by default
//mutable variable : use 'mut' keyword
//constant : use 'const' keyword
//function : use 'fn' keyword


//snake_case : hello_world 
//camelcase : HelloWorld
//Cargo is package manager for Rust projects
//toml full form is "Tom's Obvious, Minimal Language"



//OWNERSHIP

let str1 = String::from("Hello"); //str1 is the owner of Hello
let str2 = str1; // Transfer of ownership now str2 is the new owner of Hello
//println!("str1={}",str1); // Thus this line will give error str1 now does not exist.
println!("str2={}",str2);

//Double free error

//Ownership Example#1

let s1:String = get_string(); //Transfer of ownership from new_string to s1
println!("This is s1:{}" , s1);

let s2:String = String::from("world"); //s2 is the owner of "world"
let s3:String = send_get_string(s2); // Transfer of ownership from received_string to s3 


println!("This is s3:{}",s3); //s3 is owner of "world"

fn get_string()->String{
    let new_string = String::from("hello");
    return new_string;
}

//transfer of ownership from s2 to received_string
fn send_get_string(received_string:String)->String{
    return received_string;
}


//Ownership Issue

let s1:String = String::from("hello"); //s1 owner
let len:usize = calculate_length(s1); //ownership transfer
//println!("The length of {} is {} ",s1 ,len); // Here is will cause issue.

fn calculate_length(s:String)->usize{ //s will be the new owner
    return s.len(); //return 5
}

//Avoiding Ownership using Tupple

let a:String = String::from("hello"); //a owner
let (b,len) = calculate_length2(a); //ownership transfer , new owner b
println!("The length of {} is {} ",b ,len); // Now here there will be no issue

fn calculate_length2(c:String)->(String,usize){ //c will be the new owner
    let length:usize = c.len();
    return (c,length);
}

//Avoiding Ownership using Cloning

let m:String = String::from("hello"); //s1 owner
let len:usize = calculate_length3(m.clone()); //ownership transfer
println!("The length of {} is {} ",m ,len); // Here is will cause issue.

fn calculate_length3(n:String)->usize{ //s will be the new owner
    return n.len(); //return 5
}

//Avoiding Ownership issue using Borrowing.
// We will not transfer we will borrow the value by passing the reference "&s1"

//in borrow operation any mutable operation cannot be performed using the referenced value which can hinder the orginal value

  

}





