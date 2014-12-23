
pub fn insertion_sort<T: Primitive + PartialOrd>(list: &mut [T]) {
	let mut x: T;
	let mut j: uint;
	for i in range(0u, list.len()) {
		x = list[i];
		j = i;
		while j > 0 && list[j - 1] > x {
			list[j] = list[j - 1];
			j -= 1
		}
		list[j] = x;
	}
}
