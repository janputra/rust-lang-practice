#[derive(Debug)]
enum Message{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
impl Message{
    fn call(&self){
        println!("The enum is : {:?}",self)
    }
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

impl UsState{
    fn existed_in(&self, year: u16) -> bool{
        match self{
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


impl Coin{
    fn value_in_cent(&self)->u8{
        match self{
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(_) => 25,
        }
    }
    fn describe_state_quarter(&self) -> Option<String>{
       let Coin::Quarter(state) = self else{
            return None;
       }; 
       if state.existed_in(1900){
            Some(format!("{state:?} is pretty old, for America!"))
       }else{
            Some(format!("{state:?} is relatively new."))
       }
    }

}

fn main() {
    let x = Message::Write(String::from("hello"));
    x.call();
    
    let c = Coin::Penny;
    let c2 = Coin::Nickel;
    let c3 = Coin::Quarter(UsState::Alabama);
    let coins:[&Coin;3] = [&c,&c2,&c3];

    for item in coins{
        println!("Print the coint value {}",item.value_in_cent()); 
        if let Coin::Nickel = item{  // used to check only 1 condition
            println!("Using if let to check if item is Nickel");
        }
        if let Some(desc) = item.describe_state_quarter(){  // only runs when it's Some
            println!("{desc}");
        }
    }
}
