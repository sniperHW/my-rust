fn main() {
	let a: [i32; 3] = [1, 2, 3];
	//println!("{}",a[1]);
	for e in a.iter() {
    		println!("{}",e);
	}
	let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
	println!("{},{},{}", names[0],names[1],names[2]);
}