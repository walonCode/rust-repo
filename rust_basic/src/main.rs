//By default all variable in rust are immutable 
//By add the mut keyword a variable become mutable 

//rust hate when a value is not use or an import no used

//constant in Rust
const THREE_HOURS_IN_SECOND:u32 = 60 * 60 * 3;

fn main(){
    let mut x = 5;
    println!("Value of x is {x}");
    
    x = 8;
    println!("value of x is now {x}");
    
    println!("{THREE_HOURS_IN_SECOND}");
    
    //doing mut vs shadow
    
    //this is shadowing but the variable must be of the same data type
    // while mut allow you to change the value of the variable
    let y = 40;
    let y  = y + 1;
    
    {
        let y = 50;
        println!("The value of y inner {y}");
    }
    
    println!("value of y outer {y}");
    
    //data type - scalar type [int, float, bool, char, number]
    
    let guess:i32 = "42".parse().expect("Not a number");
    println!("{guess}");
    
    {
        //float things
        let j:f64 = 50.9944;
        println!("valie of j is {j}");
    }
    
    {
        //booleans
        // characters
        let j  = 'h';
        let k = true;
        
        println!("Values of j and k =  {j}, {k}");
    }
    
    //compound types- [array, tuples]
    
    let tup = (500,5.4, 3);
    let (x,y,z) = tup;
    
    println!("{x} {y} {z}");
    
    {
        let arr = [1,2,4];
        let one = arr[0];
        println!("{one}");
    }
    
    {
        let a  = [1,2,3,4,5];
        
        println!("Please enter and index");
        
        let index = 3;
        
        // io::stdin().read_line(&mut index).expect("failed to readline");
        
        // let index:usize = index.trim().parse().expect("enter a valid index");
        
        let element = a[index];
        
        println!("The value of the element is {element}");
    }
    
    let sum = add(3,5);
    println!("summ is {sum}");
    
    print("Walon is sexy");
    
    
    //control flow in rust ---- if/else
    {
        let number = 4;
        if number < 3 {
            println!("condition true");
        }else if number == 4 {
            println!("number is 4");
        }
        else {
            println!("condition false");
        }
    }
    
    {
        let condition:bool = false;
        
        let number = if condition { 5} else { 4};
        
        println!("{number}");
    }
    
    
    //loop in rust 
    // loops, while, for

    let mut counter = 0;
    
    // in rust using loop u can use it to get a result after certain iteration
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2 ;
        }
    };
    
    println!("{result}");
    
    //using while loop 
    let mut  value = 3;
    
    while value != 0 {
        println!("{value}");
        
        value -= 1;
    };
    
    //for loops
    
    let arr2 = [1,2,3,4,5];
    
    for element in arr2 {
        println!("{element}");
    }
}


//function in rust
// has the keyword fn 
fn add(a:i32, b:i32) -> i32 {
    a + b
}

fn print(word:&str){
    println!("{word}"); //this is a comment
}