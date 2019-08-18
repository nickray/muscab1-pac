#[doc = "Reader of register GPTCOUNTER"]
pub type R = crate::R<u32, super::GPTCOUNTER>;
#[doc = "Reader of field `GPTCOUNTER`"]
pub type GPTCOUNTER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current value of 32-bit Timer Counter"]
    #[inline(always)]
    pub fn gptcounter(&self) -> GPTCOUNTER_R {
        GPTCOUNTER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
