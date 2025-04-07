use std::mem::{align_of, size_of};

use layout_inspect::{
	defs::{DefStruct, DefStructField, DefType},
	inspect, Inspect,
};

#[test]
fn struct_single_field() {
	#[derive(Inspect)]
	struct Foo {
		num: u8,
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "num".to_string(),
				ser_name: "num".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
				skip: false,
			}],
			tag: None,
			transparent: false,
		})
	);

	let field_ids = get_field_ids(&type_defs[0]);
	assert_eq!(type_defs[field_ids[0]].name(), "u8");
}

#[test]
fn struct_empty() {
	#[derive(Inspect)]
	struct Foo {}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(0),
			align: Some(1),
			fields: vec![],
			tag: None,
			transparent: false,
		})
	);
}

#[test]
fn struct_unit() {
	#[derive(Inspect)]
	struct Foo;

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(0),
			align: Some(1),
			fields: vec![],
			tag: None,
			transparent: false,
		})
	);
}

#[test]
fn struct_multiple_fields() {
	#[derive(Inspect)]
	struct Foo {
		small: u8,
		medium: u16,
		veccy: Vec<u8>,
		recurse: Option<Box<Foo>>,
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![
				DefStructField {
					name: "small".to_string(),
					ser_name: "small".to_string(),
					type_id: 1,
					offset: size_of::<usize>() * 4 + size_of::<u16>(),
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "medium".to_string(),
					ser_name: "medium".to_string(),
					type_id: 2,
					offset: size_of::<usize>() * 4,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "veccy".to_string(),
					ser_name: "veccy".to_string(),
					type_id: 3,
					offset: 0,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "recurse".to_string(),
					ser_name: "recurse".to_string(),
					type_id: 4,
					offset: size_of::<usize>() * 3,
					flatten: false,
					skip: false,
				}
			],
			tag: None,
			transparent: false,
		})
	);

	let field_ids = get_field_ids(&type_defs[0]);
	assert_eq!(type_defs[field_ids[0]].name(), "u8");
	assert_eq!(type_defs[field_ids[1]].name(), "u16");
	assert_eq!(type_defs[field_ids[2]].name(), "Vec<u8>");
	assert_eq!(type_defs[field_ids[3]].name(), "Option<Box<Foo>>");
}

#[test]
fn struct_raw_identifier_field_name() {
	#[derive(Inspect)]
	struct Foo {
		r#type: u8,
		r#enum: u8,
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![
				DefStructField {
					name: "type".to_string(),
					ser_name: "type".to_string(),
					type_id: 1,
					offset: 0,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "enum".to_string(),
					ser_name: "enum".to_string(),
					type_id: 1,
					offset: 1,
					flatten: false,
					skip: false,
				}
			],
			tag: None,
			transparent: false,
		})
	);

	let field_ids = get_field_ids(&type_defs[0]);
	assert_eq!(type_defs[field_ids[0]].name(), "u8");
}

#[test]
fn tuple_struct_empty() {
	#[derive(Inspect)]
	struct Foo();

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(0),
			align: Some(1),
			fields: vec![],
			tag: None,
			transparent: false,
		})
	);
}

#[test]
fn tuple_struct_single_field() {
	#[derive(Inspect)]
	struct Foo(u8);

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "0".to_string(),
				ser_name: "0".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
				skip: false,
			}],
			tag: None,
			transparent: false,
		})
	);

	let field_ids = get_field_ids(&type_defs[0]);
	assert_eq!(type_defs[field_ids[0]].name(), "u8");
}

#[test]
fn tuple_struct_multiple_fields() {
	#[derive(Inspect)]
	struct Foo(u8, u16, Vec<u8>, Option<Box<Foo>>);

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![
				DefStructField {
					name: "0".to_string(),
					ser_name: "0".to_string(),
					type_id: 1,
					offset: size_of::<usize>() * 4 + size_of::<u16>(),
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "1".to_string(),
					ser_name: "1".to_string(),
					type_id: 2,
					offset: size_of::<usize>() * 4,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "2".to_string(),
					ser_name: "2".to_string(),
					type_id: 3,
					offset: 0,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "3".to_string(),
					ser_name: "3".to_string(),
					type_id: 4,
					offset: size_of::<usize>() * 3,
					flatten: false,
					skip: false,
				}
			],
			tag: None,
			transparent: false,
		})
	);

	let field_ids = get_field_ids(&type_defs[0]);
	assert_eq!(type_defs[field_ids[0]].name(), "u8");
	assert_eq!(type_defs[field_ids[1]].name(), "u16");
	assert_eq!(type_defs[field_ids[2]].name(), "Vec<u8>");
	assert_eq!(type_defs[field_ids[3]].name(), "Option<Box<Foo>>");
}

