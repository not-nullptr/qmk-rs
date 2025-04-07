use syn::{AttrStyle, Attribute, Lit, Meta, NestedMeta};

#[derive(Default)]
pub struct SerdeAttrs {
	pub rename: Option<String>,
	pub rename_all: Option<String>,
	pub tag: Option<String>,
	pub content: Option<String>,
	pub untagged: bool,
	pub flatten: bool,
	pub skip: bool,
	pub transparent: bool,
}

/// Parse `#[serde()]` attributes for a struct / enum / struct field / enum
/// variant
pub fn get_serde_attrs(attrs: &Vec<Attribute>, host: &str) -> SerdeAttrs {
	let mut out = SerdeAttrs::default();
	for attr in attrs {
		if attr.style != AttrStyle::Outer || !attr.path.is_ident("serde") {
			continue;
		}

		let meta = attr.parse_meta().unwrap();
		let list = if let Meta::List(list) = meta {
			list
		} else {
			continue;
		};

		for item in list.nested {
			let meta = if let NestedMeta::Meta(meta) = item {
				meta
			} else {
				continue;
			};

			match meta {
				Meta::NameValue(name_value) => {
					let name = if let Some(name) = name_value.path.get_ident() {
						name.to_string()
					} else {
						continue;
					};

					let value = if let Lit::Str(value) = name_value.lit {
						value.value()
					} else {
						continue;
					};

					match &*name {
						"rename" => {
							if out.rename.is_some() {
								panic!("Multiple serde `rename` tags on same {}", host);
							}
							out.rename = Some(value);
						}
						"rename_all" => {
							if out.rename_all.is_some() {
								panic!("Multiple serde `rename_all` tags on same {}", host);
							}
							out.rename_all = Some(value);
						}
						"tag" => {
							if out.tag.is_some() {
								panic!("Multiple serde `tag` tags on same {}", host);
							}
							out.tag = Some(value);
						}
						"content" => {
							if out.content.is_some() {
								panic!("Multiple serde `content` tags on same {}", host);
							}
							out.content = Some(value);
						}
						_ => {}
					}
				}
				Meta::Path(path) => {
					let name = if let Some(name) = path.get_ident() {
						name.to_string()
					} else {
						continue;
					};

					match &*name {
						"flatten" => {
							out.flatten = true;
						}
						"skip" => {
							out.skip = true;
						}
						"untagged" => {
							out.untagged = true;
						}
						"transparent" => {
							out.transparent = true;
						}
						_ => {}
					}
				}
				_ => {}
			}
		}
	}
	out
}
