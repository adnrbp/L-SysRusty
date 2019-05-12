
#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
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

	println!("\n{} x {} with area: {}", ob.width, ob.height, area(&ob) );

    
}