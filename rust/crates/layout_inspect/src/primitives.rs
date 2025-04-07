use std::{
	mem::{align_of, size_of},
	num::{
		NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
		NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
	},
};

use crate::{
	defs::{DefPrimitive, DefType},
	Inspect, TypesCollector,
};

macro_rules! primitive {
	($type:ty) => {
		impl Inspect for $type {
			fn name() -> String {
				stringify!($type).to_string()
			}

			fn size() -> Option<usize> {
				Some(size_of::<Self>())
			}

			fn align() -> Option<usize> {
				Some(align_of::<Self>())
			}

			fn def(_collector: &mut TypesCollector) -> DefType {
				DefType::Primitive(DefPrimitive {
					name: Self::name(),
					size: Self::size().unwrap(),
					align: Self::align().unwrap(),
				})
			}
		}
	};
}

primitive!(u8);
primitive!(u16);
primitive!(u32);
primitive!(u64);
primitive!(u128);
primitive!(usize);

primitive!(i8);
primitive!(i16);
primitive!(i32);
primitive!(i64);
primitive!(i128);
primitive!(isize);

primitive!(NonZeroU8);
primitive!(NonZeroU16);
primitive!(NonZeroU32);
primitive!(NonZeroU64);
primitive!(NonZeroU128);
primitive!(NonZeroUsize);

primitive!(NonZeroI8);
primitive!(NonZeroI16);
primitive!(NonZeroI32);
primitive!(NonZeroI64);
primitive!(NonZeroI128);
primitive!(NonZeroIsize);

primitive!(f32);
primitive!(f64);

primitive!(bool);
primitive!(char);

primitive!(());
