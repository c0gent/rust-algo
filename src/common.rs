#[phase(plugin, link)] extern crate log;

use std;
use std::rand;
use std::rand::distributions::{IndependentSample, Range};
use std::fmt;
use std::default;

pub static C_DEFAULT: &'static str = "\x1b[0m";
pub static C_RED: &'static str = "\x1b[91m";
pub static C_CYA: &'static str = "\x1b[36m";
pub static C_GRE: &'static str = "\x1b[32m";
pub static C_BLU: &'static str = "\x1b[94m";
pub static C_MAG: &'static str = "\x1b[95m";
pub static C_PUR: &'static str = "\x1b[35m";
pub static C_ORA: &'static str = "\x1b[33m";
pub static C_YEL: &'static str = "\x1b[93m";

pub static DATA_FILE_LENGTH: uint = 100000;


pub static ARRAYSIZE: uint = 1000000; // try to do 10^n's
pub static RANDMAX: int = 1000;
pub static LOOPCOUNT: uint = 5;

/* 
====combos to test====
A: 10000,	R: 1000
A: 100000,	R: 5000
A: 100,		R: 1
A: 100,		R: 2
*/


/* =================== read_data_file() =====================
	Open a text file and read contents into a vector.


	(to do) update: parameter to determine whether or not to loop the 
	lines of the input file to place items into sub-arrays.
*/
pub fn read_data_file(filename: &str) -> Box<Vec<int>> {
	let data_file = std::io::File::open(&std::path::Path::new(filename));
	let mut reader = std::io::BufferedReader::new(data_file);
	let mut array: Box<Vec<int>> = box Vec::with_capacity(DATA_FILE_LENGTH);

	loop {
		match reader.read_line() {
			Ok(line) => {
				match from_str(line.as_slice().trim()) {
					Some(x) => array.push(x),
					None => {},
				}
			}
			Err(_) => {
				break;
			}
		}
	}
	array
}

// Edge.vert1 and Edge.vert2 are indexes, not values
struct Edge {
	vert1: uint,
	vert2: uint,
}

struct Vertex {
	val: uint,
	edges: Vec<Edge>,
}

pub fn read_data_file_graph(filename: &str) -> Box<Vec<Vertex>> {
	let data_file = std::io::File::open(&std::path::Path::new(filename));
	let mut reader = std::io::BufferedReader::new(data_file);
	let mut vertices: Box<Vec<Vertex>> = box Vec::new();

	let mut i = 0u;
	loop {
		let new_vert: Vertex;
		match reader.read_line() {
			Ok(line) => {
				let mut j = -1i;
				let mut new_edges: Vec<Edge> = Vec::new();
				//vertices.push(new_vert);


				//pop first line

				let mut new_vert_val: uint = 0;


			    for word in line.as_slice().words() {
		    		let new_word: uint = match from_str(word.as_slice().trim()) {
						Some(x) => x,
						None => 999999999,
					};
			    	if j == 0 {
			    		new_vert_val = new_word;
			    		
			    	} else {
			    		let new_edge = Edge { vert1: new_vert_val, vert2: new_word };
			    		new_edges.push(new_edge);
			    	}
			    	
			        j += 1;
			    }


			    assert!(new_edges.len() == j as uint);
			    //println!("Vertex: {}, Edges: {}", i, new_edges.len()); 

			    new_vert = Vertex { val: new_vert_val, edges: new_edges };

				/*
				match from_str(line.as_slice().trim()) {
					Some(x) => array.push(x),
					None => {},
				}
				*/
			}
			Err(_) => {
				break;
			}
		}
		vertices.push(new_vert);
		i += 1;
	}

	println!("vertices.len() = {}",vertices.len());

	vertices
}

pub fn generate_array_points() -> Box<Vec<(f32, f32)>> {
	let rng_range = Range::new(0f32 - RANDMAX as f32, RANDMAX as f32);
	let mut rng = rand::task_rng();
	let array = box Vec::from_fn(ARRAYSIZE, |idx| (rng_range.ind_sample(&mut rng),rng_range.ind_sample(&mut rng)));

	array
}

