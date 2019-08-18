#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QSPI Configuration Register"]
    pub qspicfg: QSPICFG,
    #[doc = "0x04 - Device Read Instruction Register"]
    pub devreadinstr: DEVREADINSTR,
    #[doc = "0x08 - Device Write Instruction Configuration Register"]
    pub devwriteinstr: DEVWRITEINSTR,
    _reserved3: [u8; 8usize],
    #[doc = "0x14 - Device Size Configuration Register"]
    pub devsize: DEVSIZE,
    _reserved4: [u8; 12usize],
    #[doc = "0x24 - Remap Address Register"]
    pub remapaddr: REMAPADDR,
    _reserved5: [u8; 104usize],
    #[doc = "0x90 - Flash Command Control Register"]
    pub flashcmdctrl: FLASHCMDCTRL,
    #[doc = "0x94 - Flash Command Address Register"]
    pub flashcmdaddr: FLASHCMDADDR,
    _reserved7: [u8; 8usize],
    #[doc = "0xa0 - Flash Command Read Data Register (Lower)"]
    pub flashcmdrdatalow: FLASHCMDRDATALOW,
    #[doc = "0xa4 - Flash Command Read Data Register (Upper)"]
    pub flashcmdrdataup: FLASHCMDRDATAUP,
    #[doc = "0xa8 - Flash Command Write Data Register (Lower)"]
    pub flashcmdwrdatalow: FLASHCMDWRDATALOW,
    #[doc = "0xac - Flash Command Write Data Register (Upper)"]
    pub flashcmdwrdataup: FLASHCMDWRDATAUP,
}
#[doc = "QSPI Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qspicfg](qspicfg) module"]
pub type QSPICFG = crate::Reg<u32, _QSPICFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPICFG;
#[doc = "`read()` method returns [qspicfg::R](qspicfg::R) reader structure"]
impl crate::Readable for QSPICFG {}
#[doc = "`write(|w| ..)` method takes [qspicfg::W](qspicfg::W) writer structure"]
impl crate::Writable for QSPICFG {}
#[doc = "QSPI Configuration Register"]
pub mod qspicfg;
#[doc = "Device Read Instruction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devreadinstr](devreadinstr) module"]
pub type DEVREADINSTR = crate::Reg<u32, _DEVREADINSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVREADINSTR;
#[doc = "`read()` method returns [devreadinstr::R](devreadinstr::R) reader structure"]
impl crate::Readable for DEVREADINSTR {}
#[doc = "`write(|w| ..)` method takes [devreadinstr::W](devreadinstr::W) writer structure"]
impl crate::Writable for DEVREADINSTR {}
#[doc = "Device Read Instruction Register"]
pub mod devreadinstr;
#[doc = "Device Write Instruction Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devwriteinstr](devwriteinstr) module"]
pub type DEVWRITEINSTR = crate::Reg<u32, _DEVWRITEINSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVWRITEINSTR;
#[doc = "`read()` method returns [devwriteinstr::R](devwriteinstr::R) reader structure"]
impl crate::Readable for DEVWRITEINSTR {}
#[doc = "`write(|w| ..)` method takes [devwriteinstr::W](devwriteinstr::W) writer structure"]
impl crate::Writable for DEVWRITEINSTR {}
#[doc = "Device Write Instruction Configuration Register"]
pub mod devwriteinstr;
#[doc = "Device Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devsize](devsize) module"]
pub type DEVSIZE = crate::Reg<u32, _DEVSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVSIZE;
#[doc = "`read()` method returns [devsize::R](devsize::R) reader structure"]
impl crate::Readable for DEVSIZE {}
#[doc = "`write(|w| ..)` method takes [devsize::W](devsize::W) writer structure"]
impl crate::Writable for DEVSIZE {}
#[doc = "Device Size Configuration Register"]
pub mod devsize;
#[doc = "Remap Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [remapaddr](remapaddr) module"]
pub type REMAPADDR = crate::Reg<u32, _REMAPADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REMAPADDR;
#[doc = "`read()` method returns [remapaddr::R](remapaddr::R) reader structure"]
impl crate::Readable for REMAPADDR {}
#[doc = "`write(|w| ..)` method takes [remapaddr::W](remapaddr::W) writer structure"]
impl crate::Writable for REMAPADDR {}
#[doc = "Remap Address Register"]
pub mod remapaddr;
#[doc = "Flash Command Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flashcmdctrl](flashcmdctrl) module"]
pub type FLASHCMDCTRL = crate::Reg<u32, _FLASHCMDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCMDCTRL;
#[doc = "`read()` method returns [flashcmdctrl::R](flashcmdctrl::R) reader structure"]
impl crate::Readable for FLASHCMDCTRL {}
#[doc = "`write(|w| ..)` method takes [flashcmdctrl::W](flashcmdctrl::W) writer structure"]
impl crate::Writable for FLASHCMDCTRL {}
#[doc = "Flash Command Control Register"]
pub mod flashcmdctrl;
#[doc = "Flash Command Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flashcmdaddr](flashcmdaddr) module"]
pub type FLASHCMDADDR = crate::Reg<u32, _FLASHCMDADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCMDADDR;
#[doc = "`read()` method returns [flashcmdaddr::R](flashcmdaddr::R) reader structure"]
impl crate::Readable for FLASHCMDADDR {}
#[doc = "`write(|w| ..)` method takes [flashcmdaddr::W](flashcmdaddr::W) writer structure"]
impl crate::Writable for FLASHCMDADDR {}
#[doc = "Flash Command Address Register"]
pub mod flashcmdaddr;
#[doc = "Flash Command Read Data Register (Lower)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flashcmdrdatalow](flashcmdrdatalow) module"]
pub type FLASHCMDRDATALOW = crate::Reg<u32, _FLASHCMDRDATALOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCMDRDATALOW;
#[doc = "`read()` method returns [flashcmdrdatalow::R](flashcmdrdatalow::R) reader structure"]
impl crate::Readable for FLASHCMDRDATALOW {}
#[doc = "Flash Command Read Data Register (Lower)"]
pub mod flashcmdrdatalow;
#[doc = "Flash Command Read Data Register (Upper)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flashcmdrdataup](flashcmdrdataup) module"]
pub type FLASHCMDRDATAUP = crate::Reg<u32, _FLASHCMDRDATAUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCMDRDATAUP;
#[doc = "`read()` method returns [flashcmdrdataup::R](flashcmdrdataup::R) reader structure"]
impl crate::Readable for FLASHCMDRDATAUP {}
#[doc = "Flash Command Read Data Register (Upper)"]
pub mod flashcmdrdataup;
#[doc = "Flash Command Write Data Register (Lower)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flashcmdwrdatalow](flashcmdwrdatalow) module"]
pub type FLASHCMDWRDATALOW = crate::Reg<u32, _FLASHCMDWRDATALOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCMDWRDATALOW;
#[doc = "`read()` method returns [flashcmdwrdatalow::R](flashcmdwrdatalow::R) reader structure"]
impl crate::Readable for FLASHCMDWRDATALOW {}
#[doc = "`write(|w| ..)` method takes [flashcmdwrdatalow::W](flashcmdwrdatalow::W) writer structure"]
impl crate::Writable for FLASHCMDWRDATALOW {}
#[doc = "Flash Command Write Data Register (Lower)"]
pub mod flashcmdwrdatalow;
#[doc = "Flash Command Write Data Register (Upper)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flashcmdwrdataup](flashcmdwrdataup) module"]
pub type FLASHCMDWRDATAUP = crate::Reg<u32, _FLASHCMDWRDATAUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCMDWRDATAUP;
#[doc = "`read()` method returns [flashcmdwrdataup::R](flashcmdwrdataup::R) reader structure"]
impl crate::Readable for FLASHCMDWRDATAUP {}
#[doc = "`write(|w| ..)` method takes [flashcmdwrdataup::W](flashcmdwrdataup::W) writer structure"]
impl crate::Writable for FLASHCMDWRDATAUP {}
#[doc = "Flash Command Write Data Register (Upper)"]
pub mod flashcmdwrdataup;
