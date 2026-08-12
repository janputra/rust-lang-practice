fn main() {
    variable_mutability();    
    tupple_array();
    function();
    control_flow();
}

fn variable_mutability(){
    println!("Variable & Mutability");
    println!("-------------------------");
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
    println!("-------------------------");
}

fn tupple_array(){
    println!("Tupple & Array ");
    println!("-------------------------");
    let tup = (500,6.4,1);
    let (x,y,z) = tup;
    println!("the value of y is : {y}");
    
    let a : [i32;5] = [1,2,3,4,5];
    let mut i = 0;
    println!("Printing member of array");
    loop {
       println!("Member {i} of array : {}",a[i]); 
       i = i+1;
       if i>= 5 {
            break;
       }
    }
    println!("-------------------------");
}

fn function(){
    println!("Functions");
    println!("-------------------------");
    another_function(5);
    another_function_twoargs(2, 'g');
    statement_expression();
    let y = plus_one(4);

    println!("Result of function with return value : {y}");

    println!("-------------------------");
}

fn another_function(x: i32){
    println!("The value of argument is : {x}")    
}

fn another_function_twoargs(x: i32,label: char){
    println!("The value of first argument is : {x}");   
    println!("The value of second argument is : {label}");
}

fn statement_expression(){
    let y = {
        let x = 3;
        x+1
    };
    println!("Printing the value of expression of value y = {y}");
}

fn plus_one(x: i32) -> i32{
    x+1
}

fn control_flow(){
    println!("Control Flow");
    println!("-------------------------");

    println!("using let with condition");

    let cond = true;

    let number= if cond {6} else {7};

    println!("The condition is {cond} so the number is {number}");

    println!("-------------------------");
    println!("multiple condition");

    if number % 2 == 0 {
        println!("Number is divisible by 2");
    }
    else if number % 3 == 0{
        println!("Number is divisible by 2");
    }
    else if number % 5 == 0 {
        println!("Number is divisible by 2");
    }else{
        println!("Number is not divisible by 2,3,5");
    }

    println!("-------------------------");
    println!("using let with loop");
    let mut count = 0;
    let ret_loop = loop {
            count = count +1;
            if count == 10 {
                break count *2;
            }
    } ;
    println!("Return value from loop is : {ret_loop}");
    println!("-------------------------");

    println!("using nested loop");
    nested_loop();
    println!("-------------------------");
    println!("using while loop");

    let mut count = 10;
    while count != 0{
        println!("{count}");
        count -= 1;
    } 
    println!("Counter is finished");

    println!("-------------------------");
    println!("using for loop");
    let a:[i32;5]= [0,2,4,6,8];
    for element in a {
        println!("Print member of arary with for loop:{}",element);
    }
    for element in 1..4{
        println!("Print range for loop : {element}");
    }
    for element in (1..4).rev(){
        println!("Print range for loop : {element}");
    }
    println!("-------------------------");
}

fn nested_loop(){
    let mut count = 0;
    'counting_up: loop {
        println!("Counting Up {count}");
        let mut remaining = 9;
        loop{
            println!("remaining : {remaining}");
            
            if remaining == 8{
                break;
            }
            if count == 3{
                break 'counting_up;
            }
            remaining -=1;
        }
        count += 1;
    }
    println!("Ended neseted loop");
}