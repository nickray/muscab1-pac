#[doc = "Reader of register ICHWPARAMS"]
pub type R = crate::R<u32, super::ICHWPARAMS>;
#[doc = "Reader of field `CSIZE`"]
pub type CSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `STATS`"]
pub type STATS_R = crate::R<bool, bool>;
#[doc = "Possible values of the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "The Instruction cache supports\n                          pre-fetch and locking"]
    SUPPORT,
    #[doc = "The Instruction cache does not\n                          support pre-fetch and locking"]
    UNSUPPORT,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        match variant {
            DMA_A::SUPPORT => true,
            DMA_A::UNSUPPORT => false,
        }
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, DMA_A>;
impl DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            true => DMA_A::SUPPORT,
            false => DMA_A::UNSUPPORT,
        }
    }
    #[doc = "Checks if the value of the field is `SUPPORT`"]
    #[inline(always)]
    pub fn is_support(&self) -> bool {
        *self == DMA_A::SUPPORT
    }
    #[doc = "Checks if the value of the field is `UNSUPPORT`"]
    #[inline(always)]
    pub fn is_unsupport(&self) -> bool {
        *self == DMA_A::UNSUPPORT
    }
}
#[doc = "Possible values of the field `INVMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVMAT_A {
    #[doc = "Indicates Invalidate Cache Line\n                          on Write Match is enabled"]
    ENABLED,
}
impl From<INVMAT_A> for bool {
    #[inline(always)]
    fn from(variant: INVMAT_A) -> Self {
        match variant {
            INVMAT_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `INVMAT`"]
pub type INVMAT_R = crate::R<bool, INVMAT_A>;
impl INVMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, INVMAT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(INVMAT_A::ENABLED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INVMAT_A::ENABLED
    }
}
#[doc = "Reader of field `COFFSIZE`"]
pub type COFFSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `COFFSET`"]
pub type COFFSET_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - Cache size: Defines the size of the instruction cache"]
    #[inline(always)]
    pub fn csize(&self) -> CSIZE_R {
        CSIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Presence of Statistic Functionality"]
    #[inline(always)]
    pub fn stats(&self) -> STATS_R {
        STATS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Presence of DMA Engine"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates whether invalidate cache line on write match is enabled"]
    #[inline(always)]
    pub fn invmat(&self) -> INVMAT_R {
        INVMAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Cacheable Block Size"]
    #[inline(always)]
    pub fn coffsize(&self) -> COFFSIZE_R {
        COFFSIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Cacheable Offset Address"]
    #[inline(always)]
    pub fn coffset(&self) -> COFFSET_R {
        COFFSET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
