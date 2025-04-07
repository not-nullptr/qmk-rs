use convert_case::{Case, Casing};
use syn::Ident;

/// Get struct field / enum variant name.
pub fn get_ident_name(ident: &Ident) -> String {
	let mut name = ident.to_string();
	if name.starts_with("r#") {
		name = name[2..].to_string();
	}
	name
}

/// Get struct field / enum variant serialization name.
/// `#[serde(rename)` takes priority.
/// Apply `#[serde(rename_all)` transform.
pub fn get_ser_name(name: &str, renamed: &Option<String>, rename_all: &Option<String>) -> String {
	if let Some(name) = renamed {
		name.clone()
	} else if let Some(rename_all) = rename_all {
		rename(name, rename_all)
	} else {
		name.to_string()
	}
}

/// Rename field/variant name according to `#[serde(rename_all = "...")]` attr
fn rename(s: &str, rename_all: &str) -> String {
	match rename_all {
		"lowercase" => s.to_lowercase(),
		"UPPERCASE" => s.to_uppercase(),
		"PascalCase" => s.to_case(Case::Pascal),
		"camelCase" => s.to_case(Case::Camel),
		"snake_case" => s.to_case(Case::Snake),
		"SCREAMING_SNAKE_CASE" => s.to_case(Case::UpperSnake),
		"kebab-case" => s.to_case(Case::Kebab),
		"SCREAMING-KEBAB-CASE" => s.to_case(Case::UpperKebab),
		_ => panic!("Unexpected `serde(rename_all)` attribute `{}`", rename_all),
	}
}
