#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: Sts,
    dt: Dt,
    baudr: Baudr,
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    ctrl3: Ctrl3,
    gdiv: Gdiv,
}
impl RegisterBlock {
    #[doc = "0x00 - Status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x04 - Data register"]
    #[inline(always)]
    pub const fn dt(&self) -> &Dt {
        &self.dt
    }
    #[doc = "0x08 - Baud rate register"]
    #[inline(always)]
    pub const fn baudr(&self) -> &Baudr {
        &self.baudr
    }
    #[doc = "0x0c - Control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x10 - Control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x14 - Control register 3"]
    #[inline(always)]
    pub const fn ctrl3(&self) -> &Ctrl3 {
        &self.ctrl3
    }
    #[doc = "0x18 - Guard time and division register"]
    #[inline(always)]
    pub const fn gdiv(&self) -> &Gdiv {
        &self.gdiv
    }
}
#[doc = "STS (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "Status register"]
pub mod sts;
#[doc = "DT (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
#[doc(alias = "DT")]
pub type Dt = crate::Reg<dt::DtSpec>;
#[doc = "Data register"]
pub mod dt;
#[doc = "BAUDR (rw) register accessor: Baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baudr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudr`]
module"]
#[doc(alias = "BAUDR")]
pub type Baudr = crate::Reg<baudr::BaudrSpec>;
#[doc = "Baud rate register"]
pub mod baudr;
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
#[doc = "CTRL3 (rw) register accessor: Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl3`]
module"]
#[doc(alias = "CTRL3")]
pub type Ctrl3 = crate::Reg<ctrl3::Ctrl3Spec>;
#[doc = "Control register 3"]
pub mod ctrl3;
#[doc = "GDIV (rw) register accessor: Guard time and division register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdiv`]
module"]
#[doc(alias = "GDIV")]
pub type Gdiv = crate::Reg<gdiv::GdivSpec>;
#[doc = "Guard time and division register"]
pub mod gdiv;
