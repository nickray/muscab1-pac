#[doc = "Reader of register ICIRQSTAT"]
pub type R = crate::R<u32, super::ICIRQSTAT>;
#[doc = "Possible values of the field `IC_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC_STATUS_A {
    #[doc = "Indicates that a cache invalidation\n                          process has been completed"]
    COMPLETED,
}
impl From<IC_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: IC_STATUS_A) -> Self {
        match variant {
            IC_STATUS_A::COMPLETED => true,
        }
    }
}
#[doc = "Reader of field `IC_STATUS`"]
pub type IC_STATUS_R = crate::R<bool, IC_STATUS_A>;
impl IC_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, IC_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(IC_STATUS_A::COMPLETED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == IC_STATUS_A::COMPLETED
    }
}
#[doc = "Possible values of the field `CDC_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDC_STATUS_A {
    #[doc = "Indicates that a request to disable\n                          the cache has been completed"]
    COMPLETED,
}
impl From<CDC_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CDC_STATUS_A) -> Self {
        match variant {
            CDC_STATUS_A::COMPLETED => true,
        }
    }
}
#[doc = "Reader of field `CDC_STATUS`"]
pub type CDC_STATUS_R = crate::R<bool, CDC_STATUS_A>;
impl CDC_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CDC_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CDC_STATUS_A::COMPLETED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CDC_STATUS_A::COMPLETED
    }
}
#[doc = "Possible values of the field `CEC_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEC_STATUS_A {
    #[doc = "Indicates that a request to enable\n                          the cache has been completed"]
    COMPLETED,
}
impl From<CEC_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CEC_STATUS_A) -> Self {
        match variant {
            CEC_STATUS_A::COMPLETED => true,
        }
    }
}
#[doc = "Reader of field `CEC_STATUS`"]
pub type CEC_STATUS_R = crate::R<bool, CEC_STATUS_A>;
impl CEC_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CEC_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CEC_STATUS_A::COMPLETED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CEC_STATUS_A::COMPLETED
    }
}
#[doc = "Possible values of the field `CFE_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFE_STATUS_A {
    #[doc = "Indicates that a bus error occurred\n                          while filling a cache line"]
    ERR_OCCURRED,
}
impl From<CFE_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CFE_STATUS_A) -> Self {
        match variant {
            CFE_STATUS_A::ERR_OCCURRED => true,
        }
    }
}
#[doc = "Reader of field `CFE_STATUS`"]
pub type CFE_STATUS_R = crate::R<bool, CFE_STATUS_A>;
impl CFE_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CFE_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CFE_STATUS_A::ERR_OCCURRED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ERR_OCCURRED`"]
    #[inline(always)]
    pub fn is_err_occurred(&self) -> bool {
        *self == CFE_STATUS_A::ERR_OCCURRED
    }
}
#[doc = "Reader of field `SV_STATUS`"]
pub type SV_STATUS_R = crate::R<bool, bool>;
#[doc = "Possible values of the field `SS_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_STATUS_A {
    #[doc = "Indicates that the internal\n                          statistic counters have saturated"]
    SATURATED,
}
impl From<SS_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SS_STATUS_A) -> Self {
        match variant {
            SS_STATUS_A::SATURATED => true,
        }
    }
}
#[doc = "Reader of field `SS_STATUS`"]
pub type SS_STATUS_R = crate::R<bool, SS_STATUS_A>;
impl SS_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SS_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SS_STATUS_A::SATURATED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SATURATED`"]
    #[inline(always)]
    pub fn is_saturated(&self) -> bool {
        *self == SS_STATUS_A::SATURATED
    }
}
impl R {
    #[doc = "Bit 0 - Invalidate Complete IRQ Status"]
    #[inline(always)]
    pub fn ic_status(&self) -> IC_STATUS_R {
        IC_STATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Cache Disable Complete IRQ Status"]
    #[inline(always)]
    pub fn cdc_status(&self) -> CDC_STATUS_R {
        CDC_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cache Enable Complete IRQ Status"]
    #[inline(always)]
    pub fn cec_status(&self) -> CEC_STATUS_R {
        CEC_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Cache Fill Error IRQ Status"]
    #[inline(always)]
    pub fn cfe_status(&self) -> CFE_STATUS_R {
        CFE_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security violation IRQ Status"]
    #[inline(always)]
    pub fn sv_status(&self) -> SV_STATUS_R {
        SV_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Statistics Saturated Status"]
    #[inline(always)]
    pub fn ss_status(&self) -> SS_STATUS_R {
        SS_STATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
