#[doc = "Reader of register RNR"]
pub type R = crate::R<u32, super::RNR>;
#[doc = "Writer for register RNR"]
pub type W = crate::W<u32, super::RNR>;
#[doc = "Register RNR `reset()`'s with value 0"]
impl crate::ResetValue for super::RNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `REGION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION_A {
    #[doc = "Select SAU Region 0"]
    SAU_REGION_0,
    #[doc = "Select SAU Region 1"]
    SAU_REGION_1,
    #[doc = "Select SAU Region 2"]
    SAU_REGION_2,
    #[doc = "Select SAU Region 3"]
    SAU_REGION_3,
}
impl From<REGION_A> for u8 {
    #[inline(always)]
    fn from(variant: REGION_A) -> Self {
        match variant {
            REGION_A::SAU_REGION_0 => 0,
            REGION_A::SAU_REGION_1 => 1,
            REGION_A::SAU_REGION_2 => 2,
            REGION_A::SAU_REGION_3 => 3,
        }
    }
}
#[doc = "Reader of field `REGION`"]
pub type REGION_R = crate::R<u8, REGION_A>;
impl REGION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REGION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REGION_A::SAU_REGION_0),
            1 => Val(REGION_A::SAU_REGION_1),
            2 => Val(REGION_A::SAU_REGION_2),
            3 => Val(REGION_A::SAU_REGION_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAU_REGION_0`"]
    #[inline(always)]
    pub fn is_sau_region_0(&self) -> bool {
        *self == REGION_A::SAU_REGION_0
    }
    #[doc = "Checks if the value of the field is `SAU_REGION_1`"]
    #[inline(always)]
    pub fn is_sau_region_1(&self) -> bool {
        *self == REGION_A::SAU_REGION_1
    }
    #[doc = "Checks if the value of the field is `SAU_REGION_2`"]
    #[inline(always)]
    pub fn is_sau_region_2(&self) -> bool {
        *self == REGION_A::SAU_REGION_2
    }
    #[doc = "Checks if the value of the field is `SAU_REGION_3`"]
    #[inline(always)]
    pub fn is_sau_region_3(&self) -> bool {
        *self == REGION_A::SAU_REGION_3
    }
}
#[doc = "Write proxy for field `REGION`"]
pub struct REGION_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select SAU Region 0"]
    #[inline(always)]
    pub fn sau_region_0(self) -> &'a mut W {
        self.variant(REGION_A::SAU_REGION_0)
    }
    #[doc = "Select SAU Region 1"]
    #[inline(always)]
    pub fn sau_region_1(self) -> &'a mut W {
        self.variant(REGION_A::SAU_REGION_1)
    }
    #[doc = "Select SAU Region 2"]
    #[inline(always)]
    pub fn sau_region_2(self) -> &'a mut W {
        self.variant(REGION_A::SAU_REGION_2)
    }
    #[doc = "Select SAU Region 3"]
    #[inline(always)]
    pub fn sau_region_3(self) -> &'a mut W {
        self.variant(REGION_A::SAU_REGION_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Currently selected SAU region"]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Currently selected SAU region"]
    #[inline(always)]
    pub fn region(&mut self) -> REGION_W {
        REGION_W { w: self }
    }
}
