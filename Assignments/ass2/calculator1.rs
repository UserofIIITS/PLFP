use std::io;

fn main() {    

println!("Enter 1 for Addition 2 for Substraction 3 for Multiplication 4 for Division 5 for Mod");
	let mut input_text1= String::new();
	io::stdin()
		.read_line(&mut input_text1)
		.expect("failed to read from stdin");
	
	                 
	let trimmed = input_text1.trim();
	match trimmed.parse::<u32>() {

		Ok(i) => println!("you entered {}", i),
			Err(..) => println!("this was not an integer: {}", trimmed)
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
	
let my_int: u32 = trimmed.parse::<u32>().unwrap();
let i1_int: u32 = trimmed1.parse::<u32>().unwrap();
let i2_int: u32 = trimmed2.parse::<u32>().unwrap();

      // println!("Sum is {}",me*2);
       //println!("Sum is {}",m);
   if my_int== 1{
       println!("Sum is {}", i1_int+i2_int);
    }else if my_int==2 {
        println!("Difference is {}", i1_int-i2_int);
    }else if my_int==3 {
        println!("Product is {}", i1_int*i2_int);
    }else if my_int==4 {
        println!("Division is {}", i1_int/i2_int);
    }else {
        println!("Mod is {}", i1_int%i2_int);
    }





	
}
