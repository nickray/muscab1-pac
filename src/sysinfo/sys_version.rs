#[doc = "Reader of register SYS_VERSION"]
pub type R = crate::R<u32, super::SYS_VERSION>;
#[doc = "Reader of field `PART_NUMBER`"]
pub type PART_NUMBER_R = crate::R<u16, u16>;
#[doc = "Reader of field `DESIGNER_ID`"]
pub type DESIGNER_ID_R = crate::R<u8, u8>;
#[doc = "Reader of field `MINOR_REVISION`"]
pub type MINOR_REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJOR_REVISION`"]
pub type MAJOR_REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `CONFIGURATION`"]
pub type CONFIGURATION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Part Number for the SSE-200"]
    #[inline(always)]
    pub fn part_number(&self) -> PART_NUMBER_R {
        PART_NUMBER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - Arm Product with designer code 0x41"]
    #[inline(always)]
    pub fn designer_id(&self) -> DESIGNER_ID_R {
        DESIGNER_ID_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - Minor Revision"]
    #[inline(always)]
    pub fn minor_revision(&self) -> MINOR_REVISION_R {
        MINOR_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Major Revision"]
    #[inline(always)]
    pub fn major_revision(&self) -> MAJOR_REVISION_R {
        MAJOR_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CONFIGURATION for SSE-200 r2: 0x2"]
    #[inline(always)]
    pub fn configuration(&self) -> CONFIGURATION_R {
        CONFIGURATION_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
