use crate::{LiquidPoly, Vertex};
use paste::paste;
use std::ops::{Index, IndexMut, Range, RangeBounds};

macro_rules! poly_index {
    ($name:ident, $fetch_vec:ident) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(pub usize);

        impl From<$name> for usize {
            fn from(ix: $name) -> Self {
                ix.0
            }
        }

        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }

        impl Index<$name> for LiquidPoly {
            type Output = Vertex;

            fn index(&self, index: $name) -> &Self::Output {
                &self.$fetch_vec[index.0]
            }
        }

        impl Index<Range<$name>> for LiquidPoly {
            type Output = [Vertex];

            fn index(&self, range: Range<$name>) -> &Self::Output {
                &self.$fetch_vec[range.start.0..range.end.0]
            }
        }

        paste! {
            impl IndexMut<$name> for LiquidPoly {
                fn index_mut(&mut self, index: $name) -> &mut Self::Output {
                    &mut self.$fetch_vec[index.0]
                }
            }

            impl IndexMut<Range<$name>> for LiquidPoly {

                fn index_mut(&mut self, range: Range<$name>) -> &mut Self::Output {
                    &mut self.$fetch_vec[range.start.0..range.end.0]
                }
            }
        }
    };
}

poly_index!(Vix, vertices);
