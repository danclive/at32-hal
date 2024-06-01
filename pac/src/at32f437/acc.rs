#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: Sts,
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    c1: C1,
    c2: C2,
    c3: C3,
}
impl RegisterBlock {
    #[doc = "0x00 - status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x04 - control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x08 - control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x0c - compare value 1"]
    #[inline(always)]
    pub const fn c1(&self) -> &C1 {
        &self.c1
    }
    #[doc = "0x10 - compare value 2"]
    #[inline(always)]
    pub const fn c2(&self) -> &C2 {
        &self.c2
    }
    #[doc = "0x14 - compare value 3"]
    #[inline(always)]
    pub const fn c3(&self) -> &C3 {
        &self.c3
    }
}
#[doc = "STS (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "status register"]
pub mod sts;
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
#[doc = "C1 (rw) register accessor: compare value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`]
module"]
pub type C1 = crate::Reg<c1::C1Spec>;
#[doc = "compare value 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: compare value 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`]
module"]
pub type C2 = crate::Reg<c2::C2Spec>;
#[doc = "compare value 2"]
pub mod c2;
#[doc = "C3 (rw) register accessor: compare value 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3`]
module"]
pub type C3 = crate::Reg<c3::C3Spec>;
#[doc = "compare value 3"]
pub mod c3;