#[test]
fn tuple_struct_with_serde_transparent() {
	#[derive(Inspect)]
	#[serde(transparent)]
	struct Foo(u8);

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "0".to_string(),
				ser_name: "0".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
				skip: false,
			}],
			tag: None,
			transparent: true,
		})
	);

	let field_ids = get_field_ids(&type_defs[0]);
	assert_eq!(type_defs[field_ids[0]].name(), "u8");
}

#[test]
fn struct_generic_one_type_param() {
	#[derive(Inspect)]
	struct Foo {
		big: Bar<u32>,
		small: Bar<u8>,
	}

	#[derive(Inspect)]
	struct Bar<T> {
		inner: T,
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![
				DefStructField {
					name: "big".to_string(),
					ser_name: "big".to_string(),
					type_id: 1,
					offset: 0,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "small".to_string(),
					ser_name: "small".to_string(),
					type_id: 3,
					offset: size_of::<Bar<u32>>(),
					flatten: false,
					skip: false,
				}
			],
			tag: None,
			transparent: false,
		})
	);

	let foo_field_ids = get_field_ids(&type_defs[0]);
	let bar_u32_def = &type_defs[foo_field_ids[0]];
	assert_eq!(
		bar_u32_def,
		&DefType::Struct(DefStruct {
			name: "Bar<u32>".to_string(),
			ser_name: "Bar<u32>".to_string(),
			size: Some(size_of::<Bar<u32>>()),
			align: Some(align_of::<Bar<u32>>()),
			fields: vec![DefStructField {
				name: "inner".to_string(),
				ser_name: "inner".to_string(),
				type_id: 2,
				offset: 0,
				flatten: false,
				skip: false,
			}],
			tag: None,
			transparent: false,
		})
	);

	let u32_id = get_field_ids(bar_u32_def)[0];
	assert_eq!(type_defs[u32_id].name(), "u32");

	let bar_u8_def = &type_defs[foo_field_ids[1]];
	assert_eq!(
		bar_u8_def,
		&DefType::Struct(DefStruct {
			name: "Bar<u8>".to_string(),
			ser_name: "Bar<u8>".to_string(),
			size: Some(size_of::<Bar<u8>>()),
			align: Some(align_of::<Bar<u8>>()),
			fields: vec![DefStructField {
				name: "inner".to_string(),
				ser_name: "inner".to_string(),
				type_id: 4,
				offset: 0,
				flatten: false,
				skip: false,
			}],
			tag: None,
			transparent: false,
		})
	);

	let u8_id = get_field_ids(bar_u8_def)[0];
	assert_eq!(type_defs[u8_id].name(), "u8");
}

#[test]
fn struct_generic_two_type_params() {
	#[derive(Inspect)]
	struct Foo {
		big: Bar<u64, u32>,
		small: Bar<u16, u8>,
	}

	#[derive(Inspect)]
	struct Bar<T, U> {
		one: T,
		two: U,
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![
				DefStructField {
					name: "big".to_string(),
					ser_name: "big".to_string(),
					type_id: 1,
					offset: 0,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "small".to_string(),
					ser_name: "small".to_string(),
					type_id: 4,
					offset: size_of::<Bar<u64, u32>>(),
					flatten: false,
					skip: false,
				}
			],
			tag: None,
			transparent: false,
		})
	);

	let foo_field_ids = get_field_ids(&type_defs[0]);
	let bar_u64_u32_def = &type_defs[foo_field_ids[0]];
	assert_eq!(
		bar_u64_u32_def,
		&DefType::Struct(DefStruct {
			name: "Bar<u64,u32>".to_string(),
			ser_name: "Bar<u64,u32>".to_string(),
			size: Some(size_of::<Bar<u64, u32>>()),
			align: Some(align_of::<Bar<u64, u32>>()),
			fields: vec![
				DefStructField {
					name: "one".to_string(),
					ser_name: "one".to_string(),
					type_id: 2,
					offset: 0,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "two".to_string(),
					ser_name: "two".to_string(),
					type_id: 3,
					offset: size_of::<u64>(),
					flatten: false,
					skip: false,
				}
			],
			tag: None,
			transparent: false,
		})
	);

	let bar_u64_u32_field_ids = get_field_ids(bar_u64_u32_def);
	assert_eq!(type_defs[bar_u64_u32_field_ids[0]].name(), "u64");
	assert_eq!(type_defs[bar_u64_u32_field_ids[1]].name(), "u32");

	let bar_u16_u8_def = &type_defs[foo_field_ids[1]];
	assert_eq!(
		bar_u16_u8_def,
		&DefType::Struct(DefStruct {
			name: "Bar<u16,u8>".to_string(),
			ser_name: "Bar<u16,u8>".to_string(),
			size: Some(size_of::<Bar<u16, u8>>()),
			align: Some(align_of::<Bar<u16, u8>>()),
			fields: vec![
				DefStructField {
					name: "one".to_string(),
					ser_name: "one".to_string(),
					type_id: 5,
					offset: 0,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "two".to_string(),
					ser_name: "two".to_string(),
					type_id: 6,
					offset: size_of::<u16>(),
					flatten: false,
					skip: false,
				}
			],
			tag: None,
			transparent: false,
		})
	);

	let bar_u16_u8_field_ids = get_field_ids(bar_u16_u8_def);
	assert_eq!(type_defs[bar_u16_u8_field_ids[0]].name(), "u16");
	assert_eq!(type_defs[bar_u16_u8_field_ids[1]].name(), "u8");
}

