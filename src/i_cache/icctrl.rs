#[doc = "Reader of register ICCTRL"]
pub type R = crate::R<u32, super::ICCTRL>;
#[doc = "Writer for register ICCTRL"]
pub type W = crate::W<u32, super::ICCTRL>;
#[doc = "Register ICCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ICCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CACHEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEEN_A {
    #[doc = "Caching is enabled"]
    ENABLED,
    #[doc = "All accesses bypass the cache"]
    DISABLED,
}
impl From<CACHEEN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEEN_A) -> Self {
        match variant {
            CACHEEN_A::ENABLED => true,
            CACHEEN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CACHEEN`"]
pub type CACHEEN_R = crate::R<bool, CACHEEN_A>;
impl CACHEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEEN_A {
        match self.bits {
            true => CACHEEN_A::ENABLED,
            false => CACHEEN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CACHEEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CACHEEN_A::DISABLED
    }
}
#[doc = "Write proxy for field `CACHEEN`"]
pub struct CACHEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Caching is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CACHEEN_A::ENABLED)
    }
    #[doc = "All accesses bypass the cache"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CACHEEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `FINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINV_AW {
    #[doc = "Triggers the instruction cache to start\n                          invalidating all cache lines"]
    INVALIDATE,
}
impl From<FINV_AW> for bool {
    #[inline(always)]
    fn from(variant: FINV_AW) -> Self {
        match variant {
            FINV_AW::INVALIDATE => true,
        }
    }
}
#[doc = "Write proxy for field `FINV`"]
pub struct FINV_W<'a> {
    w: &'a mut W,
}
impl<'a> FINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINV_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Triggers the instruction cache to start invalidating all cache lines"]
    #[inline(always)]
    pub fn invalidate(self) -> &'a mut W {
        self.variant(FINV_AW::INVALIDATE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `STATEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATEN_A {
    #[doc = "Cache statistic counters are enabled"]
    ENABLED,
    #[doc = "Cache statistic counters are disabled"]
    DISABLED,
}
impl From<STATEN_A> for bool {
    #[inline(always)]
    fn from(variant: STATEN_A) -> Self {
        match variant {
            STATEN_A::ENABLED => true,
            STATEN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `STATEN`"]
pub type STATEN_R = crate::R<bool, STATEN_A>;
impl STATEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATEN_A {
        match self.bits {
            true => STATEN_A::ENABLED,
            false => STATEN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STATEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STATEN_A::DISABLED
    }
}
#[doc = "Write proxy for field `STATEN`"]
pub struct STATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STATEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cache statistic counters are enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STATEN_A::ENABLED)
    }
    #[doc = "Cache statistic counters are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STATEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `STATC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATC_AW {
    #[doc = "Triggers the instruction cache to start\n                          clear all cache statistic counters"]
    CLEAR,
}
impl From<STATC_AW> for bool {
    #[inline(always)]
    fn from(variant: STATC_AW) -> Self {
        match variant {
            STATC_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `STATC`"]
pub struct STATC_W<'a> {
    w: &'a mut W,
}
impl<'a> STATC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Triggers the instruction cache to start clear all cache statistic counters"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STATC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `HALLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALLOC_A {
    #[doc = "All incoming handler code fetches are not\n                          allocated a cache line if a miss occurs"]
    LOW,
    #[doc = "Handler code access is treated like any other\n                          code access arriving at its interface"]
    HIGH,
}
impl From<HALLOC_A> for bool {
    #[inline(always)]
    fn from(variant: HALLOC_A) -> Self {
        match variant {
            HALLOC_A::LOW => false,
            HALLOC_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `HALLOC`"]
pub type HALLOC_R = crate::R<bool, HALLOC_A>;
impl HALLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALLOC_A {
        match self.bits {
            false => HALLOC_A::LOW,
            true => HALLOC_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == HALLOC_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HALLOC_A::HIGH
    }
}
#[doc = "Write proxy for field `HALLOC`"]
pub struct HALLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> HALLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALLOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All incoming handler code fetches are not allocated a cache line if a miss occurs"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(HALLOC_A::LOW)
    }
    #[doc = "Handler code access is treated like any other code access arriving at its interface"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(HALLOC_A::HIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Cache"]
    #[inline(always)]
    pub fn cacheen(&self) -> CACHEEN_R {
        CACHEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Statistic function"]
    #[inline(always)]
    pub fn staten(&self) -> STATEN_R {
        STATEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Handler Allocation"]
    #[inline(always)]
    pub fn halloc(&self) -> HALLOC_R {
        HALLOC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Cache"]
    #[inline(always)]
    pub fn cacheen(&mut self) -> CACHEEN_W {
        CACHEEN_W { w: self }
    }
    #[doc = "Bit 2 - Full Cache Invalidate"]
    #[inline(always)]
    pub fn finv(&mut self) -> FINV_W {
        FINV_W { w: self }
    }
    #[doc = "Bit 3 - Enable Statistic function"]
    #[inline(always)]
    pub fn staten(&mut self) -> STATEN_W {
        STATEN_W { w: self }
    }
    #[doc = "Bit 4 - Clear Statistic values"]
    #[inline(always)]
    pub fn statc(&mut self) -> STATC_W {
        STATC_W { w: self }
    }
    #[doc = "Bit 5 - Enable Handler Allocation"]
    #[inline(always)]
    pub fn halloc(&mut self) -> HALLOC_W {
        HALLOC_W { w: self }
    }
}
