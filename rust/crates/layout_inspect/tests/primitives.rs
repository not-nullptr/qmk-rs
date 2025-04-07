use std::mem::{align_of, size_of};

use layout_inspect::{
	defs::{DefPrimitive, DefType},
	inspect,
};

#[test]
fn u8() {
	assert_eq!(
		inspect::<u8>()[0],
		DefType::Primitive(DefPrimitive {
			name: "u8".to_string(),
			size: size_of::<u8>(),
			align: align_of::<u8>(),
		})
	);
}

#[test]
fn u32() {
	assert_eq!(
		inspect::<u32>()[0],
		DefType::Primitive(DefPrimitive {
			name: "u32".to_string(),
			size: size_of::<u32>(),
			align: align_of::<u32>(),
		})
	);
}

#[test]
fn u128() {
	assert_eq!(
		inspect::<u128>()[0],
		DefType::Primitive(DefPrimitive {
			name: "u128".to_string(),
			size: size_of::<u128>(),
			align: align_of::<u128>(),
		})
	);
}

#[test]
fn i8() {
	assert_eq!(
		inspect::<i8>()[0],
		DefType::Primitive(DefPrimitive {
			name: "i8".to_string(),
			size: size_of::<i8>(),
			align: align_of::<i8>(),
		})
	);
}

#[test]
fn f64() {
	assert_eq!(
		inspect::<f64>()[0],
		DefType::Primitive(DefPrimitive {
			name: "f64".to_string(),
			size: size_of::<f64>(),
			align: align_of::<f64>(),
		})
	);
}

#[test]
fn usize() {
	assert_eq!(
		inspect::<usize>()[0],
		DefType::Primitive(DefPrimitive {
			name: "usize".to_string(),
			size: size_of::<usize>(),
			align: align_of::<usize>(),
		})
	);
}

#[test]
fn isize() {
	assert_eq!(
		inspect::<isize>()[0],
		DefType::Primitive(DefPrimitive {
			name: "isize".to_string(),
			size: size_of::<isize>(),
			align: align_of::<isize>(),
		})
	);
}

#[test]
fn non_zero_u8() {
	use std::num::NonZeroU8;
	assert_eq!(
		inspect::<NonZeroU8>()[0],
		DefType::Primitive(DefPrimitive {
			name: "NonZeroU8".to_string(),
			size: size_of::<NonZeroU8>(),
			align: align_of::<NonZeroU8>(),
		})
	);
}

#[test]
fn bool() {
	assert_eq!(
		inspect::<bool>()[0],
		DefType::Primitive(DefPrimitive {
			name: "bool".to_string(),
			size: size_of::<bool>(),
			align: align_of::<bool>(),
		})
	);
}

#[test]
fn char() {
	assert_eq!(
		inspect::<char>()[0],
		DefType::Primitive(DefPrimitive {
			name: "char".to_string(),
			size: size_of::<char>(),
			align: align_of::<char>(),
		})
	);
}

#[test]
fn unit() {
	assert_eq!(
		inspect::<()>()[0],
		DefType::Primitive(DefPrimitive {
			name: "()".to_string(),
			size: 0,
			align: 1,
		})
	);
}
