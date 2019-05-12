
use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

//Methods
impl Object {
	fn area(&self) -> u32 {
		self.width * self.height
	}


	fn show(&self){
		println!("\n{} x {} with area: {}", self.width, self.height, self.area());
	}
}

//Related Functions
impl Object {
    fn new(width: u32, height: u32) -> Object {
		//this version cannot avoid ": width"
		Object {
			width: width,
			height: height,
		}
	}
}

impl fmt::Display for Object {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.width, self.height)
	}
}


fn area(obj: &Object) -> u32 {
	obj.width * obj.height
}

pub fn area_finder(){
	//instanciate Object
	let ob = Object {
		width: 35,
		height: 55,
	};

	let obj = Object::new(57,83);

	//using plain struct with public function
	println!("\n{} x {} with area: {}", ob.width, ob.height, area(&ob));


	//Using a method implementation
	println!("\n{} x {} with area: {}", ob.width, ob.height, ob.area());

	//using related function new
	println!("\n{} x {} with area: {}", obj.width, obj.height, obj.area());

	//using object function declared:
	ob.show();
	obj.show();


	println!("{:#?}", ob);
	println!("{:#?}", obj);

	println!("{}", obj);



    
}