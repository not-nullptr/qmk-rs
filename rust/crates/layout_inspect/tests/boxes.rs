use std::mem::{align_of, size_of};

use layout_inspect::{
	defs::{DefBox, DefType},
	inspect, Inspect,
};

#[test]
fn boxed_primitive() {
	let type_defs = inspect::<Box<u8>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Box(DefBox {
			name: "Box<u8>".to_string(),
			size: size_of::<usize>(),
			align: align_of::<usize>(),
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "u8");
}

#[test]
fn boxed_struct() {
	#[derive(Inspect)]
	struct Foo {
		small: u8,
		big: u128,
	}

	let type_defs = inspect::<Box<Foo>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Box(DefBox {
			name: "Box<Foo>".to_string(),
			size: size_of::<usize>(),
			align: align_of::<usize>(),
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "Foo");
}
