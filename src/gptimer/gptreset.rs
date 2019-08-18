#[doc = "Reader of register GPTRESET"]
pub type R = crate::R<u32, super::GPTRESET>;
#[doc = "Reader of field `GPTRESET`"]
pub type GPTRESET_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - CPU0 interrupt status"]
    #[inline(always)]
    pub fn gptreset(&self) -> GPTRESET_R {
        GPTRESET_R::new((self.bits & 0x03) as u8)
    }
}
