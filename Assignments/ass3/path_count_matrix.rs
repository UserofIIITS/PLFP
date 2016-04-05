fn main() {
	   struct Fact<'s> { f: &'s Fn(&Fact, u32,u32) -> u32 }
	      let fact = Fact {
		             f: &|fact, x, y | if x == 0 {1} else {x * (fact.f)(fact, x - 1,y)}
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


	         println!("{}", (fact.f)(&fact,i1_int,i2_int));
}
