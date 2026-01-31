#![allow(unused)]
//enum in struct
//\\\

//simple Enum
enum Word { Man, _Words}

#[derive(Debug)]
enum Message {
    _Quit,
    _Move { x:i32, y:i32},
    Write(String),
    _ChangeColor(i32,i32,i32),
}


impl Message {
    fn call(&self){
        
    }
}

//struct binidng 
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

impl UsState {
    fn existed_in(&self, year:u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn describle_state_quarter(coin:Coin) -> Option<String>{
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900){
            Some(format!("{state:?} is prety old for america"))
        }else {
            Some(format!("{state:?} is relatively new."))
        }
    }else {
        None
    }
}

fn new_describe(coin:Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };
    
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old for america"))
    }else {
        Some(format!("{state:?} is relatively new"))
    }
}

fn main(){
    let m = Message::Write(String::from("Hello"));
    m.call();
    
    println!("{:?}", (m));
    
    let _something =  Word::Man;
    
    let some_number = Some(5);
    
    let y:i8 = 5;
    let absent_number = Some(1044);
    
    // let sum = some_number + absent_number;
    
    
    //match control flow
    // is must be exhaustive of all the conditions 
    
    let value = value_in_cent(Coin::Quarter(UsState::Alabama));
    
    println!("value is {value}");
    
    if value > 1 {
        //do something
    }else {
        // do other thing 
    }
    
    fn plus_one(x:Option<i32>) -> Option<i32>{
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    
    let dice_roll = 9;
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => ()
    }
    
    let config_max = Some(3u8);
    
    //instead of all this boiler plate code 
    // match config_max {
    //     Some(max) => println!("The maximun is configured to be {max}"),
    //     _ => (),
    // }
    
    //we can use this instead
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    
    if let Some(desc) = new_describe(Coin::Quarter(UsState::Alaska)){
        println!("{desc}");
    }
}

fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn move_player(num_space:u8){}
fn reroll(){}

fn value_in_cent(coin:Coin) -> u8 {
    match coin {
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Penny => 1,
        Coin::Quarter(state) => {
            println!("State quarter is from {state:?}");
            25
        } 
    }
}