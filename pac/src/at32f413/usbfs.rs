#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ept0: Ept0,
    ept1: Ept1,
    ept2: Ept2,
    ept3: Ept3,
    ept4: Ept4,
    ept5: Ept5,
    ept6: Ept6,
    ept7: Ept7,
    _reserved8: [u8; 0x20],
    ctrl: Ctrl,
    intsts: Intsts,
    sofrnum: Sofrnum,
    devaddr: Devaddr,
    buftbl: Buftbl,
    _reserved13: [u8; 0x0c],
    cfg: Cfg,
}
impl RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    #[inline(always)]
    pub const fn ept0(&self) -> &Ept0 {
        &self.ept0
    }
    #[doc = "0x04 - endpoint 1 register"]
    #[inline(always)]
    pub const fn ept1(&self) -> &Ept1 {
        &self.ept1
    }
    #[doc = "0x08 - endpoint 2 register"]
    #[inline(always)]
    pub const fn ept2(&self) -> &Ept2 {
        &self.ept2
    }
    #[doc = "0x0c - endpoint 3 register"]
    #[inline(always)]
    pub const fn ept3(&self) -> &Ept3 {
        &self.ept3
    }
    #[doc = "0x10 - endpoint 4 register"]
    #[inline(always)]
    pub const fn ept4(&self) -> &Ept4 {
        &self.ept4
    }
    #[doc = "0x14 - endpoint 5 register"]
    #[inline(always)]
    pub const fn ept5(&self) -> &Ept5 {
        &self.ept5
    }
    #[doc = "0x18 - endpoint 6 register"]
    #[inline(always)]
    pub const fn ept6(&self) -> &Ept6 {
        &self.ept6
    }
    #[doc = "0x1c - endpoint 7 register"]
    #[inline(always)]
    pub const fn ept7(&self) -> &Ept7 {
        &self.ept7
    }
    #[doc = "0x40 - control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x44 - interrupt status register"]
    #[inline(always)]
    pub const fn intsts(&self) -> &Intsts {
        &self.intsts
    }
    #[doc = "0x48 - frame number register"]
    #[inline(always)]
    pub const fn sofrnum(&self) -> &Sofrnum {
        &self.sofrnum
    }
    #[doc = "0x4c - device address"]
    #[inline(always)]
    pub const fn devaddr(&self) -> &Devaddr {
        &self.devaddr
    }
    #[doc = "0x50 - Buffer table address"]
    #[inline(always)]
    pub const fn buftbl(&self) -> &Buftbl {
        &self.buftbl
    }
    #[doc = "0x60 - CFG control register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
}
#[doc = "EPT0 (rw) register accessor: endpoint 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept0`]
module"]
#[doc(alias = "EPT0")]
pub type Ept0 = crate::Reg<ept0::Ept0Spec>;
#[doc = "endpoint 0 register"]
pub mod ept0;
#[doc = "EPT1 (rw) register accessor: endpoint 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept1`]
module"]
#[doc(alias = "EPT1")]
pub type Ept1 = crate::Reg<ept1::Ept1Spec>;
#[doc = "endpoint 1 register"]
pub mod ept1;
#[doc = "EPT2 (rw) register accessor: endpoint 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept2`]
module"]
#[doc(alias = "EPT2")]
pub type Ept2 = crate::Reg<ept2::Ept2Spec>;
#[doc = "endpoint 2 register"]
pub mod ept2;
#[doc = "EPT3 (rw) register accessor: endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept3`]
module"]
#[doc(alias = "EPT3")]
pub type Ept3 = crate::Reg<ept3::Ept3Spec>;
#[doc = "endpoint 3 register"]
pub mod ept3;
#[doc = "EPT4 (rw) register accessor: endpoint 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept4`]
module"]
#[doc(alias = "EPT4")]
pub type Ept4 = crate::Reg<ept4::Ept4Spec>;
#[doc = "endpoint 4 register"]
pub mod ept4;
#[doc = "EPT5 (rw) register accessor: endpoint 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept5`]
module"]
#[doc(alias = "EPT5")]
pub type Ept5 = crate::Reg<ept5::Ept5Spec>;
#[doc = "endpoint 5 register"]
pub mod ept5;
#[doc = "EPT6 (rw) register accessor: endpoint 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept6`]
module"]
#[doc(alias = "EPT6")]
pub type Ept6 = crate::Reg<ept6::Ept6Spec>;
#[doc = "endpoint 6 register"]
pub mod ept6;
#[doc = "EPT7 (rw) register accessor: endpoint 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept7`]
module"]
#[doc(alias = "EPT7")]
pub type Ept7 = crate::Reg<ept7::Ept7Spec>;
#[doc = "endpoint 7 register"]
pub mod ept7;
#[doc = "CTRL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "control register"]
pub mod ctrl;
#[doc = "INTSTS (rw) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsts`]
module"]
#[doc(alias = "INTSTS")]
pub type Intsts = crate::Reg<intsts::IntstsSpec>;
#[doc = "interrupt status register"]
pub mod intsts;
#[doc = "SOFRNUM (rw) register accessor: frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sofrnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sofrnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sofrnum`]
module"]
#[doc(alias = "SOFRNUM")]
pub type Sofrnum = crate::Reg<sofrnum::SofrnumSpec>;
#[doc = "frame number register"]
pub mod sofrnum;
#[doc = "DEVADDR (rw) register accessor: device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaddr`]
module"]
#[doc(alias = "DEVADDR")]
pub type Devaddr = crate::Reg<devaddr::DevaddrSpec>;
#[doc = "device address"]
pub mod devaddr;
#[doc = "BUFTBL (rw) register accessor: Buffer table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buftbl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buftbl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buftbl`]
module"]
#[doc(alias = "BUFTBL")]
pub type Buftbl = crate::Reg<buftbl::BuftblSpec>;
#[doc = "Buffer table address"]
pub mod buftbl;
#[doc = "CFG (rw) register accessor: CFG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "CFG control register"]
pub mod cfg;
