#[path = "units_display_name_v1_marker.rs.data"]
mod data;



extern crate alloc;

pub mod icu {
    pub mod experimental {
        pub mod dimension {
            pub mod provider {
                pub mod units {
                    pub struct UnitsDisplayNameV1<'a> {
                        pub patterns:
                            crate::icu::experimental::relativetime::provider::PluralPatterns<'a>,
                    }
                }
            }
        }
        pub mod relativetime {
            pub mod provider {
                pub struct PluralPatterns<'a> {
                    pub strings: crate::icu::plurals::provider::PluralElementsPackedCow<'a>,
                    pub _phantom: core::marker::PhantomData<&'a ()>,
                }
            }
        }
    }

    pub mod plurals {
        pub mod provider {
            pub struct PluralElementsPackedCow<'a> {
                pub elements: alloc::borrow::Cow<'a, PluralElementsPackedULE>,
            }
            #[repr(transparent)]
            pub struct PluralElementsPackedULE([u8]);
            impl PluralElementsPackedULE {
                pub const unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
                    // Safety: repr(transparent)
                    core::mem::transmute(bytes)
                }
            }
            impl ToOwned for PluralElementsPackedULE {
                type Owned = alloc::boxed::Box<Self>;
                fn to_owned(&self) -> Self::Owned {
                    unimplemented!()
                }
            }
        }
    }
}

pub mod icu_provider_baked {
    pub mod zerotrie {
        pub struct Data<T: 'static> {
            pub values: &'static [T; 25203],
        }
        pub struct ZeroTrieSimpleAscii {
            pub store: &'static [u8],
        }
    }
}


pub struct Baked;

impl_units_display_name_v1_marker!(Baked);