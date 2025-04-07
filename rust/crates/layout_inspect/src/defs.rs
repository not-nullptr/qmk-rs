use macro_rules_attribute::apply;

use super::TypeId;

#[derive(PartialEq, Eq, Hash, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "kind", rename_all = "camelCase")
)]
pub enum DefType {
    Primitive(DefPrimitive),
    Struct(DefStruct),
    Enum(DefEnum),
    String(DefString),
    Str(DefStr),
    StrSlice(DefStrSlice),
    Box(DefBox),
    Vec(DefVec),
    Option(DefOption),
    PhantomData(DefPhantomData),
    Cell(DefCell),
    RefCell(DefRefCell),
    Mutex(DefMutex),
    RwLock(DefRwLock),
    Rc(DefRc),
    Arc(DefArc),
    Result(DefResult),
}

macro_rules! getter {
    ($field:ident, $rtn_type:ty, $out:expr) => {
        getter!($field, $rtn_type, $out, $out, $out);
    };
    ($field:ident, $rtn_type:ty, $out:expr, $unsized_out:expr, $unsized_unaligned:expr) => {
        pub fn $field(&self) -> $rtn_type {
            match &self {
                DefType::Primitive(DefPrimitive { $field, .. }) => $out,
                DefType::Struct(DefStruct { $field, .. }) => $unsized_unaligned,
                DefType::Enum(DefEnum { $field, .. }) => $out,
                DefType::String(DefString { $field, .. }) => $out,
                DefType::Str(DefStr { $field, .. }) => $unsized_out,
                DefType::StrSlice(DefStrSlice { $field, .. }) => $out,
                DefType::Box(DefBox { $field, .. }) => $out,
                DefType::Vec(DefVec { $field, .. }) => $out,
                DefType::Option(DefOption { $field, .. }) => $out,
                DefType::PhantomData(DefPhantomData { $field, .. }) => $out,
                DefType::Cell(DefCell { $field, .. }) => $out,
                DefType::RefCell(DefRefCell { $field, .. }) => $out,
                DefType::Mutex(DefMutex { $field, .. }) => $out,
                DefType::RwLock(DefRwLock { $field, .. }) => $out,
                DefType::Rc(DefRc { $field, .. }) => $out,
                DefType::Arc(DefArc { $field, .. }) => $out,
                DefType::Result(DefResult { $field, .. }) => $out,
            }
        }
    };
}

macro_rules! to_methods {
    ($name:ident, $def:ty, $into:ident, $ref:ident) => {
        pub fn $into(self) -> Option<$def> {
            match self {
                DefType::$name(def) => Some(def),
                _ => None,
            }
        }

        pub fn $ref(&self) -> Option<&$def> {
            match self {
                DefType::$name(def) => Some(def),
                _ => None,
            }
        }
    };
}

impl DefType {
    getter!(name, &str, &name[..]);

    getter!(size, Option<usize>, Some(*size), *size, *size);

    getter!(align, Option<usize>, Some(*align), Some(*align), *align);

    to_methods!(Primitive, DefPrimitive, into_primitive, to_primitive);

    to_methods!(Struct, DefStruct, into_struct, to_struct);

    to_methods!(Enum, DefEnum, into_enum, to_enum);

    to_methods!(String, DefString, into_string, to_string);

    to_methods!(Str, DefStr, into_str, to_str);

    to_methods!(StrSlice, DefStrSlice, into_str_slice, to_str_slice);

    to_methods!(Box, DefBox, into_box, to_box);

    to_methods!(Vec, DefVec, into_vec, to_vec);

    to_methods!(Option, DefOption, into_option, to_option);

    to_methods!(
        PhantomData,
        DefPhantomData,
        into_phantom_data,
        to_phantom_data
    );

    to_methods!(Cell, DefCell, into_cell, to_cell);

    to_methods!(RefCell, DefRefCell, into_ref_cell, to_ref_cell);

    to_methods!(Mutex, DefMutex, into_mutex, to_mutex);

    to_methods!(RwLock, DefRwLock, into_rw_lock, to_rw_lock);

    to_methods!(Rc, DefRc, into_rc, to_rc);

    to_methods!(Arc, DefArc, into_arc, to_arc);

    to_methods!(Result, DefResult, into_result, to_result);
}

macro_rules! def {
    ($def:item) => {
        #[derive(PartialEq, Eq, Hash, Debug)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
            serde(rename_all = "camelCase")
        )]
        $def
    };
}

#[apply(def)]
pub struct DefPrimitive {
    pub name: String,
    pub size: usize,
    pub align: usize,
}

#[apply(def)]
pub struct DefStruct {
    pub name: String,
    pub ser_name: String,
    pub size: Option<usize>,
    pub align: Option<usize>,
    pub fields: Vec<DefStructField>,
    pub tag: Option<String>,
    pub transparent: bool,
}

#[apply(def)]
pub struct DefStructField {
    pub name: String,
    pub ser_name: String,
    pub type_id: TypeId,
    pub offset: usize,
    pub flatten: bool,
    pub skip: bool,
}

#[apply(def)]
pub struct DefEnum {
    pub name: String,
    pub ser_name: String,
    pub size: usize,
    pub align: usize,
    pub variants: Vec<DefEnumVariant>,
    pub tag: DefEnumTag,
}

#[apply(def)]
pub struct DefEnumVariant {
    pub name: String,
    pub discriminant: u64,
    // TODO: Need `offset` here or `discriminant_size` in `DefEnum`
    pub ser_value: Option<String>,
    pub value_type_id: Option<TypeId>,
}

#[apply(def)]
#[cfg_attr(feature = "serde", serde(tag = "kind", content = "data"))]
pub enum DefEnumTag {
    None,
    Tag(String),
    TagAndContent { tag: String, content: String },
    Untagged,
}

pub fn foo() -> DefEnumTag {
    DefEnumTag::TagAndContent {
        tag: "tt".to_string(),
        content: "cc".to_string(),
    }
}

macro_rules! single_type_param {
    ($def:ident) => {
        #[apply(def)]
        pub struct $def {
            pub name: String,
            pub size: usize,
            pub align: usize,
            pub value_type_id: TypeId,
        }
    };
}

single_type_param!(DefBox);
single_type_param!(DefVec);
single_type_param!(DefOption);
single_type_param!(DefPhantomData);
single_type_param!(DefCell);
single_type_param!(DefRefCell);
single_type_param!(DefMutex);
single_type_param!(DefRwLock);
single_type_param!(DefRc);
single_type_param!(DefArc);

macro_rules! double_type_param {
    ($def:ident, $field1:ident, $field2:ident) => {
        #[apply(def)]
        pub struct $def {
            pub name: String,
            pub size: usize,
            pub align: usize,
            pub $field1: TypeId,
            pub $field2: TypeId,
        }
    };
}

double_type_param!(DefResult, ok_type_id, err_type_id);

#[apply(def)]
pub struct DefString {
    pub name: String,
    pub size: usize,
    pub align: usize,
}

#[apply(def)]
pub struct DefStr {
    pub name: String,
    pub size: Option<usize>,
    pub align: usize,
}

#[apply(def)]
pub struct DefStrSlice {
    pub name: String,
    pub size: usize,
    pub align: usize,
}
