use std::mem::{align_of, size_of};

use layout_inspect::{
	defs::{DefType, DefVec},
	inspect, Inspect,
};

#[test]
fn vec_primitive() {
	let type_defs = inspect::<Vec<u8>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Vec(DefVec {
			name: "Vec<u8>".to_string(),
			size: size_of::<usize>() * 3,
			align: align_of::<usize>(),
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "u8");
}

#[test]
fn vec_struct() {
	#[derive(Inspect)]
	struct Foo {
		small: u8,
		big: u128,
	}

	let type_defs = inspect::<Vec<Foo>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Vec(DefVec {
			name: "Vec<Foo>".to_string(),
			size: size_of::<usize>() * 3,
			align: align_of::<usize>(),
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "Foo");
}
