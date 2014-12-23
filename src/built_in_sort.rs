use common;
use time; 

pub fn built_in_sort(){
	println!("=== Built-In-Sort ===");
	print!("Generating Array...  ");
	let mut time_start = time::get_time();
	let box mut list = common::generate_array();
	println!("Array Generated in: {}", time_start - time::get_time());
	println!("Vec.sorting...");
	time_start = time::get_time();
	list.sort();
	println!("One Iteration Complete in: {}", time_start - time::get_time());
}