#[test]
fn struct_with_serde_type_rename() {
	#[derive(Inspect)]
	#[serde(rename = "Bar")]
	struct Foo {
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Bar".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "num".to_string(),
				ser_name: "num".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
				skip: false,
			}],
			tag: None,
			transparent: false,
		})
	);
}

#[test]
fn struct_with_serde_tag() {
	#[derive(Inspect)]
	#[serde(tag = "type")]
	struct Foo {
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "num".to_string(),
				ser_name: "num".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
				skip: false,
			}],
			tag: Some("type".to_string()),
			transparent: false,
		})
	);
}

#[test]
fn struct_with_serde_transparent() {
	#[derive(Inspect)]
	#[serde(transparent)]
	struct Foo {
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "num".to_string(),
				ser_name: "num".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
				skip: false,
			}],
			tag: None,
			transparent: true,
		})
	);
}

#[test]
fn struct_with_serde_field_rename() {
	#[derive(Inspect)]
	struct Foo {
		#[serde(rename = "bar")]
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "num".to_string(),
				ser_name: "bar".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
				skip: false,
			}],
			tag: None,
			transparent: false,
		})
	);
}

#[test]
fn struct_with_serde_fields_rename_all() {
	#[derive(Inspect)]
	#[serde(rename_all = "camelCase")]
	struct Foo {
		field_one: u8,
		r#field_two: u8,
		#[serde(rename = "field_three_oh_yes")]
		field_three: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![
				DefStructField {
					name: "field_one".to_string(),
					ser_name: "fieldOne".to_string(),
					type_id: 1,
					offset: 0,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "field_two".to_string(),
					ser_name: "fieldTwo".to_string(),
					type_id: 1,
					offset: 1,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "field_three".to_string(),
					ser_name: "field_three_oh_yes".to_string(),
					type_id: 1,
					offset: 2,
					flatten: false,
					skip: false,
				},
			],
			tag: None,
			transparent: false,
		})
	);
}

#[test]
fn struct_with_serde_field_flatten() {
	#[derive(Inspect)]
	struct Foo {
		#[serde(flatten)]
		bar: Bar,
	}

	#[derive(Inspect)]
	struct Bar {
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "bar".to_string(),
				ser_name: "bar".to_string(),
				type_id: 1,
				offset: 0,
				flatten: true,
				skip: false,
			}],
			tag: None,
			transparent: false,
		})
	);
}

#[test]
fn struct_with_serde_field_skip() {
	#[derive(Inspect)]
	struct Foo {
		#[serde(skip)]
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "num".to_string(),
				ser_name: "num".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
				skip: true,
			}],
			tag: None,
			transparent: false,
		})
	);
}

#[test]
fn struct_with_serde_field_rename_and_flatten() {
	#[derive(Inspect)]
	struct Foo {
		#[serde(flatten, rename = "qux")]
		bar: Bar,
	}

	#[derive(Inspect)]
	struct Bar {
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "bar".to_string(),
				ser_name: "qux".to_string(),
				type_id: 1,
				offset: 0,
				flatten: true,
				skip: false,
			}],
			tag: None,
			transparent: false,
		})
	);
}

#[test]
fn struct_with_serde_field_default() {
	#[derive(Inspect)]
	struct Foo {
		#[serde(default)]
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![DefStructField {
				name: "num".to_string(),
				ser_name: "num".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
				skip: false,
			}],
			tag: None,
			transparent: false,
		})
	);
}

fn get_field_ids(struct_def: &DefType) -> Vec<usize> {
	struct_def
		.to_struct()
		.unwrap()
		.fields
		.iter()
		.map(|field| field.type_id as usize)
		.collect()
}
