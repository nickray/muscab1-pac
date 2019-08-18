#[doc = "Reader of register CHIP_ID"]
pub type R = crate::R<u32, super::CHIP_ID>;
#[doc = "Reader of field `chip_id`"]
pub type CHIP_ID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Component ID information"]
    #[inline(always)]
    pub fn chip_id(&self) -> CHIP_ID_R {
        CHIP_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
