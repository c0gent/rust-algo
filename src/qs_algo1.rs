#![allow(unused_imports, unused_mut, unused_assignments)]
use std::fmt;
use std::default;
use time;

use common::{LOOPCOUNT, C_DEFAULT, C_RED, C_CYA, C_GRE, C_BLU, C_MAG, C_PUR, C_ORA, print_slice, generate_array, confirm_sorted, read_data_file, median};
use insertion_sort;

static MINIMUM_LEN: uint = 2;
static DEBUG: int = 0;  // 1001 = everything; 101 = whatever; 81= less; 11 = less info 

/* 
==== Comparisons Made ====
leftmost value for pivot: 	162085
rightmost value for pivot: 	164123
Median of bot/mid/top:		138382


*/

pub fn qs_algo1() {
	println!("\n=== QS_ALGO1 ===")
	let mut total_errors = 0u;
	let mut file_name = "data/QuickSort.txt";
	let mut comparisons: uint = 0;
	println!("Looping ({}) times...", LOOPCOUNT);
	for i in range(0u, LOOPCOUNT) {
		//let box mut list = read_data_file(file_name);
		let box mut list = generate_array();

		//println!("Sorting...");
		let mut time_start = time::get_time();
		_qs_algo1(list.as_mut_slice(), 0, &mut comparisons);
		println!("One Iteration Complete in: {}", time_start - time::get_time());
		println!("{} Comparisons Made", comparisons);
		comparisons = 0;

		total_errors += confirm_sorted(list.as_slice(), DEBUG);
	}
	if total_errors > 0 {
		println!("\nTotal Sorting Errors: {}", total_errors);
	} else {
		println!("({}) Loops Completed Successfully!", LOOPCOUNT)
	}
}

pub fn _qs_algo1<T: Primitive + PartialOrd + fmt::Show + default::Default>(list: &mut [T], level: uint, cmp: &mut uint) {
	let len = list.len();

	if DEBUG > 80 {
		if len > 0 {
			//println!("");
			print_slice(list, "TOP ", format!("[LVL:{}, LEN:{}]", level, len).as_slice(), C_MAG);
		} else {
			//println!("");
			println!("TOP : [{m}EMPTY{d}] [LVL:{}, LEN:{}]", level, len, m=C_MAG, d=C_DEFAULT);
		}
	}
	
	if len >= MINIMUM_LEN {
		let i_init = 1;
		let mut i = i_init;
		let mut j = 0u;
		let mut p_val = default::Default::default();

		if DEBUG > 100 {
			print_slice(list, "PRE ", format!("[LVL:{}, LEN:{}][PIV:{b}{}{d}, I:{}, J:{}]", level, len, p_val, i, j, b=C_BLU, d=C_DEFAULT).as_slice(), C_CYA);
		}

		/*
		let med_idx = match median(list[0], list[(len - 1) / 2], list[len - 1]) {
			0 => 0,
			1 => (len - 1) / 2,
			2 => len - 1,
			_ => fail!("median was > 2"),
		};
		list.swap(0, med_idx);
		if len == 2 {
			med_idx == 0;
		}
		*/
		if level <= len - 1 {
			list.swap(0, level);
			if DEBUG > 1000 {
				print!(" PRE_SWAP({},{}) ", 0u, level);
			}
		} else {
			list.swap(0, level % len);
			if DEBUG > 1000 {
				print!(" PRE_SWAP({},{}) ", 0u, level % len);
			}
		}
		
		
		let p = 0;
		let l = 1;
		let r = list.len() - 1;
		p_val = list[p];
		i = 1;

		// Will Still Overflow when all elements are =

		for k in range(l, r + 1) {
			if list[k] < p_val {
				list.swap(i, k);
				i += 1;
				if DEBUG > 1000 {
					print!(" SWAP({},{}) ", i, k);
				}
			}
		}
		list.swap(p, i - 1);

		if DEBUG > 1000 {
			print!("[LVL:{}, LEN:{}, i:{}]", level, len, i);
			println!(" END_SWAP({},{}) ", p, i - 1);
		}

		if DEBUG > 100 {
			print_slice(list, "POST", format!("[LVL:{}, LEN:{}][PIV:{b}{}{d}, I:{}, P:{}]", level, len, p_val, i, p, b=C_BLU, d=C_DEFAULT).as_slice(), C_GRE);
		}

		if list[0] != list[len - 1] {
			if DEBUG > 100 {
				println!("");
			}
			_qs_algo1(list.slice_mut(0u, i - 1), level + 1, cmp);
			_qs_algo1(list.slice_mut(i, len), level + 1, cmp);
		}
		
		if DEBUG > 1000 {
			print_slice(list, "BOT ", format!("[LVL:{}, LEN:{}][PIV:{b}{}{d}, I:{}, P:{}]",level, len, p_val, i, p, b=C_BLU, d=C_DEFAULT).as_slice(), C_ORA);
		}
		if DEBUG > 100 {
			println!("");
		}
	} else {
		//insertion_sort::insertion_sort(list);
		//return;
	}
}
