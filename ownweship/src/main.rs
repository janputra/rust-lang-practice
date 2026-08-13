fn main() {
    let s = String::from("hello");  // s is allocated in heap
    takes_ownership(s);
    
    let x = 4;                  // x is allocated stack 
    makes_copy(x);
    move_ownership();
    borrow_ownership();
    slice();
}

fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(some_integer: i32){
   println!("{some_integer}"); 
}

fn move_ownership(){
    let s1 = gives_ownership();
    println!("{s1}");
    
    let s2 = String::from("hello transfering");
    let s3 = takes_and_gives_back(s2);
    println!("{s3}");
    let (s4, len) = calculate_length(s3);
    println!("{s4} : {len}");
}

fn gives_ownership() -> String {
    let s = String::from("ownership moving");
    s
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s:String) -> (String,usize){
    let len = s.len();
    (s, len)
}

fn borrow_ownership(){
    let mut s = String::from("Hello");
    println!("the len {s}: {}",calculate_length_ref(&s));
    change(&mut s);
    println!("the len {s}: {}",calculate_length_ref(&s));
}

fn calculate_length_ref(s: &String) -> usize{
   s.len()
}

fn change(some_string: &mut String){
   some_string.push_str(", world");
}
fn slice(){
    let s = String::from("Hello, World");
    
    let first = &s[0..5];
    let second = &s[5..];
    
    println!("{first}");
    println!("{second}");
}
