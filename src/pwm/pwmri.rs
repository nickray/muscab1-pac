#[doc = "Reader of register PWMRI"]
pub type R = crate::R<u32, super::PWMRI>;
#[doc = "Possible values of the field `Enable_BIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_BIT_A {
    #[doc = "Interrupt is Enabled"]
    ENABLED,
}
impl From<ENABLE_BIT_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_BIT_A) -> Self {
        match variant {
            ENABLE_BIT_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `Enable_BIT`"]
pub type ENABLE_BIT_R = crate::R<bool, ENABLE_BIT_A>;
impl ENABLE_BIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ENABLE_BIT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ENABLE_BIT_A::ENABLED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_BIT_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 0 - Check whether the Interrupt is Enabled"]
    #[inline(always)]
    pub fn enable_bit(&self) -> ENABLE_BIT_R {
        ENABLE_BIT_R::new((self.bits & 0x01) != 0)
    }
}
