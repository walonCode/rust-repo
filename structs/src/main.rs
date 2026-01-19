struct User {
    active: bool,
    username: String,
    age:i32
}

fn main(){
    let mut  user1 = User{
        active: true,
        username: String::from("walondev"),
        age:40,
    };
    
    user1.active = false;
    
    println!("{}, {}, {}",user1.active, user1.age, user1.username);
    
    let user2 = build_user(String::from("walon"), 20);
    let user3 = User {
        active:false,
        ..user2
    };
    
    println!("{}", user3.username);
}

fn build_user(username:String,age:i32) -> User {
    User { active: true, username, age }
}