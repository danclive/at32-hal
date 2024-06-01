#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: Ctrl1,
    _reserved1: [u8; 0x04],
    stctrl: Stctrl,
    iden: Iden,
    ists: Ists,
    swevt: Swevt,
    _reserved_5_cm1: [u8; 0x04],
    _reserved6: [u8; 0x04],
    cctrl: Cctrl,
    cval: Cval,
    div: Div,
    pr: Pr,
    _reserved10: [u8; 0x04],
    c1dt: C1dt,
    c2dt: C2dt,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x08 - Subordinate TMR control register"]
    #[inline(always)]
    pub const fn stctrl(&self) -> &Stctrl {
        &self.stctrl
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
    #[doc = "0x18 - Channel input mode register 1"]
    #[inline(always)]
    pub const fn cm1_input(&self) -> &Cm1Input {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - Channel output mode register"]
    #[inline(always)]
    pub const fn cm1_output(&self) -> &Cm1Output {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x20 - Channel control register"]
    #[inline(always)]
    pub const fn cctrl(&self) -> &Cctrl {
        &self.cctrl
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
    #[doc = "0x34 - Channel 1 data register"]
    #[inline(always)]
    pub const fn c1dt(&self) -> &C1dt {
        &self.c1dt
    }
    #[doc = "0x38 - Channel 2 data register"]
    #[inline(always)]
    pub const fn c2dt(&self) -> &C2dt {
        &self.c2dt
    }
}
#[doc = "CTRL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Control register 1"]
pub mod ctrl1;
#[doc = "STCTRL (rw) register accessor: Subordinate TMR control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stctrl`]
module"]
#[doc(alias = "STCTRL")]
pub type Stctrl = crate::Reg<stctrl::StctrlSpec>;
#[doc = "Subordinate TMR control register"]
pub mod stctrl;
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
#[doc = "CM1_OUTPUT (rw) register accessor: Channel output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm1_output`]
module"]
#[doc(alias = "CM1_OUTPUT")]
pub type Cm1Output = crate::Reg<cm1_output::Cm1OutputSpec>;
#[doc = "Channel output mode register"]
pub mod cm1_output;
#[doc = "CM1_INPUT (rw) register accessor: Channel input mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm1_input`]
module"]
#[doc(alias = "CM1_INPUT")]
pub type Cm1Input = crate::Reg<cm1_input::Cm1InputSpec>;
#[doc = "Channel input mode register 1"]
pub mod cm1_input;
#[doc = "CCTRL (rw) register accessor: Channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cctrl`]
module"]
#[doc(alias = "CCTRL")]
pub type Cctrl = crate::Reg<cctrl::CctrlSpec>;
#[doc = "Channel control register"]
pub mod cctrl;
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
#[doc = "C1DT (rw) register accessor: Channel 1 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1dt`]
module"]
#[doc(alias = "C1DT")]
pub type C1dt = crate::Reg<c1dt::C1dtSpec>;
#[doc = "Channel 1 data register"]
pub mod c1dt;
#[doc = "C2DT (rw) register accessor: Channel 2 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2dt`]
module"]
#[doc(alias = "C2DT")]
pub type C2dt = crate::Reg<c2dt::C2dtSpec>;
#[doc = "Channel 2 data register"]
pub mod c2dt;