pub fn split_array<T: Primitive>(array: Vec<T>) -> (Vec<T>, Vec<T>) {
	let len = array.len();
	let hlen = len / 2;

	let mut array1: Vec<T> = Vec::with_capacity(hlen);
	let mut array2: Vec<T> = Vec::with_capacity(len - hlen);

	for i in range(0u, hlen) {
		array1.push(array[i]);
		//println!("A1 - {}: {}", i, array1[i]);
	}
	for i in range(hlen, len) {
		array2.push(array[i]);
		//println!("A2 - {}: {}", i - hlen, array2[i - hlen]);
	}

	(array1, array2)
} 

pub fn print_array<T: fmt::Show>(array: &Vec<T>, prefix: &str) {
	print_slice(array.as_slice(), prefix, "", "");
}

pub fn print_slice<T: fmt::Show>(array: &[T], prefix: &str, postfix: &str, color_str: &str) {
	print!("{}: [", prefix);
	for i in range(0u, array.len()) {
		//println!("{}[{}]: {}", prefix, i, array[i]);
		print!("{c}{}{d}", array[i], c=color_str, d=C_DEFAULT);
		if i < array.len() - 1 {
			if i % 10 == 0 {
				let bars = (i / 10) % 5;
				if bars < 4 && i > 9 {
					print!(" ");
					for i in range(0u, bars) {
						print!("|"); 
					}
					print!(" ");
				}
				if bars == 4 {print!(" |\\/ ");}
				if bars == 0 {print!("][");}
			} else {
				print!(", ");
			}
		} else {
			print!("] {}\n", postfix);
		}
	}
}

pub fn generate_array() -> Box<Vec<uint>> {
	let rng_range = Range::new(0u, RANDMAX as uint);
	let mut rng = rand::task_rng();
	let array: Box<Vec<uint>> = box Vec::from_fn(ARRAYSIZE, |idx| rng_range.ind_sample(&mut rng));

	/*
	let mut array: Vec<uint> = Vec::with_capacity(ARRAYSIZE);
	for _ in range(0u, array.capacity()) {
		array.push((rand::random::<uint>() % 1000u) + 1u);
		//println!("{}{}: {}", "array", i, array[i]);
	}
	*/
	array
}

pub fn confirm_sorted<T: Primitive + PartialOrd + fmt::Show + default::Default>(list: &[T], verbosity: int) -> uint {
	let mut errors: uint = 0;
	for i in range(1u, list.len()) {
		if list[i] < list[i - 1] {
			errors += 1; 
			if verbosity != 0 {
				print!("|SE_IDX:[{r}{}{d}]|[{}]={}|[{}]={b}{}{d}|{d} ", i , i - 1, list[i - 1], i, list[i], b=C_BLU, r=C_RED, d=C_DEFAULT);
			}
		}
	}

	if errors > 0 {
		println!("");
		if verbosity != 0 {
			print_slice(list.as_slice(), format!("{r}BAD{d}", r=C_RED,d=C_DEFAULT).as_slice(), "", C_RED);
		}
		println!("{}Sorting Errors: {}{}", C_RED, errors, C_DEFAULT);
	} else {
		if verbosity != 0 {
			println!("{} elements inspected with no sorting errors.", list.len());
		}
	}
	errors
}

pub fn median<T: PartialOrd + Primitive + fmt::Show>(a: T, b: T, c: T) -> uint {
	//println!{"a: {}, b: {}, c: {}", a, b, c};
	if a < b {
		if b < c { // a b c
			1
		} else {
			if c <= a { // c a b
				0
			} else { // a c b
				2
			}
		}
	} else {
		if c < b { // c b a
			1
		} else {
			if c <= a { // b c a
				2
			} else { // b a c
				0
			}
		}
		
	}
}
