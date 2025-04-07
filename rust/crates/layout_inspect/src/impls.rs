use std::{
    cell::{Cell, RefCell},
    marker::PhantomData,
    mem::{align_of, size_of},
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    defs::{
        DefArc, DefBox, DefCell, DefMutex, DefOption, DefPhantomData, DefRc, DefRefCell, DefResult,
        DefRwLock, DefStr, DefString, DefType, DefVec,
    },
    Inspect, TypesCollector,
};

impl Inspect for String {
    fn name() -> String {
        "String".to_string()
    }

    fn size() -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn align() -> Option<usize> {
        Some(align_of::<Self>())
    }

    fn def(_collector: &mut TypesCollector) -> DefType {
        DefType::String(DefString {
            name: Self::name(),
            size: Self::size().unwrap(),
            align: Self::align().unwrap(),
        })
    }
}

impl Inspect for str {
    fn name() -> String {
        "str".to_string()
    }

    fn size() -> Option<usize> {
        None
    }

    fn align() -> Option<usize> {
        Some(align_of::<u8>())
    }

    fn def(_collector: &mut TypesCollector) -> DefType {
        DefType::Str(DefStr {
            name: Self::name(),
            size: Self::size(),
            align: Self::align().unwrap(),
        })
    }
}

macro_rules! single_type_param {
    ($name:ident, $def:ident) => {
        impl<T: Inspect> Inspect for $name<T> {
            fn name() -> String {
                stringify!($name).to_string() + "<" + &T::name() + ">"
            }

            fn size() -> Option<usize> {
                Some(size_of::<Self>())
            }

            fn align() -> Option<usize> {
                Some(align_of::<Self>())
            }

            fn def(collector: &mut TypesCollector) -> DefType {
                DefType::$name($def {
                    name: Self::name(),
                    size: Self::size().unwrap(),
                    align: Self::align().unwrap(),
                    value_type_id: collector.collect::<T>(),
                })
            }
        }
    };
}

single_type_param!(Box, DefBox);
single_type_param!(Vec, DefVec);
single_type_param!(Option, DefOption);
single_type_param!(PhantomData, DefPhantomData);
single_type_param!(Cell, DefCell);
single_type_param!(RefCell, DefRefCell);
single_type_param!(Mutex, DefMutex);
single_type_param!(RwLock, DefRwLock);
single_type_param!(Rc, DefRc);
single_type_param!(Arc, DefArc);

macro_rules! double_type_param {
    ($name:ident, $def:ident, $field1:ident, $field2:ident) => {
        impl<T: Inspect, T2: Inspect> Inspect for $name<T, T2> {
            fn name() -> String {
                stringify!($name).to_string() + "<" + &T::name() + "," + &T2::name() + ">"
            }

            fn size() -> Option<usize> {
                Some(size_of::<Self>())
            }

            fn align() -> Option<usize> {
                Some(align_of::<Self>())
            }

            fn def(collector: &mut TypesCollector) -> DefType {
                DefType::$name($def {
                    name: Self::name(),
                    size: Self::size().unwrap(),
                    align: Self::align().unwrap(),
                    $field1: collector.collect::<T>(),
                    $field2: collector.collect::<T2>(),
                })
            }
        }
    };
}

double_type_param!(Result, DefResult, ok_type_id, err_type_id);
