mod clock;
mod colour;
mod components;
mod debug;
mod hello_world;
mod home;
mod settings;
mod transition;

pub use clock::*;
pub use colour::*;
pub use debug::*;
pub use hello_world::*;
pub use home::*;
pub use settings::*;
pub use transition::*;

// #[macro_export]
// macro_rules! define_options {
//     ($($name:expr => $fun:expr; $fn_ident:ident),*$(,)?) => {
//         #[allow(dead_code)]
//         const LIST_STRINGS: &[&str] = &[$($name),*];
//         type CreatePage = fn() -> ::alloc::boxed::Box<dyn crate::page::Page>;
//         $(
//             fn $fn_ident() -> ::alloc::boxed::Box<dyn crate::page::Page> {
//                 ::alloc::boxed::Box::new($fun())
//             }
//         )*

//         #[allow(dead_code)]
//         const LIST_CONSTRUCTORS: &[CreatePage] = &[$($fn_ident),*];
//     };
// }

#[macro_export]
macro_rules! define_options {
    (self => $type:ty, $($name:expr => $fun:expr),*$(,)?) => {
        #[allow(dead_code)]
        const LIST_STRINGS: &[&str] = &[$($name),*];
        type CreatePage = fn(&mut $type) -> ::core::option::Option<::alloc::boxed::Box<dyn crate::page::Page>>;

        ::paste::paste! {
            $(
                #[allow(non_snake_case)]
                fn [<option_ $name>](arg: &mut $type) -> ::core::option::Option<::alloc::boxed::Box<dyn crate::page::Page>> {
                    let func: CreatePage = $fun;
                    func(arg)
                }
            )*
        }

        ::paste::paste! {
            #[allow(dead_code)]
            const LIST_CONSTRUCTORS: &[CreatePage] = &[
                $(
                    [<option_ $name>],
                )*
            ];
        }
    };

    ($($name:expr => $fun:expr),*$(,)?) => {
        #[allow(dead_code)]
        const LIST_STRINGS: &[&str] = &[$($name),*];
        type CreatePage = fn(&mut ::alloc::vec::Vec<::alloc::boxed::Box<dyn FnOnce()>>) -> ::core::option::Option<::alloc::boxed::Box<dyn crate::page::Page>>;

        ::paste::paste! {
            $(
                #[allow(non_snake_case)]
                fn [<option_ $name>](arg: &mut ::alloc::vec::Vec<::alloc::boxed::Box<dyn FnOnce()>>) -> ::core::option::Option<::alloc::boxed::Box<dyn crate::page::Page>> {
                    if let Some(page) = $fun(arg) {
                        return Some(::alloc::boxed::Box::new(page));
                    }

                    None::<::alloc::boxed::Box<dyn crate::page::Page>>
                }
            )*
        }

        ::paste::paste! {
            #[allow(dead_code)]
            const LIST_CONSTRUCTORS: &[CreatePage] = &[
                $(
                    [<option_ $name>],
                )*
            ];
        }
    };
}

#[macro_export]
macro_rules! call_option {
    ($idx:expr, $acts:expr, $ctx:expr) => {
        if let Some(page) = $ctx[$idx]($acts) {
            return Some(page);
        }
    };

    ($idx: expr, $self:expr, $ctx:expr) => {
        if let Some(page) = $ctx[$idx]($self) {
            return Some(page);
        }
    };
}
