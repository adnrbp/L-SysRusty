use std::io;


//parametric polymorphism (type theory)
//types/function that have multiple forms)

enum Option<T> {
	Some(T),		//some value of type (generic)
	None,
}

let x: Option<i32> = Some(5);
//Some(T), T es 5........
//5 es i32
//fail:
//let x: Option<f64> = Some(5);
	//5 es diferente de f64.....
let y: Option<f64> = Some(5.0f64)
//-------------Multiple values:
enum Result<T,E>{ //cualquier letra.....no solo "T y E"
	Ok(T),
	Err(E),
}

//generic function...
fn takes_anything<T,U>(x: T, y:U){ //take multiple types with similar syntax.

}

//generic structs
struct Point<T> {
	x: T,
	y: T,
}
let int_original = Point{x:0, y:0};
let float_origin = Point{x:0.0, y:0.0};)

impl<T> Point<T> {
	fn swap(&mut self){
		std::mem::swap(&mut self.x, &mut self.y);
	}
}

//traits:
trait HashArea {
    fn area(&self) -> f64;
}
struct Circle {
	x: f64,
	y: f64,
	radius: f64,
}
//impl Circle {
impl HashArea for Circle{
	fn area(&self) -> f64 {
		std::f64::consts::PI * (self.radius * self.radius)
	}
}

struct Square {
	x: f64,
	y: f64,
	side: f64,
}
//impl Square {
impl HashArea for Square {
	fn area(&self) -> f64{
		self.side * self.side
	}
}

fn print_area<T:HashArea>(shape: T){
	println!("this shape has an area of {}", shape.area() );
}


fn main() {
    println!("Hello, world!");
    //input Number of shapes....
    let mut shapeinput = String::new();
    io::stdin().read_line(&mut shapeinput)
            .expect("Failed to read line");



    let c = Circle {
    	x: 0.0f64,
    	y: 0.0f64,
    	radius: 1.0f64,
    };

    let s = Square {
    	x: 0.0f64,
    	y:0.0f64,
    	side:1.0f64,
    };

    print_area(c);
    print_area(s);

}
