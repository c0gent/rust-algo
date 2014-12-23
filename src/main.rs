#![allow(dead_code, unused_variable)]
#![feature(slicing_syntax)]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;

extern crate time;

use std::fmt;

mod quicksort;
mod common;
mod insertion_sort;
mod merge_sort;
mod built_in_sort;
mod qs_algo1;
mod randomized_contraction;
mod test;

fn main (){
	println!("\nArray Size: {}, Running Algorithms... ", common::ARRAYSIZE);
	
	//built_in_sort::built_in_sort();
	//merge_sort::merge_sort();
	//inversion_count();
	//closest_pair();
	//quicksort::quicksort();
	randomized_contraction::min_cut();
	//test::test();
	//qs_algo1::qs_algo1();
}

fn inversion_count() {
	println!("=== Inversion Count ===");
	println!("Loading Array From File...");
	let box array =  common::read_data_file("data/IntegerArray.txt");
	println!("Sorting and Counting...");
	let (_, inv_count) = _inversion_count(array);
	println!("Inversion Count: {}", inv_count);
}

fn closest_pair() {
	println!("=== Closest Pair ===");
	println!("Generating Points...");
	let box array =  common::generate_array_points();
	//print_array(&array, "");
	println!("Sorting Points by X and Y...");
	let (array_sorted_x, array_sorted_y) = _sort_points(array);
	//print_array(&array_sorted_x, "x:");
	//print_array(&array_sorted_y, "y:");
	println!("Determining Closest Pair via Recursion...");
	let (distance, clp) = _closest_pair(&array_sorted_x, &array_sorted_y);
	println!("Closest Pair: {} (distance: {})", clp, distance);

	println!("Determining Closest Pair via Brute Force...");
	let (distance_b, clp_b) = _closest_pair_brute(&array_sorted_x, &array_sorted_y);
	println!("Closest Pair: {} (distance: {})", clp_b, distance_b);

	println!("Looping and Comparing Both Methods...");
	let mut failure = false;
	for i in range(0u, common::LOOPCOUNT) {
		let (array_sorted_x, array_sorted_y) = _sort_points(*common::generate_array_points());
		let (d_r, _) = _closest_pair(&array_sorted_x, &array_sorted_y);
		let (d_bf, _) = _closest_pair_brute(&array_sorted_x, &array_sorted_y);
		if d_r != d_bf {
			println!("Closest Pair Distance Mismatch: {}, {}", d_r, d_bf);
			failure = true;
			break;
		}
	}
	if !failure {
		println!("{} Loops Completed With Matching Results.", common::LOOPCOUNT);
	}
}

fn _closest_pair_brute<T: PartialOrd + Primitive + Float + fmt::Show>(
				points_x: &Vec<(T, T)>, 
				points_y: &Vec<(T, T)>,
	) -> (
				T, 
				((T, T), (T, T)), 
	){
	let len = points_x.len();
	let mut clp = (points_x[0], points_x[1]);
	let mut dist_clp = points_distance(clp);

	for i in range(0u, points_x.len()) {
		for j in range(i + 1, points_x.len()) {
			let ij_dist = points_distance((points_x[i], points_x[j]));
			if ij_dist < dist_clp {
				clp = (points_x[i], points_x[j]);
				dist_clp = ij_dist;
				
			}
			//println!("points: {},{}; points_distance: {}", points_x[i], points_x[j], points_distance((points_x[i], points_x[j])));
		}
	}

	(dist_clp, clp)
}

fn _closest_pair<T: PartialOrd + Primitive + Float + fmt::Show>(
				points_x: &Vec<(T, T)>, 
				points_y: &Vec<(T, T)>,
	) -> (
				T, 
				((T, T), (T, T)), 
	) {
	let len = points_x.len();

	if len > 3u {
		let hlen = len / 2;
		let points_xl = points_x.slice(0, hlen).to_vec();
		let points_xr = points_x.slice(hlen, len).to_vec();
		
		let point_median = points_xr[0];

		//let (points_yl, points_yr) = points_y.partitioned(|&(x, y)| x >= point_median.val0());

		let mut points_yl: Vec<(T, T)> = Vec::with_capacity(points_xl.len());
		let mut points_yr: Vec<(T, T)> = Vec::with_capacity(points_xr.len());

		for &p in points_y.iter() {
			if p.val0() < point_median.val0() {
				points_yl.push(p);
			} else {
				points_yr.push(p);
			}
		}

		let (dist_clp_l, clp_l) = _closest_pair(&points_xl, &points_yl);
		let (dist_clp_r, clp_r) = _closest_pair(&points_xr, &points_yr);

		let (mut dist_clp, mut clp) = match dist_clp_l < dist_clp_r {
			true => (dist_clp_l, clp_l),
			false => (dist_clp_r, clp_r),
		};

		let strip_min_x = point_median.val0() - dist_clp;
		let strip_max_x = point_median.val0() + dist_clp;

		let mut points_y_strip: Vec<(T, T)> = Vec::with_capacity(3);

		for &p in points_y.iter() {
			if p.val0() >= strip_min_x && p.val0() <= strip_max_x {
				//println!("p: {}", p);
				points_y_strip.push(p);
			}
		}

		let py_len = points_y_strip.len();

		for i in range(0u, py_len) {
			for j in range(i + 1, if i + 7 > py_len {py_len} else {i + 7}) {
				//println!("j: {}", j);
				let ij_dist = points_distance((points_y_strip[i], points_y_strip[j]));
				if ij_dist < dist_clp {
					clp = (points_y_strip[i], points_y_strip[j]);
					dist_clp = ij_dist;
				}
				//println!("points: {},{}; points_distance: {}", points_y_strip[i], points_y_strip[j], points_distance((points_y_strip[i], points_y_strip[j])));
			}
		}

		//println!("Len: {}", len);
		//println!("Median: {}", point_median);

		//println!("strip_min_x: {}",strip_min_x);
		//println!("strip_max_x: {}",strip_max_x);

		//println!("len() -- points_xl: {}; points_xr: {}; points_yl: {}; points_yr: {}", points_xl.len(), points_xr.len(), points_yl.len(), points_yr.len());

		//print_array(&points_xl, "xl");
		//print_array(&points_xr, "xr");

		/*
		if py_len > 0 {
			println!("points_y_strip.len(): {}", py_len);
		} 
		*/
		
		(dist_clp, clp)
	} else {
		// 2 - 3 Points
		let mut clp = (points_x[0], points_x[1]);
		let mut dist_clp = points_distance(clp);

		if len == 3 {
			let clp2 = (points_x[0], points_x[2]);
			let dist_clp2 = points_distance(clp2);
			if dist_clp2 < dist_clp {
				clp = clp2;
				dist_clp = dist_clp2;
			}

			let clp3 = (points_x[1], points_x[2]);	
			let dist_clp3 = points_distance(clp3);
			if dist_clp3 < dist_clp {
				clp = clp3;
				dist_clp = dist_clp3;
			}
		}
		(dist_clp, clp)
	}
}

