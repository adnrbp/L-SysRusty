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
    //(Self, &String)
    let concat = h + &w;
    println!("{}", concat);


    //validate ownership
    ownership_validation();

    copy_values();

}

fn take(v: Vec<i32>){
	println!("we took v: {}", v[10] + v[100]);
}
fn ownership_validation(){
	//define a vector of dynamic size
	let mut v = Vec::new();

	//fill the vector with data
	for i in 1..1000 {
	    v.push(i);
	}
	//transfer ownership from this function to "take" fn
	//never return ownership ("moving")
	//Vector exists on the heap
	take(v);

	println!("Finished");
}

fn cop(a: i32, b: i32){
	println!("{}", a+b);
}
fn copy_values(){
	let a = 32;
	let b = 45;

	cop(a,b);
	//not unallocated in this function, still exists here
	//values copied (exists in stack, not the heap)
	println!("we have a: {} and b: {}", a,b);
}