#![allow(unused_imports, unused_mut, unused_assignments)]

use common;
use time; 

pub fn merge_sort() {
	println!("=== Merge-Sort ===");
	print!("Generating Array...  ");
	let mut time_start = time::get_time();
	let box mut list = common::generate_array();
	println!("Array Generated in: {}", time_start - time::get_time());
	println!("Merge-Sorting...");
	time_start = time::get_time();
	let (array_s, ml) = _merge_sort(list, 0u, 0);
	println!("Merge Sort Max Level: {}", ml);
	println!("One Iteration Complete in: {}", time_start - time::get_time());

}

fn _merge_sort(array: Vec<uint>, level: uint, max_level: uint) -> (Vec<uint>, uint) {
	let len = array.len();
	let hlen = len / 2;
	let mut ml = level;
	if len > 1u {
		let mut array1: Vec<uint> = Vec::with_capacity(hlen);
		let mut array2: Vec<uint> = Vec::with_capacity(len - hlen);

		for i in range(0u, hlen) {
			array1.push(array[i]);
			//println!("A1 - {}: {}", i, array1[i]);
		}
		for i in range(hlen, len) {
			array2.push(array[i]);
			//println!("A2 - {}: {}", i - hlen, array2[i - hlen]);
		}

		let (array_1o, ml1) = _merge_sort(array1, level + 1, max_level);
		let (array_2o, ml2) = _merge_sort(array2, level + 1, max_level);

		if ml1 > ml {
			ml = ml1;
		}
		if ml2 > ml {
			ml = ml2;
		}

		let mut array_s = Vec::with_capacity(len);
		let mut i1 = 0u;
		let mut i2 = 0u;

		while array_s.len() != array_s.capacity() {

			if i1 >= array_1o.len() {
				array_s.push(array_2o[i2]);
				i2 += 1;
				continue;
			}
			if i2 >= array_2o.len() {
				array_s.push(array_1o[i1]);
				i1 += 1;
				continue;
			}
			if array_1o[i1] < array_2o[i2] {
				array_s.push(array_1o[i1]);
				i1 += 1;

			} else {
				array_s.push(array_2o[i2]);
				i2 += 1;
			}
			
		}
		(array_s, ml)
	} else {
		(array, ml)
	}
}
