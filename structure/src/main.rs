use std::{arch::x86_64::_SIDD_CMP_EQUAL_ORDERED, hint::select_unpredictable};

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle{
    fn area(&self)-> u32{
        self.width * self.height
    }
    fn width(&self)->bool{
        self.width >0
    }
    fn can_hold(&self, other: &Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32)->Self{
        Self { width: size, height: size }
    }
}
fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    if rect1.width(){
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    

    let rect2= Rectangle{
        width:20,
        height: 10,
    };
    let rect3= Rectangle{
        width:60,
        height:100,
    };

    let rects:[&Rectangle;2]=[&rect2,&rect3]; 
    for r in rects{
        if rect1.can_hold(&r){
            println!("Can hold this {:#?}",r);
        }
        else {
            println!("Cannot hold this {:#?}",r);
        }
    }
    let sq = Rectangle::square(10);
    println!("Here is the square : {:#?}",sq);
}
