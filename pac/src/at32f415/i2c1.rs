#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    oaddr1: Oaddr1,
    oaddr2: Oaddr2,
    dt: Dt,
    sts1: Sts1,
    sts2: Sts2,
    clkctrl: Clkctrl,
    tmrise: Tmrise,
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
    #[doc = "0x08 - Own address register 1"]
    #[inline(always)]
    pub const fn oaddr1(&self) -> &Oaddr1 {
        &self.oaddr1
    }
    #[doc = "0x0c - Own address register 2"]
    #[inline(always)]
    pub const fn oaddr2(&self) -> &Oaddr2 {
        &self.oaddr2
    }
    #[doc = "0x10 - Data register"]
    #[inline(always)]
    pub const fn dt(&self) -> &Dt {
        &self.dt
    }
    #[doc = "0x14 - Status register 1"]
    #[inline(always)]
    pub const fn sts1(&self) -> &Sts1 {
        &self.sts1
    }
    #[doc = "0x18 - Status register 2"]
    #[inline(always)]
    pub const fn sts2(&self) -> &Sts2 {
        &self.sts2
    }
    #[doc = "0x1c - Clock control register"]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x20 - TRISE register"]
    #[inline(always)]
    pub const fn tmrise(&self) -> &Tmrise {
        &self.tmrise
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
#[doc = "OADDR1 (rw) register accessor: Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oaddr1`]
module"]
#[doc(alias = "OADDR1")]
pub type Oaddr1 = crate::Reg<oaddr1::Oaddr1Spec>;
#[doc = "Own address register 1"]
pub mod oaddr1;
#[doc = "OADDR2 (rw) register accessor: Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oaddr2`]
module"]
#[doc(alias = "OADDR2")]
pub type Oaddr2 = crate::Reg<oaddr2::Oaddr2Spec>;
#[doc = "Own address register 2"]
pub mod oaddr2;
#[doc = "DT (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
#[doc(alias = "DT")]
pub type Dt = crate::Reg<dt::DtSpec>;
#[doc = "Data register"]
pub mod dt;
#[doc = "STS1 (rw) register accessor: Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts1`]
module"]
#[doc(alias = "STS1")]
pub type Sts1 = crate::Reg<sts1::Sts1Spec>;
#[doc = "Status register 1"]
pub mod sts1;
#[doc = "STS2 (r) register accessor: Status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts2`]
module"]
#[doc(alias = "STS2")]
pub type Sts2 = crate::Reg<sts2::Sts2Spec>;
#[doc = "Status register 2"]
pub mod sts2;
#[doc = "CLKCTRL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Clock control register"]
pub mod clkctrl;
#[doc = "TMRISE (rw) register accessor: TRISE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmrise::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmrise::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmrise`]
module"]
#[doc(alias = "TMRISE")]
pub type Tmrise = crate::Reg<tmrise::TmriseSpec>;
#[doc = "TRISE register"]
pub mod tmrise;
