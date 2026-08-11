fn main() {
    variable_mutability();    

}

fn variable_mutability(){
    // variable and mutability (default behaviour for variable is immutable)
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6; 
    println!("The value of x is {x}");
    
    // const can't be declared with mut 
    const THREE_HOURS_IN_SECOND: u32 = 60*60*3;
    println!("The value of const is {THREE_HOURS_IN_SECOND}");
    
    // Shadowing
    let x = 5;
    let x = x+1;

    {
        let x = x*2;
        println!("Here  is the x value inside scope : {x}");
    }
    
    println!("The value of x is : {x}");   
}