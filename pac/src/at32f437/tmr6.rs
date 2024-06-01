#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    _reserved2: [u8; 0x04],
    iden: Iden,
    ists: Ists,
    swevt: Swevt,
    _reserved5: [u8; 0x0c],
    cval: Cval,
    div: Div,
    pr: Pr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x04 - Control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x0c - Interrupt/DMA enable register"]
    #[inline(always)]
    pub const fn iden(&self) -> &Iden {
        &self.iden
    }
    #[doc = "0x10 - Interrupt status register"]
    #[inline(always)]
    pub const fn ists(&self) -> &Ists {
        &self.ists
    }
    #[doc = "0x14 - Software event register"]
    #[inline(always)]
    pub const fn swevt(&self) -> &Swevt {
        &self.swevt
    }
    #[doc = "0x24 - Counter value"]
    #[inline(always)]
    pub const fn cval(&self) -> &Cval {
        &self.cval
    }
    #[doc = "0x28 - Divider value"]
    #[inline(always)]
    pub const fn div(&self) -> &Div {
        &self.div
    }
    #[doc = "0x2c - Period value"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
}
#[doc = "CTRL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Control register 2"]
pub mod ctrl2;
#[doc = "IDEN (rw) register accessor: Interrupt/DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iden::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iden::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iden`]
module"]
#[doc(alias = "IDEN")]
pub type Iden = crate::Reg<iden::IdenSpec>;
#[doc = "Interrupt/DMA enable register"]
pub mod iden;
#[doc = "ISTS (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ists::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ists::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ists`]
module"]
#[doc(alias = "ISTS")]
pub type Ists = crate::Reg<ists::IstsSpec>;
#[doc = "Interrupt status register"]
pub mod ists;
#[doc = "SWEVT (rw) register accessor: Software event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swevt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swevt`]
module"]
#[doc(alias = "SWEVT")]
pub type Swevt = crate::Reg<swevt::SwevtSpec>;
#[doc = "Software event register"]
pub mod swevt;
#[doc = "CVAL (rw) register accessor: Counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cval`]
module"]
#[doc(alias = "CVAL")]
pub type Cval = crate::Reg<cval::CvalSpec>;
#[doc = "Counter value"]
pub mod cval;
#[doc = "DIV (rw) register accessor: Divider value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
#[doc(alias = "DIV")]
pub type Div = crate::Reg<div::DivSpec>;
#[doc = "Divider value"]
pub mod div;
#[doc = "PR (rw) register accessor: Period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "Period value"]
pub mod pr;
