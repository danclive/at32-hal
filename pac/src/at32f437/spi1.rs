#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    sts: Sts,
    dt: Dt,
    cpoly: Cpoly,
    rcrc: Rcrc,
    tcrc: Tcrc,
    i2sctrl: I2sctrl,
    i2sclk: I2sclk,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x08 - status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x0c - data register"]
    #[inline(always)]
    pub const fn dt(&self) -> &Dt {
        &self.dt
    }
    #[doc = "0x10 - CRC polynomial register"]
    #[inline(always)]
    pub const fn cpoly(&self) -> &Cpoly {
        &self.cpoly
    }
    #[doc = "0x14 - Receive CRC register"]
    #[inline(always)]
    pub const fn rcrc(&self) -> &Rcrc {
        &self.rcrc
    }
    #[doc = "0x18 - Transmit CRC register"]
    #[inline(always)]
    pub const fn tcrc(&self) -> &Tcrc {
        &self.tcrc
    }
    #[doc = "0x1c - I2S control register"]
    #[inline(always)]
    pub const fn i2sctrl(&self) -> &I2sctrl {
        &self.i2sctrl
    }
    #[doc = "0x20 - I2S clock register"]
    #[inline(always)]
    pub const fn i2sclk(&self) -> &I2sclk {
        &self.i2sclk
    }
}
#[doc = "CTRL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "STS (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "status register"]
pub mod sts;
#[doc = "DT (rw) register accessor: data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
#[doc(alias = "DT")]
pub type Dt = crate::Reg<dt::DtSpec>;
#[doc = "data register"]
pub mod dt;
#[doc = "CPOLY (rw) register accessor: CRC polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpoly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpoly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpoly`]
module"]
#[doc(alias = "CPOLY")]
pub type Cpoly = crate::Reg<cpoly::CpolySpec>;
#[doc = "CRC polynomial register"]
pub mod cpoly;
#[doc = "RCRC (r) register accessor: Receive CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcrc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcrc`]
module"]
#[doc(alias = "RCRC")]
pub type Rcrc = crate::Reg<rcrc::RcrcSpec>;
#[doc = "Receive CRC register"]
pub mod rcrc;
#[doc = "TCRC (r) register accessor: Transmit CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcrc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcrc`]
module"]
#[doc(alias = "TCRC")]
pub type Tcrc = crate::Reg<tcrc::TcrcSpec>;
#[doc = "Transmit CRC register"]
pub mod tcrc;
#[doc = "I2SCTRL (rw) register accessor: I2S control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctrl`]
module"]
#[doc(alias = "I2SCTRL")]
pub type I2sctrl = crate::Reg<i2sctrl::I2sctrlSpec>;
#[doc = "I2S control register"]
pub mod i2sctrl;
#[doc = "I2SCLK (rw) register accessor: I2S clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sclk`]
module"]
#[doc(alias = "I2SCLK")]
pub type I2sclk = crate::Reg<i2sclk::I2sclkSpec>;
#[doc = "I2S clock register"]
pub mod i2sclk;
