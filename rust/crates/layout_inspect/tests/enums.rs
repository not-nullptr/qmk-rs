use std::mem::{align_of, size_of, transmute};

use layout_inspect::{
	defs::{DefEnum, DefEnumTag, DefEnumVariant, DefType},
	inspect, Inspect,
};

#[test]
fn enum_fieldless() {
	#[allow(dead_code)]
	#[derive(Inspect)]
	enum Foo {
		Opt1,
		Opt2,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: 1,
			align: 1,
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: Some("Opt1".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: Some("Opt2".to_string()),
					value_type_id: None
				},
			],
			tag: DefEnumTag::None,
		})
	);

	// Check discriminants are correct
	fn to_u8(foo: Foo) -> u8 {
		unsafe { transmute(foo) }
	}
	assert_eq!(to_u8(Foo::Opt1), 0);
	assert_eq!(to_u8(Foo::Opt2), 1);
}

#[test]
fn enum_fieldless_raw_identifier_variant_name() {
	#[allow(dead_code, non_camel_case_types)]
	#[derive(Inspect)]
	enum Foo {
		r#type,
		r#enum,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: 1,
			align: 1,
			variants: vec![
				DefEnumVariant {
					name: "type".to_string(),
					discriminant: 0,
					ser_value: Some("type".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "enum".to_string(),
					discriminant: 1,
					ser_value: Some("enum".to_string()),
					value_type_id: None
				},
			],
			tag: DefEnumTag::None,
		})
	);
}

#[test]
fn enum_fieldless_with_serde_type_rename() {
	#[allow(dead_code)]
	#[derive(Inspect)]
	#[serde(rename = "Bar")]
	enum Foo {
		Opt1,
		Opt2,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Bar".to_string(),
			size: 1,
			align: 1,
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: Some("Opt1".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: Some("Opt2".to_string()),
					value_type_id: None
				},
			],
			tag: DefEnumTag::None,
		})
	);
}

#[test]
fn enum_fieldless_with_serde_variant_rename() {
	#[allow(dead_code)]
	#[derive(Inspect)]
	enum Foo {
		#[serde(rename = "o1")]
		Opt1,
		#[serde(rename = "o2")]
		Opt2,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: 1,
			align: 1,
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: Some("o1".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: Some("o2".to_string()),
					value_type_id: None
				},
			],
			tag: DefEnumTag::None,
		})
	);
}

#[test]
fn enum_fieldless_with_serde_variants_rename_all() {
	#[allow(dead_code, clippy::enum_variant_names)]
	#[derive(Inspect)]
	#[serde(rename_all = "camelCase")]
	enum Foo {
		OptOne,
		r#OptTwo,
		#[serde(rename = "opt_three")]
		OptThree,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: 1,
			align: 1,
			variants: vec![
				DefEnumVariant {
					name: "OptOne".to_string(),
					discriminant: 0,
					ser_value: Some("optOne".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "OptTwo".to_string(),
					discriminant: 1,
					ser_value: Some("optTwo".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "OptThree".to_string(),
					discriminant: 2,
					ser_value: Some("opt_three".to_string()),
					value_type_id: None
				},
			],
			tag: DefEnumTag::None,
		})
	);
}

#[test]
fn enum_fieldless_with_discriminants() {
	#[allow(dead_code)]
	#[derive(Inspect)]
	enum Foo {
		Opt1 = 5,
		Opt2 = 10,
		Opt3,
		Opt4,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: 1,
			align: 1,
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 5,
					ser_value: Some("Opt1".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 10,
					ser_value: Some("Opt2".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt3".to_string(),
					discriminant: 11,
					ser_value: Some("Opt3".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt4".to_string(),
					discriminant: 12,
					ser_value: Some("Opt4".to_string()),
					value_type_id: None
				},
			],
			tag: DefEnumTag::None,
		})
	);

	// Check discriminants are correct
	fn to_u8(foo: Foo) -> u8 {
		unsafe { transmute(foo) }
	}
	assert_eq!(to_u8(Foo::Opt1), 5);
	assert_eq!(to_u8(Foo::Opt2), 10);
	assert_eq!(to_u8(Foo::Opt3), 11);
	assert_eq!(to_u8(Foo::Opt4), 12);
}

#[test]
fn enum_fieldful() {
	#[allow(dead_code)]
	#[derive(Inspect)]
	enum Foo {
		Opt1(u8),
		Opt2(u16),
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: None,
					value_type_id: Some(1)
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: None,
					value_type_id: Some(2)
				},
			],
			tag: DefEnumTag::None,
		})
	);

	let variant_ids = get_variant_ids(&type_defs[0]);
	assert_eq!(type_defs[variant_ids[0].unwrap()].name(), "u8");
	assert_eq!(type_defs[variant_ids[1].unwrap()].name(), "u16");
}

#[test]
fn enum_fieldful_with_serde_tag() {
	#[allow(dead_code)]
	#[derive(Inspect)]
	#[serde(tag = "type")]
	enum Foo {
		Opt1(u8),
		Opt2(u16),
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: 4,
			align: 2,
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: None,
					value_type_id: Some(1)
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: None,
					value_type_id: Some(2)
				},
			],
			tag: DefEnumTag::Tag("type".to_string()),
		})
	);
}

#[test]
fn enum_fieldful_with_serde_tag_and_content() {
	#[allow(dead_code)]
	#[derive(Inspect)]
	#[serde(tag = "t", content = "c")]
	enum Foo {
		Opt1(u8),
		Opt2(u16),
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: 4,
			align: 2,
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: None,
					value_type_id: Some(1)
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: None,
					value_type_id: Some(2)
				},
			],
			tag: DefEnumTag::TagAndContent {
				tag: "t".to_string(),
				content: "c".to_string()
			},
		})
	);
}

#[test]
fn enum_fieldful_with_serde_untagged() {
	#[allow(dead_code)]
	#[derive(Inspect)]
	#[serde(untagged)]
	enum Foo {
		Opt1(u8),
		Opt2(u16),
	}

	assert_eq!(
		&inspect::<Foo>()[0],
		&DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: None,
					value_type_id: Some(1)
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: None,
					value_type_id: Some(2)
				},
			],
			tag: DefEnumTag::Untagged,
		})
	);
}

#[test]
fn enum_mixed_fieldless_and_fieldful() {
	#[allow(dead_code)]
	#[derive(Inspect)]
	enum Foo {
		Opt1,
		Opt2(u8),
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: 2,
			align: 1,
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: Some("Opt1".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: None,
					value_type_id: Some(1)
				},
			],
			tag: DefEnumTag::None,
		})
	);

	let variant_ids = get_variant_ids(&type_defs[0]);
	assert_eq!(type_defs[variant_ids[1].unwrap()].name(), "u8");

	// Check discriminants are correct
	fn to_bytes(foo: Foo) -> [u8; 2] {
		unsafe { transmute(foo) }
	}
	assert_eq!(to_bytes(Foo::Opt1), [0, 0]);
	assert_eq!(to_bytes(Foo::Opt2(10)), [1, 10]);
}

fn get_variant_ids(enum_def: &DefType) -> Vec<Option<usize>> {
	enum_def
		.to_enum()
		.unwrap()
		.variants
		.iter()
		.map(|variant| variant.value_type_id.map(|id| id as usize))
		.collect()
}
