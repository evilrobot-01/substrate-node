use frame_support::BoundedVec;
use sp_arithmetic::traits::BaseArithmetic;
#[cfg(test)]
use sp_core::{bounded_vec, ConstU32};

/// Determine the median of a slice of values.
///
/// **Warning:** Will panic if passed an empty slice.
pub(crate) fn median<T: Copy + BaseArithmetic, S>(numbers: &mut BoundedVec<T, S>) -> T {
	numbers.sort();

	let mid = numbers.len() / 2;
	if numbers.len() % 2 == 0 {
		numbers[mid - 1].saturating_add(numbers[mid]) / 2.into()
	} else {
		numbers[mid]
	}
}

#[test]
fn median_works() {
	let mut values: BoundedVec<u32, ConstU32<10>> = bounded_vec![4u32, 6, 2, 7];
	assert_eq!(median(&mut values), 5);
	let mut values: BoundedVec<u32, ConstU32<10>> = bounded_vec![4u32, 6, 2, 7, 9];
	assert_eq!(median(&mut values), 6);
}

#[test]
#[should_panic]
fn median_panics_on_empty_slice() {
	let mut empty: BoundedVec<u32, ConstU32<10>> = bounded_vec![];
	median(&mut empty);
}
