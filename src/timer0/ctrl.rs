#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "Timer is disabled"]
    DISABLE,
    #[doc = "Timer is enabled"]
    ENABLE,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        match variant {
            ENABLE_A::DISABLE => false,
            ENABLE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLE,
            true => ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "Timer is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
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
#[doc = "Possible values of the field `EXTIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIN_A {
    #[doc = "External Input as Enable is disabled"]
    DISABLE,
    #[doc = "External Input as Enable is enabled"]
    ENABLE,
}
impl From<EXTIN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTIN_A) -> Self {
        match variant {
            EXTIN_A::DISABLE => false,
            EXTIN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `EXTIN`"]
pub type EXTIN_R = crate::R<bool, EXTIN_A>;
impl EXTIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIN_A {
        match self.bits {
            false => EXTIN_A::DISABLE,
            true => EXTIN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXTIN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EXTIN_A::ENABLE
    }
}
#[doc = "Write proxy for field `EXTIN`"]
pub struct EXTIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External Input as Enable is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXTIN_A::DISABLE)
    }
    #[doc = "External Input as Enable is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EXTIN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `EXTCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTCLK_A {
    #[doc = "External Clock is disabled"]
    DISABLE,
    #[doc = "External Clock is enabled"]
    ENABLE,
}
impl From<EXTCLK_A> for bool {
    #[inline(always)]
    fn from(variant: EXTCLK_A) -> Self {
        match variant {
            EXTCLK_A::DISABLE => false,
            EXTCLK_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `EXTCLK`"]
pub type EXTCLK_R = crate::R<bool, EXTCLK_A>;
impl EXTCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTCLK_A {
        match self.bits {
            false => EXTCLK_A::DISABLE,
            true => EXTCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXTCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EXTCLK_A::ENABLE
    }
}
#[doc = "Write proxy for field `EXTCLK`"]
pub struct EXTCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External Clock is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXTCLK_A::DISABLE)
    }
    #[doc = "External Clock is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EXTCLK_A::ENABLE)
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
#[doc = "Possible values of the field `INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "Interrupt is disabled"]
    DISABLE,
    #[doc = "Interrupt is enabled"]
    ENABLE,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        match variant {
            INTEN_A::DISABLE => false,
            INTEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `INTEN`"]
pub type INTEN_R = crate::R<bool, INTEN_A>;
impl INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::DISABLE,
            true => INTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `INTEN`"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTEN_A::DISABLE)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTEN_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Input as Enable"]
    #[inline(always)]
    pub fn extin(&self) -> EXTIN_R {
        EXTIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Clock Enable"]
    #[inline(always)]
    pub fn extclk(&self) -> EXTCLK_R {
        EXTCLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - External Input as Enable"]
    #[inline(always)]
    pub fn extin(&mut self) -> EXTIN_W {
        EXTIN_W { w: self }
    }
    #[doc = "Bit 2 - External Clock Enable"]
    #[inline(always)]
    pub fn extclk(&mut self) -> EXTCLK_W {
        EXTCLK_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
}
