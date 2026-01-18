

fn main(){
    //heap string
    let mut name = String::from("walon");
    name.push_str("-jalloh");
    println!("{name}");
    
    {
        //stack string
        let name = "walon";
        println!("{name}");
    }
    
    {
        let x = 5;
        let y = x;
        
        println!("{} {}", x,y);
    }
    
    {
        let s1 = String::from("Hello");
        let _s2 = s1.clone();
        
        println!("{s1}, world!");
    }
    
    {
        // let _s = String::from("Hello");
        let s = String::from("ahoy");
        
        println!("{s}, world")
    }
    
    let (word, size) = is_palindrome("Walon");
    println!("{word}, {size}");
    
    {
        let mut s = String::from("Hello");
        let len = calculate_length(&s);
        
        s.push_str(", world");
        
        println!("the string {s} has a length of {len}");
    }
    
    {
        //referencing and deferencing 
        let mut  num = 10;
        let p = &mut num;
        
        *p = 40;
        
        println!("{}",p);
        println!("Memory address of num is {num}");
    }
    
    {
        let mut s = String::from("WALON");
        
        //since you can only has one reference at a time it prevents data race / race conditions
        // let r1 =  &mut s.clone();
        // *r1 = String::from("walon");
        // let r2 = s;
        change_str(&mut s);

        
        println!("{s}");
    }
    
    {
        let mut s = String::from("jalloh");
        
        //referencing
        let r1 = &s;
        let r2 = &s;
        
        //error part
        // let r3 = &mut s;
        
        println!("{r1}, {r2}");
        
        //here is work since the immutable reference has went out of scope 
        let r3 = &mut s;
        *r3 = String::from("walon");
        println!("{r3}")
    }
    
    {
        //dangling pointer
        let ref_to_nothing = dangle();
        println!("{ref_to_nothing}");
    }
    
    {
        let s = "hello world jallo";
        // let s = s.to_lowercase();
        let first = first_word(s);
        
        println!("{first}")
    }
}


//tips
// a function in rust can return multiple values in a tuple
fn is_palindrome(word:&str) -> (&str, usize){
    let length = word.len();
    
    (word, length)
}

fn calculate_length(str: &String) -> usize{
    str.len()
}

//by default reference are immutable 
fn change_str(s:&mut String) {
    s.push_str(", world");
}

//dangling function
fn dangle() -> String {
    let s = String::from("MLWJ");
    
    s
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    };
    
    &s[..]
}