
pub fn take(v: Vec<i32>){
	println!("we took v: {}", v[10] + v[100]);
}
pub fn ownership_validation(){
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








pub fn cop(a: i32, b: i32){
	println!("{}", a+b);
}
pub fn copy_values(){
	let a = 32;
	let b = 45;

	cop(a,b);
	//not unallocated in this function, still exists here
	//values copied (exists in stack, not the heap)
	println!("we have a: {} and b: {}", a,b);
}









//transfer ownership and return it
pub fn return_value(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

//takes a reference to the vector
pub fn borrow1(v: &Vec<i32>) {
	//call a pointer
    println!("{}", (*v)[10]+(*v)[12]);
}

pub fn borrow2(v: &Vec<i32>) {
	//call directly the values
	//more idiomatic
    println!("{}", v[10] + v[11]);
    //not show the actual memory reference value in print 
    //only follow the ref to the actual data value 
    //println!("{}", &v[10] + &v[11]);
}

pub fn borrowing_values(){
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










pub fn vector_counter(){
    let v = vec![4,5,3,6,7,4,2,8,3,5,8,7,3,4,4];
    for &i in &v {
    	let r = count(&v, i);
    	println!("{} is repeated {} times", i, r);
    }
    //v still exists until function ends
    println!("{}", v[1] );
}

pub fn count(v: &Vec<i32>,val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}