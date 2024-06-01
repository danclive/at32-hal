#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    oaddr1: Oaddr1,
    oaddr2: Oaddr2,
    clkctrl: Clkctrl,
    timeout: Timeout,
    sts: Sts,
    clr: Clr,
    pec: Pec,
    rxdt: Rxdt,
    txdt: Txdt,
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
    #[doc = "0x10 - Clock contorl register"]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x14 - Timeout register"]
    #[inline(always)]
    pub const fn timeout(&self) -> &Timeout {
        &self.timeout
    }
    #[doc = "0x18 - Interrupt and Status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x1c - Interrupt clear register"]
    #[inline(always)]
    pub const fn clr(&self) -> &Clr {
        &self.clr
    }
    #[doc = "0x20 - PEC register"]
    #[inline(always)]
    pub const fn pec(&self) -> &Pec {
        &self.pec
    }
    #[doc = "0x24 - Receive data register"]
    #[inline(always)]
    pub const fn rxdt(&self) -> &Rxdt {
        &self.rxdt
    }
    #[doc = "0x28 - Transmit data register"]
    #[inline(always)]
    pub const fn txdt(&self) -> &Txdt {
        &self.txdt
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
#[doc = "CLKCTRL (rw) register accessor: Clock contorl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Clock contorl register"]
pub mod clkctrl;
#[doc = "TIMEOUT (rw) register accessor: Timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout`]
module"]
#[doc(alias = "TIMEOUT")]
pub type Timeout = crate::Reg<timeout::TimeoutSpec>;
#[doc = "Timeout register"]
pub mod timeout;
#[doc = "STS (rw) register accessor: Interrupt and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "Interrupt and Status register"]
pub mod sts;
#[doc = "CLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "Interrupt clear register"]
pub mod clr;
#[doc = "PEC (r) register accessor: PEC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pec`]
module"]
#[doc(alias = "PEC")]
pub type Pec = crate::Reg<pec::PecSpec>;
#[doc = "PEC register"]
pub mod pec;
#[doc = "RXDT (r) register accessor: Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdt`]
module"]
#[doc(alias = "RXDT")]
pub type Rxdt = crate::Reg<rxdt::RxdtSpec>;
#[doc = "Receive data register"]
pub mod rxdt;
#[doc = "TXDT (rw) register accessor: Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdt`]
module"]
#[doc(alias = "TXDT")]
pub type Txdt = crate::Reg<txdt::TxdtSpec>;
#[doc = "Transmit data register"]
pub mod txdt;
