
struct User {
    active: bool,
    username: String,
    age:i32
}

//tuple struct
struct Color(i32);

// unit struct 
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.height * self.width
    }
    
    fn square(size:u32) -> Self {
        Self{
            width:size,
            height:size
        }
    }
}

fn main(){
    let mut  user1 = User{
        active: true,
        username: String::from("Walon"),
        age:40,
    };
    
    user1.active = false;
    
    println!("{}, {}, {}",user1.active, user1.age, user1.username);

    
    let user3 = User {
        active:false,
        ..user1
    };
    
    println!("{}", user3.username);
    
    let black = Color(0);
    
    println!("{}", black.0);
    
    let sq1 = Rectangle::square(32);
    println!("the square is {sq1:#?}");
    
    let _subject = AlwaysEqual;
    
    let rect1 = Rectangle{
        width:40,
        height:30
    };
    
    println!(
        "The area of the rectangle is {} square pixel",
        rect1.area()
    );
    
   println!("rect1 is {rect1:#?}")
}

// fn area(dimension: &Rectangle) -> u32 {
//     dimension.width * dimension.height
// }
