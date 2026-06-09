use borsh :: {BorshSerialize, BorshDeserialize};

#[derive(BorshDeserialize,BorshSerialize,Debug)]
struct Address{
    state:String,
    city:String,
    is_capital:bool
}

fn main() {
    let address:Address = Address{
        state:String::from("UP"),
        city:String::from("Lucknow"),
        is_capital:true
    };

    let mut buffer:Vec<u8> =  Vec::new();
    //BorshSerialize
    let ans1 = address.serialize(&mut buffer).unwrap();
    println!("{:?}",buffer); // [2, 0, 0, 0, 85, 80, 7, 0, 0, 0, 76, 117, 99, 107, 110, 111, 119, 1]%
    //printing the struct
    print!("{:?}",address);
    //BorshDeserialize
    let ans2:Address = Address::try_from_slice(&mut buffer).unwrap();
    println!("{:?}",ans2);
}

