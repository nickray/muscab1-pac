#[doc = "Reader of register WDOGCONTROL"]
pub type R = crate::R<u32, super::WDOGCONTROL>;
#[doc = "Writer for register WDOGCONTROL"]
pub type W = crate::W<u32, super::WDOGCONTROL>;
#[doc = "Register WDOGCONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::WDOGCONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "Disable Watchdog interrupt"]
    DISABLE,
    #[doc = "Enable Watchdog interrupt."]
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
    #[doc = "Disable Watchdog interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTEN_A::DISABLE)
    }
    #[doc = "Enable Watchdog interrupt."]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `RESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEN_A {
    #[doc = "Disable Watchdog reset"]
    DISABLE,
    #[doc = "Enable Watchdog reset"]
    ENABLE,
}
impl From<RESEN_A> for bool {
    #[inline(always)]
    fn from(variant: RESEN_A) -> Self {
        match variant {
            RESEN_A::DISABLE => false,
            RESEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `RESEN`"]
pub type RESEN_R = crate::R<bool, RESEN_A>;
impl RESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEN_A {
        match self.bits {
            false => RESEN_A::DISABLE,
            true => RESEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RESEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RESEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `RESEN`"]
pub struct RESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Watchdog reset"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESEN_A::DISABLE)
    }
    #[doc = "Enable Watchdog reset"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RESEN_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Enable the interrupt event"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable watchdog reset output"]
    #[inline(always)]
    pub fn resen(&self) -> RESEN_R {
        RESEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the interrupt event"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable watchdog reset output"]
    #[inline(always)]
    pub fn resen(&mut self) -> RESEN_W {
        RESEN_W { w: self }
    }
}
