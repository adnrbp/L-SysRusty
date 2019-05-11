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

    borrowing_values();

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





//transfer ownership and return it
fn return_value(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

//takes a reference to the vector
fn borrow1(v: &Vec<i32>) {
	//call a pointer
    println!("{}", (*v)[10]+(*v)[12]);
}

fn borrow2(v: &Vec<i32>) {
	//call directly the values
	//more idiomatic
    println!("{}", v[10] + v[11]);
    //not show the actual memory reference value in print 
    //only follow the ref to the actual data value 
    //println!("{}", &v[10] + &v[11]);
}

fn borrowing_values(){
	let mut v = Vec::new();

	for i in 1..1000 {
	    v.push(i);
	}

	v = return_value(v);

	println!("Still own v: {} {}", v[0], v[1]);

	borrow1(&v);
	println!("Still own v: {} {}", v[0], v[1]);

	borrow2(&v);
	println!("Still own v: {} {}", v[0], v[1]);


}