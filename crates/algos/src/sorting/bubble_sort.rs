pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
	for i in 0..v.len() {
		let mut sorted = true;

		for j in 0..(v.len() - 1) - i {
			if v[i] > v[j] {
				v.swap(i, j);
				sorted = false;
			}
		}

		if sorted {
			return;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bubble_sort() {
		let mut v = vec![5, 4, 3, 2, 1];

		bubble_sort(&mut v);

		assert_eq!(v, vec![1, 2, 3, 4, 5]);
	}
}
