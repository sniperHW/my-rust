
fn show_array(a2d: [[i32; 3]; 3],x: usize,y: usize) {
	print!("show_array\n");
	/*for j in 0..3 {
		for i in 0..3 {
			print!("{}",a2d[j][i]);
		}
		print!("\n");
	}*/
	println!("{}",a2d[y][x]);	
}

fn main() {
	let a: [i32; 3] = [1, 2, 3];
	//println!("{}",a[1]);
	for e in a.iter() {
    		println!("{}",e);
	}
	let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
	println!("{},{},{}", names[0],names[1],names[2]);

	let a2d: [[i32; 3]; 3] = [[1,2,3],[4,5,6],[7,8,9]];
	/*for j in 0..3 {
		for i in 0..3 {
			print!("{}",a2d[j][i]);
		}
		print!("\n");
	}*/
	show_array(a2d,1,2);
}