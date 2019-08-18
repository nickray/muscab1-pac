#[doc = "Reader of register UARTIFLS"]
pub type R = crate::R<u32, super::UARTIFLS>;
#[doc = "Writer for register UARTIFLS"]
pub type W = crate::W<u32, super::UARTIFLS>;
#[doc = "Register UARTIFLS `reset()`'s with value 0x12"]
impl crate::ResetValue for super::UARTIFLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x12
    }
}
#[doc = "Possible values of the field `TXIFLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIFLSEL_A {
    #[doc = "Transmit FIFO becomes less than or equal to 1/8 full"]
    _18FULL,
    #[doc = "Transmit FIFO becomes less than or equal to 1/4 full"]
    _14FULL,
    #[doc = "Transmit FIFO becomes less than or equal to 1/2 full"]
    _12FULL,
    #[doc = "Transmit FIFO becomes less than or equal to 3/4 full"]
    _34FULL,
    #[doc = "Transmit FIFO becomes less than or equal to 7/8 full"]
    _78FULL,
}
impl From<TXIFLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXIFLSEL_A) -> Self {
        match variant {
            TXIFLSEL_A::_18FULL => 0,
            TXIFLSEL_A::_14FULL => 1,
            TXIFLSEL_A::_12FULL => 2,
            TXIFLSEL_A::_34FULL => 3,
            TXIFLSEL_A::_78FULL => 4,
        }
    }
}
#[doc = "Reader of field `TXIFLSEL`"]
pub type TXIFLSEL_R = crate::R<u8, TXIFLSEL_A>;
impl TXIFLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXIFLSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXIFLSEL_A::_18FULL),
            1 => Val(TXIFLSEL_A::_14FULL),
            2 => Val(TXIFLSEL_A::_12FULL),
            3 => Val(TXIFLSEL_A::_34FULL),
            4 => Val(TXIFLSEL_A::_78FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_18FULL`"]
    #[inline(always)]
    pub fn is_18full(&self) -> bool {
        *self == TXIFLSEL_A::_18FULL
    }
    #[doc = "Checks if the value of the field is `_14FULL`"]
    #[inline(always)]
    pub fn is_14full(&self) -> bool {
        *self == TXIFLSEL_A::_14FULL
    }
    #[doc = "Checks if the value of the field is `_12FULL`"]
    #[inline(always)]
    pub fn is_12full(&self) -> bool {
        *self == TXIFLSEL_A::_12FULL
    }
    #[doc = "Checks if the value of the field is `_34FULL`"]
    #[inline(always)]
    pub fn is_34full(&self) -> bool {
        *self == TXIFLSEL_A::_34FULL
    }
    #[doc = "Checks if the value of the field is `_78FULL`"]
    #[inline(always)]
    pub fn is_78full(&self) -> bool {
        *self == TXIFLSEL_A::_78FULL
    }
}
#[doc = "Write proxy for field `TXIFLSEL`"]
pub struct TXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIFLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIFLSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Transmit FIFO becomes less than or equal to 1/8 full"]
    #[inline(always)]
    pub fn _18full(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::_18FULL)
    }
    #[doc = "Transmit FIFO becomes less than or equal to 1/4 full"]
    #[inline(always)]
    pub fn _14full(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::_14FULL)
    }
    #[doc = "Transmit FIFO becomes less than or equal to 1/2 full"]
    #[inline(always)]
    pub fn _12full(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::_12FULL)
    }
    #[doc = "Transmit FIFO becomes less than or equal to 3/4 full"]
    #[inline(always)]
    pub fn _34full(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::_34FULL)
    }
    #[doc = "Transmit FIFO becomes less than or equal to 7/8 full"]
    #[inline(always)]
    pub fn _78full(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::_78FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Possible values of the field `RXIFLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIFLSEL_A {
    #[doc = "Receive FIFO becomes greater than or equal to 1/8 full"]
    _18FULL,
    #[doc = "Receive FIFO becomes greater than or equal to 1/4 full"]
    _14FULL,
    #[doc = "Receive FIFO becomes greater than or equal to 1/2 full"]
    _12FULL,
    #[doc = "Receive FIFO becomes greater than or equal to 3/4 full"]
    _34FULL,
    #[doc = "Receive FIFO becomes greater than or equal to 7/8 full"]
    _78FULL,
}
impl From<RXIFLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXIFLSEL_A) -> Self {
        match variant {
            RXIFLSEL_A::_18FULL => 0,
            RXIFLSEL_A::_14FULL => 1,
            RXIFLSEL_A::_12FULL => 2,
            RXIFLSEL_A::_34FULL => 3,
            RXIFLSEL_A::_78FULL => 4,
        }
    }
}
#[doc = "Reader of field `RXIFLSEL`"]
pub type RXIFLSEL_R = crate::R<u8, RXIFLSEL_A>;
impl RXIFLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXIFLSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXIFLSEL_A::_18FULL),
            1 => Val(RXIFLSEL_A::_14FULL),
            2 => Val(RXIFLSEL_A::_12FULL),
            3 => Val(RXIFLSEL_A::_34FULL),
            4 => Val(RXIFLSEL_A::_78FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_18FULL`"]
    #[inline(always)]
    pub fn is_18full(&self) -> bool {
        *self == RXIFLSEL_A::_18FULL
    }
    #[doc = "Checks if the value of the field is `_14FULL`"]
    #[inline(always)]
    pub fn is_14full(&self) -> bool {
        *self == RXIFLSEL_A::_14FULL
    }
    #[doc = "Checks if the value of the field is `_12FULL`"]
    #[inline(always)]
    pub fn is_12full(&self) -> bool {
        *self == RXIFLSEL_A::_12FULL
    }
    #[doc = "Checks if the value of the field is `_34FULL`"]
    #[inline(always)]
    pub fn is_34full(&self) -> bool {
        *self == RXIFLSEL_A::_34FULL
    }
    #[doc = "Checks if the value of the field is `_78FULL`"]
    #[inline(always)]
    pub fn is_78full(&self) -> bool {
        *self == RXIFLSEL_A::_78FULL
    }
}
#[doc = "Write proxy for field `RXIFLSEL`"]
pub struct RXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIFLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIFLSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Receive FIFO becomes greater than or equal to 1/8 full"]
    #[inline(always)]
    pub fn _18full(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::_18FULL)
    }
    #[doc = "Receive FIFO becomes greater than or equal to 1/4 full"]
    #[inline(always)]
    pub fn _14full(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::_14FULL)
    }
    #[doc = "Receive FIFO becomes greater than or equal to 1/2 full"]
    #[inline(always)]
    pub fn _12full(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::_12FULL)
    }
    #[doc = "Receive FIFO becomes greater than or equal to 3/4 full"]
    #[inline(always)]
    pub fn _34full(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::_34FULL)
    }
    #[doc = "Receive FIFO becomes greater than or equal to 7/8 full"]
    #[inline(always)]
    pub fn _78full(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::_78FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select"]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W {
        TXIFLSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W {
        RXIFLSEL_W { w: self }
    }
}
