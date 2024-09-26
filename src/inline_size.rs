use core::mem;

#[derive(Clone, Copy)]
#[repr(usize)]
pub(crate) enum InlineSize8 {
    _V1 = 1,
    _V2,
    _V3,
    _V4,
    _V5,
    _V6,
    _V7,
    _V8,
}

impl InlineSize8 {
    #[inline(always)]
    /// SAFETY: The caller is responsible to ensure value is in \[5, 8\].
    pub(crate) const unsafe fn transmute_from_usize(value: usize) -> Self {
        debug_assert!(value >= Self::_V1 as usize && value <= Self::_V8 as usize);
        mem::transmute::<usize, Self>(value)
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
pub(crate) enum InlineSize16 {
    _V9 = 9,
    _V10,
    _V11,
    _V12,
    _V13,
    _V14,
    _V15,
    _V16,
}

impl InlineSize16 {
    #[inline(always)]
    /// SAFETY: The caller is responsible to ensure value is in \[9, 16\].
    pub(crate) const unsafe fn transmute_from_usize(value: usize) -> Self {
        debug_assert!(value >= Self::_V9 as usize && value <= Self::_V16 as usize);
        mem::transmute::<usize, Self>(value)
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
pub(crate) enum InlineSize32 {
    _V17 = 17,
    _V18,
    _V19,
    _V20,
    _V21,
    _V22,
    _V23,
    _V24,
    _V25,
    _V26,
    _V27,
    _V28,
    _V29,
    _V30,
    _V31,
    _V32,
}

impl InlineSize32 {
    #[inline(always)]
    /// SAFETY: The caller is responsible to ensure value is in \[17, 32\].
    pub(crate) const unsafe fn transmute_from_usize(value: usize) -> Self {
        debug_assert!(value >= Self::_V17 as usize && value <= Self::_V32 as usize);
        mem::transmute::<usize, Self>(value)
    }
}
