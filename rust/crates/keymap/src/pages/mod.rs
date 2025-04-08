mod boot;
mod clock;
mod colour;
mod components;
mod debug;
mod hello_world;
mod home;
mod settings;
mod startup;
mod transition;

pub use boot::*;
pub use clock::*;
pub use colour::*;
pub use debug::*;
pub use hello_world::*;
pub use home::*;
pub use settings::*;
pub use startup::*;
pub use transition::*;

pub type Actions = ::alloc::vec::Vec<::alloc::boxed::Box<dyn FnOnce()>>;

#[macro_export]
macro_rules! define_options {
    (self => $type:ty, $($name:expr, $i:ident => $fun:expr),*$(,)?) => {
        #[allow(dead_code)]
        const LIST_STRINGS: &[&str] = &[$($name),*];
        type CreatePage = fn(&mut $type) -> ::core::option::Option<::alloc::boxed::Box<dyn $crate::page::Page>>;

        $(
            #[allow(non_snake_case)]
            fn $i(arg: &mut $type) -> ::core::option::Option<::alloc::boxed::Box<dyn $crate::page::Page>> {
                let func: CreatePage = $fun;
                func(arg)
            }
        )*


        #[allow(dead_code)]
        const LIST_CONSTRUCTORS: &[CreatePage] = &[
            $(
                $i,
            )*
        ];

    };

    ($($name:expr, $i:ident => $fun:expr),*$(,)?) => {
        #[allow(dead_code)]
        const LIST_STRINGS: &[&str] = &[$($name),*];
        type CreatePage = fn(&mut ::alloc::vec::Vec<::alloc::boxed::Box<dyn FnOnce()>>) -> ::core::option::Option<::alloc::boxed::Box<dyn $crate::page::Page>>;

        $(
            #[allow(non_snake_case)]
            fn $i(arg: &mut $crate::pages::Actions) -> ::core::option::Option<::alloc::boxed::Box<dyn $crate::page::Page>> {
                if let Some(page) = $fun(arg) {
                    return Some(::alloc::boxed::Box::new(page));
                }

                None::<::alloc::boxed::Box<dyn $crate::page::Page>>
            }
        )*

        #[allow(dead_code)]
        const LIST_CONSTRUCTORS: &[CreatePage] = &[
            $(
                $i,
            )*
        ];
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
