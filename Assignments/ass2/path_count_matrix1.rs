fn main() {

	//Declaring the structure of the function
	struct Mat<'s> { f: &'s Fn(&Mat, u32,u32) -> u32 }
	
	//Definition of function
	let mat = Mat {
f: &|mat, x ,y | if x == 1 || y == 1  {1} else {(mat.f)(mat, x - 1,y) + (mat.f)(mat,x, y- 1)}
	};
       //Calling the function
	println!("{}", (mat.f)(&mat, 3,3));
}


