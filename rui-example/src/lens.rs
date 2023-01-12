use std::marker::PhantomData;

use rui::Lens;

pub struct ZoomedLens<L1, L2, Mid> {
    l1: L1,
    l2: L2,
    mid: PhantomData<Mid>,
}

impl<L1: Clone, L2: Clone, Mid> Clone for ZoomedLens<L1, L2, Mid> {
    fn clone(&self) -> Self {
        Self {
            l1: self.l1.clone(),
            l2: self.l2.clone(),
            mid: PhantomData,
        }
    }
}

impl<L1: Copy, L2: Copy, Mid> Copy for ZoomedLens<L1, L2, Mid> {

}

impl<A, B: 'static, C, L1: Lens<A, B>, L2: Lens<B, C>> Lens<A, C> for ZoomedLens<L1, L2, B> {
    fn focus<'a>(&self, data: &'a A) -> &'a C {
        self.l2.focus(self.l1.focus(data))
    }

    fn focus_mut<'a>(&self, data: &'a mut A) -> &'a mut C {
        self.l2.focus_mut(self.l1.focus_mut(data))
    }
}

pub trait LensExt<_A, _B>: Sized {
    fn zoom<L2, Mid>(self, other: L2) -> ZoomedLens<Self, L2, Mid> {
        ZoomedLens {
            l1: self,
            l2: other,
            mid: PhantomData,
        }
    }
}

impl<A, B, L: Lens<A, B>> LensExt<A, B> for L {}

#[macro_export]
macro_rules! make_lens {
    ($from: ty, $to: ty, $field: ident) => {
        mod $field {
            #[derive(Clone, Copy)]
            #[allow(non_camel_case_types)]
            pub struct $field;
        }
        impl rui::Lens<$from, $to> for $field::$field {
            fn focus<'a>(&self, data: &'a $from) -> &'a $to {
                &data.$field
            }

            fn focus_mut<'a>(&self, data: &'a mut $from) -> &'a mut $to {
                &mut data.$field
            }
        }
        impl $from {
            fn $field() -> $field::$field {
                $field::$field
            }
        }
    }
}
pub use make_lens;

#[macro_export]
macro_rules! make_bind {
    ($from: ty, $to: ty, $field: ident) => {
        mod $field {
            #[derive(Clone, Copy)]
            #[allow(non_camel_case_types)]
            pub struct $field;

            pub(crate) trait Bind: ::rui::Binding<$from> {
                fn $field(self) -> ::rui::Map<Self, $field, $to, $from> {
                    ::rui::Map::new(self, $field)
                }
            }
            impl<B: ::rui::Binding<$from>> Bind for B {}
        }
        impl rui::Lens<$from, $to> for $field::$field {
            fn focus<'a>(&self, data: &'a $from) -> &'a $to {
                &data.$field
            }

            fn focus_mut<'a>(&self, data: &'a mut $from) -> &'a mut $to {
                &mut data.$field
            }
        }
    }
}
pub use make_bind;

pub struct Ix<T> {
    index: usize,
    phantom: PhantomData<T>,
}
impl<T> Ix<T> {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            phantom: PhantomData,
        }
    }
}
impl<T> Clone for Ix<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            phantom: PhantomData,
        }
    }
}
impl<T> Copy for Ix<T> {}

impl<T: 'static> Lens<Vec<T>, T> for Ix<T> {
    fn focus<'a>(&self, data: &'a Vec<T>) -> &'a T {
        &data[self.index]
    }

    fn focus_mut<'a>(&self, data: &'a mut Vec<T>) -> &'a mut T {
        &mut data[self.index]
    }
}
