fn main() {

	//Declaring the structure of the function
	struct Mat<'s> { f: &'s Fn(&Mat, u32,u32) -> u32 }
	
	//Definition of function
	let mat = Mat {
f: &|mat, x ,y | if x == 1 || y == 1  {1} else {(mat.f)(mat, x - 1,y) + (mat.f)(mat,x, y- 1)}
	};
       println! ("Now enter two numbers");
				   		let mut i1= String::new();
				    	let mut i2= String::new();

				   io::stdin()
				   		.read_line(&mut i1)
				   		.expect("failed to read from stdin");
				   	io::stdin()
				   		.read_line(&mut i2)
				   		.expect("failed to read from stdin");
				   	                 
				   	let trimmed1 = i1.trim();
				   	match trimmed1.parse::<u32>() {

				   		Ok(n) => print!("you entered {}", n),
				   			Err(..) => println!("this was not an integer: {}", trimmed1)
				   	};
				   	                 
				   	let trimmed2 = i2.trim();
				   	match trimmed2.parse::<u32>() {
				   		Ok(m) => println!(" and {}", m),
				   			Err(..) => println!("this was not an integer: {}", trimmed)

				   	};
				   	let i1_int: u32 = trimmed1.parse::<u32>().unwrap();
                    let i2_int: u32 = trimmed2.parse::<u32>().unwrap();

	println!("{}", (mat.f)(&mat, i1_int,i2_int));
}
