use std::mem::{align_of, size_of};

use layout_inspect::{
	defs::{DefOption, DefType},
	inspect, Inspect,
};

#[test]
fn option_primitive() {
	let type_defs = inspect::<Option<u8>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Option(DefOption {
			name: "Option<u8>".to_string(),
			size: size_of::<u8>() * 2,
			align: align_of::<u8>(),
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "u8");
}

#[test]
fn option_primitive_with_niche() {
	let type_defs = inspect::<Option<bool>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Option(DefOption {
			name: "Option<bool>".to_string(),
			size: size_of::<bool>(),
			align: align_of::<bool>(),
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "bool");
}

#[test]
fn option_struct() {
	#[derive(Inspect)]
	struct Foo {
		small: u8,
		big: u32,
	}

	let type_defs = inspect::<Option<Foo>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Option(DefOption {
			name: "Option<Foo>".to_string(),
			size: size_of::<Foo>() + align_of::<Foo>(),
			align: align_of::<Foo>(),
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "Foo");
}

#[test]
fn option_struct_with_niche() {
	use std::num::NonZeroU32;

	#[derive(Inspect)]
	struct Foo {
		small: u8,
		big: NonZeroU32,
	}

	let type_defs = inspect::<Option<Foo>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Option(DefOption {
			name: "Option<Foo>".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "Foo");
}

#[test]
fn option_box() {
	let type_defs = inspect::<Option<Box<u8>>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Option(DefOption {
			name: "Option<Box<u8>>".to_string(),
			size: size_of::<usize>(),
			align: align_of::<usize>(),
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "Box<u8>");
}

#[test]
fn option_vec() {
	let type_defs = inspect::<Option<Vec<u8>>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Option(DefOption {
			name: "Option<Vec<u8>>".to_string(),
			size: size_of::<usize>() * 3,
			align: align_of::<usize>(),
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "Vec<u8>");
}
