use std::mem;

fn main() {
	let xs: [i32; 5] = [4,5,6,7,8];
    println!("\n\t Array:
		first Value: {} 
    		length: {}
    		size in memory: {}\n", xs[0], xs.len(), mem::size_of_val(&xs));
    let array_slice = &xs[2..4];
    println!("\tSlice: {:?}", array_slice);

    //Strings
    let s = "String".to_string();
    let ss = String::from("String!");

    let str_slice = &ss[0..4];

    println!("{}",s);
    println!("{}", ss);
    println!("{}", str_slice);

    //Concatenate Strings
    let h = String::from("Big, ");
    let w = String::from("Planet! ");
    let concat = h + &w;
    println!("{}", concat);

}
