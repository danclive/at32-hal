#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg1: Cfg1,
    cfg2: Cfg2,
    exintc1: Exintc1,
    exintc2: Exintc2,
    exintc3: Exintc3,
    exintc4: Exintc4,
    _reserved6: [u8; 0x14],
    uhdrv: Uhdrv,
}
impl RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x04 - configuration register 2"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0x08 - external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exintc1(&self) -> &Exintc1 {
        &self.exintc1
    }
    #[doc = "0x0c - external interrupt configuration register 2"]
    #[inline(always)]
    pub const fn exintc2(&self) -> &Exintc2 {
        &self.exintc2
    }
    #[doc = "0x10 - external interrupt configuration register 3"]
    #[inline(always)]
    pub const fn exintc3(&self) -> &Exintc3 {
        &self.exintc3
    }
    #[doc = "0x14 - external interrupt configuration register 4"]
    #[inline(always)]
    pub const fn exintc4(&self) -> &Exintc4 {
        &self.exintc4
    }
    #[doc = "0x2c - Ultra high drive register"]
    #[inline(always)]
    pub const fn uhdrv(&self) -> &Uhdrv {
        &self.uhdrv
    }
}
#[doc = "CFG1 (rw) register accessor: configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "configuration register 1"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "configuration register 2"]
pub mod cfg2;
#[doc = "EXINTC1 (rw) register accessor: external interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc1`]
module"]
#[doc(alias = "EXINTC1")]
pub type Exintc1 = crate::Reg<exintc1::Exintc1Spec>;
#[doc = "external interrupt configuration register 1"]
pub mod exintc1;
#[doc = "EXINTC2 (rw) register accessor: external interrupt configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc2`]
module"]
#[doc(alias = "EXINTC2")]
pub type Exintc2 = crate::Reg<exintc2::Exintc2Spec>;
#[doc = "external interrupt configuration register 2"]
pub mod exintc2;
#[doc = "EXINTC3 (rw) register accessor: external interrupt configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc3`]
module"]
#[doc(alias = "EXINTC3")]
pub type Exintc3 = crate::Reg<exintc3::Exintc3Spec>;
#[doc = "external interrupt configuration register 3"]
pub mod exintc3;
#[doc = "EXINTC4 (rw) register accessor: external interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc4`]
module"]
#[doc(alias = "EXINTC4")]
pub type Exintc4 = crate::Reg<exintc4::Exintc4Spec>;
#[doc = "external interrupt configuration register 4"]
pub mod exintc4;
#[doc = "UHDRV (rw) register accessor: Ultra high drive register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhdrv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhdrv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhdrv`]
module"]
#[doc(alias = "UHDRV")]
pub type Uhdrv = crate::Reg<uhdrv::UhdrvSpec>;
#[doc = "Ultra high drive register"]
pub mod uhdrv;
