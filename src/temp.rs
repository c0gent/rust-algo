fn _quicksort<T: Primitive + PartialOrd + fmt::Show + default::Default>(list: &mut [T], level: uint) {
	let len = list.len();
	
	if len >= MINIMUM_LEN {
		let mid = len / 2;
		let pivot_radius = PIVOT_RADIUS;

		insertion_sort::insertion_sort(list.slice_mut(mid - pivot_radius, mid + pivot_radius + 1));
		let mut pivot_val = list[mid];

		list.swap(0, mid);
		if pivot_radius > 0 {
			let mid_piv0 = mid - pivot_radius;
			let end_piv0 = (len - 1) - pivot_radius;
			
			for k in range(0u, pivot_radius) {
				list.swap(1 + k, mid_piv0 + k);
				list.swap(mid + 1 + k, end_piv0 + k);
			}
		}

		let i_init =  1;
		let mut i = i_init;
		let j_init = len - 1;
		let mut j = j_init;
		let mut lt_idx = 0u;

	
		loop {
			loop {
				while list[j] > pivot_val && j > 0{
					j -= 1
				}
				if i < j {
					if list[i] > pivot_val {
						list.swap(i,j);
						if DEBUG != 0 {
							print!(" SWAP({},{}) ", i, j);
						}
					} else if list[i] < pivot_val && lt_idx == 0 {
						lt_idx = i;
					}
				} else {
					break;
				}
				i += 1;
			}

			if i < j_init  {
				break;
			} else {
				if j < j_init {
					break;
				} else {
					if lt_idx == 0 {
						return;
					} else {
						if DEBUG != 0 {
							print!("{p}RESET([0]{} -> [{}]{}){d}", pivot_val, lt_idx, list[lt_idx], p=C_PUR, d=C_DEFAULT);
						}
						list.swap(0,lt_idx);
						pivot_val = list[0];
						i = i_init;
						j = j_init;
						lt_idx = 0;
					}
				}
			}
		}

		list.swap(0u, j);

		_quicksort(list.slice_mut(0u, j), level + 1);
		_quicksort(list.slice_mut(j + 1, len), level + 1);
		
	} else {
		insertion_sort::insertion_sort(list);
	}

}
