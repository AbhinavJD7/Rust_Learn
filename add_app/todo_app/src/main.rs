use commons::User;

fn main(){
    let u:User = User{
        username:String::from("Abhinav"),
        password:String::from("1234")
    }
    println!("{:?}", u);
}