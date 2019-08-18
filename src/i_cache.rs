#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hardware Parameter Register"]
    pub ichwparams: ICHWPARAMS,
    #[doc = "0x04 - Instruction Cache Control Register"]
    pub icctrl: ICCTRL,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - Interrupt Request Status Register"]
    pub icirqstat: ICIRQSTAT,
    #[doc = "0x104 - Interrupt Status Clear register"]
    pub icirqsclr: ICIRQSCLR,
    #[doc = "0x108 - Interrupt Enable register"]
    pub icirqen: ICIRQEN,
    #[doc = "0x10c - Address where the latest fill error was seen"]
    pub icdbgfillerr: ICDBGFILLERR,
    _reserved6: [u8; 496usize],
    #[doc = "0x300 - Instruction Cache Statistic Hit Count register"]
    pub icsh: ICSH,
    #[doc = "0x304 - Instruction Cache Statistic Miss Count register"]
    pub icsm: ICSM,
    #[doc = "0x308 - Instruction Cache Statistic Uncached Count register"]
    pub icsuc: ICSUC,
    _reserved9: [u8; 3268usize],
    #[doc = "0xfd0 - Product ID Register 4"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - Product ID Register 5"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - Product ID Register 6"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - Product ID Register 7"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - Product ID Register 0"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Product ID Register 1"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Product ID Register 2"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Product ID Register 3"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Component ID Register 0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Component ID Register 1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Component ID Register 2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Component ID Register 3"]
    pub cidr3: CIDR3,
}
#[doc = "Hardware Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ichwparams](ichwparams) module"]
pub type ICHWPARAMS = crate::Reg<u32, _ICHWPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICHWPARAMS;
#[doc = "`read()` method returns [ichwparams::R](ichwparams::R) reader structure"]
impl crate::Readable for ICHWPARAMS {}
#[doc = "Hardware Parameter Register"]
pub mod ichwparams;
#[doc = "Instruction Cache Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icctrl](icctrl) module"]
pub type ICCTRL = crate::Reg<u32, _ICCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICCTRL;
#[doc = "`read()` method returns [icctrl::R](icctrl::R) reader structure"]
impl crate::Readable for ICCTRL {}
#[doc = "`write(|w| ..)` method takes [icctrl::W](icctrl::W) writer structure"]
impl crate::Writable for ICCTRL {}
#[doc = "Instruction Cache Control Register"]
pub mod icctrl;
#[doc = "Interrupt Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icirqstat](icirqstat) module"]
pub type ICIRQSTAT = crate::Reg<u32, _ICIRQSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICIRQSTAT;
#[doc = "`read()` method returns [icirqstat::R](icirqstat::R) reader structure"]
impl crate::Readable for ICIRQSTAT {}
#[doc = "Interrupt Request Status Register"]
pub mod icirqstat;
#[doc = "Interrupt Status Clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icirqsclr](icirqsclr) module"]
pub type ICIRQSCLR = crate::Reg<u32, _ICIRQSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICIRQSCLR;
#[doc = "`write(|w| ..)` method takes [icirqsclr::W](icirqsclr::W) writer structure"]
impl crate::Writable for ICIRQSCLR {}
#[doc = "Interrupt Status Clear register"]
pub mod icirqsclr;
#[doc = "Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icirqen](icirqen) module"]
pub type ICIRQEN = crate::Reg<u32, _ICIRQEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICIRQEN;
#[doc = "`read()` method returns [icirqen::R](icirqen::R) reader structure"]
impl crate::Readable for ICIRQEN {}
#[doc = "`write(|w| ..)` method takes [icirqen::W](icirqen::W) writer structure"]
impl crate::Writable for ICIRQEN {}
#[doc = "Interrupt Enable register"]
pub mod icirqen;
#[doc = "Address where the latest fill error was seen\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icdbgfillerr](icdbgfillerr) module"]
pub type ICDBGFILLERR = crate::Reg<u32, _ICDBGFILLERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICDBGFILLERR;
#[doc = "`read()` method returns [icdbgfillerr::R](icdbgfillerr::R) reader structure"]
impl crate::Readable for ICDBGFILLERR {}
#[doc = "Address where the latest fill error was seen"]
pub mod icdbgfillerr;
#[doc = "Instruction Cache Statistic Hit Count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icsh](icsh) module"]
pub type ICSH = crate::Reg<u32, _ICSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSH;
#[doc = "`read()` method returns [icsh::R](icsh::R) reader structure"]
impl crate::Readable for ICSH {}
#[doc = "Instruction Cache Statistic Hit Count register"]
pub mod icsh;
#[doc = "Instruction Cache Statistic Miss Count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icsm](icsm) module"]
pub type ICSM = crate::Reg<u32, _ICSM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSM;
#[doc = "`read()` method returns [icsm::R](icsm::R) reader structure"]
impl crate::Readable for ICSM {}
#[doc = "Instruction Cache Statistic Miss Count register"]
pub mod icsm;
#[doc = "Instruction Cache Statistic Uncached Count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icsuc](icsuc) module"]
pub type ICSUC = crate::Reg<u32, _ICSUC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSUC;
#[doc = "`read()` method returns [icsuc::R](icsuc::R) reader structure"]
impl crate::Readable for ICSUC {}
#[doc = "Instruction Cache Statistic Uncached Count register"]
pub mod icsuc;
#[doc = "Product ID Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr4](pidr4) module"]
pub type PIDR4 = crate::Reg<u32, _PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR4;
#[doc = "`read()` method returns [pidr4::R](pidr4::R) reader structure"]
impl crate::Readable for PIDR4 {}
#[doc = "Product ID Register 4"]
pub mod pidr4;
#[doc = "Product ID Register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr5](pidr5) module"]
pub type PIDR5 = crate::Reg<u32, _PIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR5;
#[doc = "`read()` method returns [pidr5::R](pidr5::R) reader structure"]
impl crate::Readable for PIDR5 {}
#[doc = "Product ID Register 5"]
pub mod pidr5;
#[doc = "Product ID Register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr6](pidr6) module"]
pub type PIDR6 = crate::Reg<u32, _PIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR6;
#[doc = "`read()` method returns [pidr6::R](pidr6::R) reader structure"]
impl crate::Readable for PIDR6 {}
#[doc = "Product ID Register 6"]
pub mod pidr6;
#[doc = "Product ID Register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr7](pidr7) module"]
pub type PIDR7 = crate::Reg<u32, _PIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR7;
#[doc = "`read()` method returns [pidr7::R](pidr7::R) reader structure"]
impl crate::Readable for PIDR7 {}
#[doc = "Product ID Register 7"]
pub mod pidr7;
#[doc = "Product ID Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr0](pidr0) module"]
pub type PIDR0 = crate::Reg<u32, _PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR0;
#[doc = "`read()` method returns [pidr0::R](pidr0::R) reader structure"]
impl crate::Readable for PIDR0 {}
#[doc = "Product ID Register 0"]
pub mod pidr0;
#[doc = "Product ID Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr1](pidr1) module"]
pub type PIDR1 = crate::Reg<u32, _PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR1;
#[doc = "`read()` method returns [pidr1::R](pidr1::R) reader structure"]
impl crate::Readable for PIDR1 {}
#[doc = "Product ID Register 1"]
pub mod pidr1;
#[doc = "Product ID Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr2](pidr2) module"]
pub type PIDR2 = crate::Reg<u32, _PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR2;
#[doc = "`read()` method returns [pidr2::R](pidr2::R) reader structure"]
impl crate::Readable for PIDR2 {}
#[doc = "Product ID Register 2"]
pub mod pidr2;
#[doc = "Product ID Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr3](pidr3) module"]
pub type PIDR3 = crate::Reg<u32, _PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR3;
#[doc = "`read()` method returns [pidr3::R](pidr3::R) reader structure"]
impl crate::Readable for PIDR3 {}
#[doc = "Product ID Register 3"]
pub mod pidr3;
#[doc = "Component ID Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr0](cidr0) module"]
pub type CIDR0 = crate::Reg<u32, _CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR0;
#[doc = "`read()` method returns [cidr0::R](cidr0::R) reader structure"]
impl crate::Readable for CIDR0 {}
#[doc = "Component ID Register 0"]
pub mod cidr0;
#[doc = "Component ID Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr1](cidr1) module"]
pub type CIDR1 = crate::Reg<u32, _CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR1;
#[doc = "`read()` method returns [cidr1::R](cidr1::R) reader structure"]
impl crate::Readable for CIDR1 {}
#[doc = "Component ID Register 1"]
pub mod cidr1;
#[doc = "Component ID Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr2](cidr2) module"]
pub type CIDR2 = crate::Reg<u32, _CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR2;
#[doc = "`read()` method returns [cidr2::R](cidr2::R) reader structure"]
impl crate::Readable for CIDR2 {}
#[doc = "Component ID Register 2"]
pub mod cidr2;
#[doc = "Component ID Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr3](cidr3) module"]
pub type CIDR3 = crate::Reg<u32, _CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR3;
#[doc = "`read()` method returns [cidr3::R](cidr3::R) reader structure"]
impl crate::Readable for CIDR3 {}
#[doc = "Component ID Register 3"]
pub mod cidr3;