fn points_distance<T: PartialOrd + Primitive + Float>( ((x_1, y_1), (x_2, y_2)): ((T, T), (T, T)) ) -> T {
	((x_2 - x_1).powi(2) + (y_2 - y_1).powi(2)).sqrt()
}

fn _sort_points<T: PartialOrd + Primitive + Float + fmt::Show>(points_u: Vec<(T, T)>) -> (Vec<(T, T)>, Vec<(T, T)>) {
	let len = points_u.len();

	if len > 1u {
		let hlen = len / 2;

		let points_ul = points_u.slice(0, hlen).to_vec();
		let points_ur = points_u.slice(hlen, len).to_vec();

		let (points_slx, points_sly) = _sort_points(points_ul);
		let (points_srx, points_sry) = _sort_points(points_ur);

		let box points_sx = merge_points(&points_slx, &points_srx, 0);
		let box points_sy = merge_points(&points_sly, &points_sry, 1);

		(points_sx, points_sy)
	} else {
		let points_base_x = points_u.slice(0, 1).to_vec();
		let points_base_y = points_u.slice(0, 1).to_vec();

		(points_base_x, points_base_y)
	}
}

fn merge_points<T: PartialOrd + Primitive>(points_l: &Vec<(T, T)>, points_r: &Vec<(T, T)>, sort_by_index: uint) -> Box<Vec<(T, T)>> {
	let len = points_l.len() + points_r.len();
	let mut array: Box<Vec<(T, T)>> = box Vec::with_capacity(len);

	let mut li = points_l.iter();
	let mut ri = points_r.iter();

	loop {
		if li.peekable().is_empty() {
			match ri.next() {
				Some(x) => {
					array.push(*x);
					continue;
				}
				None => break,
			}
		}

		if ri.peekable().is_empty() {
			match li.next() {
				Some(&x) => {
					array.push(x);
					continue;
				}
				None => break,
			}
		}

		let l_cmp = if sort_by_index == 0 {
			li.peekable().peek().unwrap().val0()
		} else {
			li.peekable().peek().unwrap().val1()
		};

		let r_cmp = if sort_by_index == 0 {
			ri.peekable().peek().unwrap().val0()
		} else {
			ri.peekable().peek().unwrap().val1()
		};

		if l_cmp <= r_cmp {
			array.push(*li.next().unwrap());
		} else {
			array.push(*ri.next().unwrap());
		}
	}
	array
}

fn _inversion_count<T: PartialOrd + Primitive>(array_u: Vec<T>) -> (Vec<T>, uint) {
	let len = array_u.len();

	if len > 1u {
		//let (array_1u, array_2u) = split_array(array_u);
		let hlen = len / 2;

		let array_1u = array_u.as_slice()[0..hlen].to_vec();
		let array_2u = array_u.as_slice()[hlen..len].to_vec();

		let (array_1s, count_1s) = _inversion_count(array_1u);
		let (array_2s, count_2s) = _inversion_count(array_2u);

		let mut inv_count = count_1s + count_2s;
		let mut array_s: Vec<T> = Vec::with_capacity(len);
		let mut i1 = 0u; let mut i2 = 0u;

		while array_s.len() != len {
			if i1 == array_1s.len() {
				array_s.push(array_2s[i2]);
				i2 += 1;
				continue;
			}
			if i2 == array_2s.len() {
				array_s.push(array_1s[i1]);
				i1 += 1;
				continue;
			}
			if array_1s[i1] <= array_2s[i2] {
				array_s.push(array_1s[i1]);
				i1 += 1;
			} else {
				array_s.push(array_2s[i2]);
				i2 += 1;
				inv_count += array_1s.len() - i1;
			}
		}
		(array_s, inv_count)
	} else {
		(array_u, 0)
	}
}





#[bench]
fn bench_inversion_count(b: &mut Bencher) {
	//let (array_sorted, inversion_count) = _inversion_count(box read_data_file("data/IntegerArray.txt"));
	b.iter(|| { _inversion_count(*read_data_file("data/IntegerArray.txt")); });
}


//let array_ul = array_u.as_slice()[0..hlen].to_vec();
//let array_ur = array_u.as_slice()[hlen..len].to_vec();
