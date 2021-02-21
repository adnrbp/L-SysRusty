




fn simple_condition() {
	let c = true;

	let n = if c {
		58
	} else {
		76
	};

	println!("{:?}", n);
}


fn use_loop() {
	let mut counter = 0;

	loop {
	    println!("finite {}", counter);
	    counter += 1;

	    if counter >= 10 {
	    	break;
	    }
	}
}

/*
fn labeled_loop(){
	'a: loop {
		println!("loop a");
		'b: loop {
		    println!("loop b");
		    'c: loop {
		    	println!("loop c");

		    	break 'b
		    	/*
		    	if true {
		    		continue
		    	}else {
		    		brea
		    	}
		    	*/
		    }
		}
		//continue 'a
	}
}
*/

/*
fn loop_asignment(){
	//not supported version:
	/*
	let x = loop {
	    break 10;
	};

	println!("x: {}", x);
	*/
}
*/

fn simple_while(){
	let mut n = 10;

	while n != 0 {
		println!("{}!", n);

		n= n-1;

	}
}

fn for_loops(){
	let a = vec![10,20,30,40,50];

	for i in a {
		println!("i: {}", i);
	}
	//exclusive range (not 101) //inclusive "..."
	for j in 1..9 {
		println!("j: {}", j);
	}
}


fn simple_match(){
	let x = 5;

	match x {
	    1 => println!("one"),
	    2 => println!("two"),
	    3 => println!("three"),
	    4 => println!("four"),
	    5 => println!("five"),
	    _ => println!("something else"),
	}
}


fn match_patterns(){
	let value = 15;

	match value {
	    1 => println!("One!"),
	   	2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
	    13...19 => println!("a teen"),
	    _ => println!("aint special"),
	}

}

fn decompose_tuple(){
	let pair = (0, -2);

	//bind to the other value
	match pair {
		//if the 1st value is 0
	    (0,y) => println!("y: {}", y),
	    //if the 2nd value is 0
	    (x,0) => println!("x: {}", x),
	    _ => println!("no match"),
	}

}

fn use_guards_match(){
	//guards
	let pair2 = (5,-5);

	match pair2 {
	    (x,y) if x ==y => println!("Equal"),
	    (x,y) if x + y == 0 => println!("Equal Zero"),
	    (x,_) if x % 2 == 0 => println!("x is even"),
	    _ => println!("no match"),

	}
}


fn bind_matched(){
	let p = 5;

	match p {
	    n @ 1 ... 12 => println!("n: {}", n),
	    n @ 13 ... 19 => println!("n: {}", n),
	    _ => println!("no match"),
	}
}

fn asign_matched(){
	let p = 5;

	let cloned = match p {
	    n @ 1 ... 12 => n,
	    n @ 13 ... 19 => n,
	    _ => 0,
	};

	println!("cloned: {}", cloned);

}

pub fn check_values() {

	simple_condition();

	//Iterate:
	use_loop();
	simple_while();
	for_loops();

	//Match:
	simple_match();
	match_patterns();
	decompose_tuple();

	use_guards_match();
	bind_matched();
	asign_matched();


}