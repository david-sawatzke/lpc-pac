#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTPUTDIRCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SETCLR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR0R {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR0R::INDEPENDENT => 0,
            SETCLR0R::REVERSED => 1,
            SETCLR0R::REVERSED_H => 2,
            SETCLR0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR0R {
        match value {
            0 => SETCLR0R::INDEPENDENT,
            1 => SETCLR0R::REVERSED,
            2 => SETCLR0R::REVERSED_H,
            i => SETCLR0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR0R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline]
    pub fn is_reversed(&self) -> bool {
        *self == SETCLR0R::REVERSED
    }
    #[doc = "Checks if the value of the field is `REVERSED_H`"]
    #[inline]
    pub fn is_reversed_h(&self) -> bool {
        *self == SETCLR0R::REVERSED_H
    }
}
#[doc = "Possible values of the field `SETCLR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR1R {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR1R::INDEPENDENT => 0,
            SETCLR1R::REVERSED => 1,
            SETCLR1R::REVERSED_H => 2,
            SETCLR1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR1R {
        match value {
            0 => SETCLR1R::INDEPENDENT,
            1 => SETCLR1R::REVERSED,
            2 => SETCLR1R::REVERSED_H,
            i => SETCLR1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR1R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline]
    pub fn is_reversed(&self) -> bool {
        *self == SETCLR1R::REVERSED
    }
    #[doc = "Checks if the value of the field is `REVERSED_H`"]
    #[inline]
    pub fn is_reversed_h(&self) -> bool {
        *self == SETCLR1R::REVERSED_H
    }
}
#[doc = "Possible values of the field `SETCLR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR2R {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR2R::INDEPENDENT => 0,
            SETCLR2R::REVERSED => 1,
            SETCLR2R::REVERSED_H => 2,
            SETCLR2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR2R {
        match value {
            0 => SETCLR2R::INDEPENDENT,
            1 => SETCLR2R::REVERSED,
            2 => SETCLR2R::REVERSED_H,
            i => SETCLR2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR2R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline]
    pub fn is_reversed(&self) -> bool {
        *self == SETCLR2R::REVERSED
    }
    #[doc = "Checks if the value of the field is `REVERSED_H`"]
    #[inline]
    pub fn is_reversed_h(&self) -> bool {
        *self == SETCLR2R::REVERSED_H
    }
}
#[doc = "Possible values of the field `SETCLR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR3R {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR3R::INDEPENDENT => 0,
            SETCLR3R::REVERSED => 1,
            SETCLR3R::REVERSED_H => 2,
            SETCLR3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR3R {
        match value {
            0 => SETCLR3R::INDEPENDENT,
            1 => SETCLR3R::REVERSED,
            2 => SETCLR3R::REVERSED_H,
            i => SETCLR3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR3R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline]
    pub fn is_reversed(&self) -> bool {
        *self == SETCLR3R::REVERSED
    }
    #[doc = "Checks if the value of the field is `REVERSED_H`"]
    #[inline]
    pub fn is_reversed_h(&self) -> bool {
        *self == SETCLR3R::REVERSED_H
    }
}
#[doc = "Possible values of the field `SETCLR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR4R {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR4R::INDEPENDENT => 0,
            SETCLR4R::REVERSED => 1,
            SETCLR4R::REVERSED_H => 2,
            SETCLR4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR4R {
        match value {
            0 => SETCLR4R::INDEPENDENT,
            1 => SETCLR4R::REVERSED,
            2 => SETCLR4R::REVERSED_H,
            i => SETCLR4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR4R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline]
    pub fn is_reversed(&self) -> bool {
        *self == SETCLR4R::REVERSED
    }
    #[doc = "Checks if the value of the field is `REVERSED_H`"]
    #[inline]
    pub fn is_reversed_h(&self) -> bool {
        *self == SETCLR4R::REVERSED_H
    }
}
#[doc = "Possible values of the field `SETCLR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR5R {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR5R::INDEPENDENT => 0,
            SETCLR5R::REVERSED => 1,
            SETCLR5R::REVERSED_H => 2,
            SETCLR5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR5R {
        match value {
            0 => SETCLR5R::INDEPENDENT,
            1 => SETCLR5R::REVERSED,
            2 => SETCLR5R::REVERSED_H,
            i => SETCLR5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR5R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline]
    pub fn is_reversed(&self) -> bool {
        *self == SETCLR5R::REVERSED
    }
    #[doc = "Checks if the value of the field is `REVERSED_H`"]
    #[inline]
    pub fn is_reversed_h(&self) -> bool {
        *self == SETCLR5R::REVERSED_H
    }
}
#[doc = "Values that can be written to the field `SETCLR0`"]
pub enum SETCLR0W {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
}
impl SETCLR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR0W::INDEPENDENT => 0,
            SETCLR0W::REVERSED => 1,
            SETCLR0W::REVERSED_H => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR0W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR0W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn reversed(self) -> &'a mut W {
        self.variant(SETCLR0W::REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn reversed_h(self) -> &'a mut W {
        self.variant(SETCLR0W::REVERSED_H)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR1`"]
pub enum SETCLR1W {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
}
impl SETCLR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR1W::INDEPENDENT => 0,
            SETCLR1W::REVERSED => 1,
            SETCLR1W::REVERSED_H => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR1W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR1W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn reversed(self) -> &'a mut W {
        self.variant(SETCLR1W::REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn reversed_h(self) -> &'a mut W {
        self.variant(SETCLR1W::REVERSED_H)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR2`"]
pub enum SETCLR2W {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
}
impl SETCLR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR2W::INDEPENDENT => 0,
            SETCLR2W::REVERSED => 1,
            SETCLR2W::REVERSED_H => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR2W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR2W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn reversed(self) -> &'a mut W {
        self.variant(SETCLR2W::REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn reversed_h(self) -> &'a mut W {
        self.variant(SETCLR2W::REVERSED_H)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR3`"]
pub enum SETCLR3W {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
}
impl SETCLR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR3W::INDEPENDENT => 0,
            SETCLR3W::REVERSED => 1,
            SETCLR3W::REVERSED_H => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR3W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR3W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn reversed(self) -> &'a mut W {
        self.variant(SETCLR3W::REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn reversed_h(self) -> &'a mut W {
        self.variant(SETCLR3W::REVERSED_H)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR4`"]
pub enum SETCLR4W {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
}
impl SETCLR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR4W::INDEPENDENT => 0,
            SETCLR4W::REVERSED => 1,
            SETCLR4W::REVERSED_H => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR4W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR4W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn reversed(self) -> &'a mut W {
        self.variant(SETCLR4W::REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn reversed_h(self) -> &'a mut W {
        self.variant(SETCLR4W::REVERSED_H)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR5`"]
pub enum SETCLR5W {
    #[doc = "Set and clear do not depend on any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    REVERSED_H,
}
impl SETCLR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR5W::INDEPENDENT => 0,
            SETCLR5W::REVERSED => 1,
            SETCLR5W::REVERSED_H => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR5W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR5W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn reversed(self) -> &'a mut W {
        self.variant(SETCLR5W::REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn reversed_h(self) -> &'a mut W {
        self.variant(SETCLR5W::REVERSED_H)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr0(&self) -> SETCLR0R {
        SETCLR0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr1(&self) -> SETCLR1R {
        SETCLR1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr2(&self) -> SETCLR2R {
        SETCLR2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr3(&self) -> SETCLR3R {
        SETCLR3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr4(&self) -> SETCLR4R {
        SETCLR4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr5(&self) -> SETCLR5R {
        SETCLR5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr0(&mut self) -> _SETCLR0W {
        _SETCLR0W { w: self }
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr1(&mut self) -> _SETCLR1W {
        _SETCLR1W { w: self }
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr2(&mut self) -> _SETCLR2W {
        _SETCLR2W { w: self }
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr3(&mut self) -> _SETCLR3W {
        _SETCLR3W { w: self }
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr4(&mut self) -> _SETCLR4W {
        _SETCLR4W { w: self }
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr5(&mut self) -> _SETCLR5W {
        _SETCLR5W { w: self }
    }
}
