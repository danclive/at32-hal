#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hcfg: Hcfg,
    hfir: Hfir,
    hfnum: Hfnum,
    _reserved3: [u8; 0x04],
    hptxsts: Hptxsts,
    haint: Haint,
    haintmsk: Haintmsk,
    _reserved6: [u8; 0x24],
    hprt: Hprt,
    _reserved7: [u8; 0xbc],
    hcchar0: Hcchar0,
    _reserved8: [u8; 0x04],
    hcint0: Hcint0,
    hcintmsk0: Hcintmsk0,
    hctsiz0: Hctsiz0,
    _reserved11: [u8; 0x0c],
    hcchar1: Hcchar1,
    _reserved12: [u8; 0x04],
    hcint1: Hcint1,
    hcintmsk1: Hcintmsk1,
    hctsiz1: Hctsiz1,
    _reserved15: [u8; 0x0c],
    hcchar2: Hcchar2,
    _reserved16: [u8; 0x04],
    hcint2: Hcint2,
    hcintmsk2: Hcintmsk2,
    hctsiz2: Hctsiz2,
    _reserved19: [u8; 0x0c],
    hcchar3: Hcchar3,
    _reserved20: [u8; 0x04],
    hcint3: Hcint3,
    hcintmsk3: Hcintmsk3,
    hctsiz3: Hctsiz3,
    _reserved23: [u8; 0x0c],
    hcchar4: Hcchar4,
    _reserved24: [u8; 0x04],
    hcint4: Hcint4,
    hcintmsk4: Hcintmsk4,
    hctsiz4: Hctsiz4,
    _reserved27: [u8; 0x0c],
    hcchar5: Hcchar5,
    _reserved28: [u8; 0x04],
    hcint5: Hcint5,
    hcintmsk5: Hcintmsk5,
    hctsiz5: Hctsiz5,
    _reserved31: [u8; 0x0c],
    hcchar6: Hcchar6,
    _reserved32: [u8; 0x04],
    hcint6: Hcint6,
    hcintmsk6: Hcintmsk6,
    hctsiz6: Hctsiz6,
    _reserved35: [u8; 0x0c],
    hcchar7: Hcchar7,
    _reserved36: [u8; 0x04],
    hcint7: Hcint7,
    hcintmsk7: Hcintmsk7,
    hctsiz7: Hctsiz7,
}
impl RegisterBlock {
    #[doc = "0x00 - OTGFS host configuration register (OTGFS_HCFG)"]
    #[inline(always)]
    pub const fn hcfg(&self) -> &Hcfg {
        &self.hcfg
    }
    #[doc = "0x04 - OTGFS Host frame interval register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &Hfir {
        &self.hfir
    }
    #[doc = "0x08 - OTGFS host frame number/frame time remaining register (OTGFS_HFNUM)"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &Hfnum {
        &self.hfnum
    }
    #[doc = "0x10 - OTGFS_Host periodic transmit FIFO/queue status register (OTGFS_HPTXSTS)"]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &Hptxsts {
        &self.hptxsts
    }
    #[doc = "0x14 - OTGFS Host all channels interrupt register"]
    #[inline(always)]
    pub const fn haint(&self) -> &Haint {
        &self.haint
    }
    #[doc = "0x18 - OTGFS host all channels interrupt mask register"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &Haintmsk {
        &self.haintmsk
    }
    #[doc = "0x40 - OTGFS host port control and status register (OTGFS_HPRT)"]
    #[inline(always)]
    pub const fn hprt(&self) -> &Hprt {
        &self.hprt
    }
    #[doc = "0x100 - OTGFS host channel-0 characteristics register (OTGFS_HCCHAR0)"]
    #[inline(always)]
    pub const fn hcchar0(&self) -> &Hcchar0 {
        &self.hcchar0
    }
    #[doc = "0x108 - OTGFS host channel-0 interrupt register (OTGFS_HCINT0)"]
    #[inline(always)]
    pub const fn hcint0(&self) -> &Hcint0 {
        &self.hcint0
    }
    #[doc = "0x10c - OTGFS host channel-0 mask register (OTGFS_HCINTMSK0)"]
    #[inline(always)]
    pub const fn hcintmsk0(&self) -> &Hcintmsk0 {
        &self.hcintmsk0
    }
    #[doc = "0x110 - OTGFS host channel-0 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz0(&self) -> &Hctsiz0 {
        &self.hctsiz0
    }
    #[doc = "0x120 - OTGFS host channel-1 characteristics register (OTGFS_HCCHAR1)"]
    #[inline(always)]
    pub const fn hcchar1(&self) -> &Hcchar1 {
        &self.hcchar1
    }
    #[doc = "0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
    #[inline(always)]
    pub const fn hcint1(&self) -> &Hcint1 {
        &self.hcint1
    }
    #[doc = "0x12c - OTGFS host channel-1 mask register (OTGFS_HCINTMSK1)"]
    #[inline(always)]
    pub const fn hcintmsk1(&self) -> &Hcintmsk1 {
        &self.hcintmsk1
    }
    #[doc = "0x130 - OTGFS host channel-1 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz1(&self) -> &Hctsiz1 {
        &self.hctsiz1
    }
    #[doc = "0x140 - OTGFS host channel-2 characteristics register (OTGFS_HCCHAR2)"]
    #[inline(always)]
    pub const fn hcchar2(&self) -> &Hcchar2 {
        &self.hcchar2
    }
    #[doc = "0x148 - OTGFS host channel-2 interrupt register (OTGFS_HCINT2)"]
    #[inline(always)]
    pub const fn hcint2(&self) -> &Hcint2 {
        &self.hcint2
    }
    #[doc = "0x14c - OTGFS host channel-2 mask register (OTGFS_HCINTMSK2)"]
    #[inline(always)]
    pub const fn hcintmsk2(&self) -> &Hcintmsk2 {
        &self.hcintmsk2
    }
    #[doc = "0x150 - OTGFS host channel-2 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz2(&self) -> &Hctsiz2 {
        &self.hctsiz2
    }
    #[doc = "0x160 - OTGFS host channel-3 characteristics register (OTGFS_HCCHAR3)"]
    #[inline(always)]
    pub const fn hcchar3(&self) -> &Hcchar3 {
        &self.hcchar3
    }
    #[doc = "0x168 - OTGFS host channel-3 interrupt register (OTGFS_HCINT3)"]
    #[inline(always)]
    pub const fn hcint3(&self) -> &Hcint3 {
        &self.hcint3
    }
    #[doc = "0x16c - OTGFS host channel-3 mask register (OTGFS_HCINTMSK3)"]
    #[inline(always)]
    pub const fn hcintmsk3(&self) -> &Hcintmsk3 {
        &self.hcintmsk3
    }
    #[doc = "0x170 - OTGFS host channel-3 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz3(&self) -> &Hctsiz3 {
        &self.hctsiz3
    }
    #[doc = "0x180 - OTGFS host channel-4 characteristics register (OTGFS_HCCHAR4)"]
    #[inline(always)]
    pub const fn hcchar4(&self) -> &Hcchar4 {
        &self.hcchar4
    }
    #[doc = "0x188 - OTGFS host channel-4 interrupt register (OTGFS_HCINT4)"]
    #[inline(always)]
    pub const fn hcint4(&self) -> &Hcint4 {
        &self.hcint4
    }
    #[doc = "0x18c - OTGFS host channel-4 mask register (OTGFS_HCINTMSK4)"]
    #[inline(always)]
    pub const fn hcintmsk4(&self) -> &Hcintmsk4 {
        &self.hcintmsk4
    }
    #[doc = "0x190 - OTGFS host channel-4 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz4(&self) -> &Hctsiz4 {
        &self.hctsiz4
    }
    #[doc = "0x1a0 - OTGFS host channel-5 characteristics register (OTGFS_HCCHAR5)"]
    #[inline(always)]
    pub const fn hcchar5(&self) -> &Hcchar5 {
        &self.hcchar5
    }
    #[doc = "0x1a8 - OTGFS host channel-5 interrupt register (OTGFS_HCINT5)"]
    #[inline(always)]
    pub const fn hcint5(&self) -> &Hcint5 {
        &self.hcint5
    }
    #[doc = "0x1ac - OTGFS host channel-5 mask register (OTGFS_HCINTMSK5)"]
    #[inline(always)]
    pub const fn hcintmsk5(&self) -> &Hcintmsk5 {
        &self.hcintmsk5
    }
    #[doc = "0x1b0 - OTGFS host channel-5 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz5(&self) -> &Hctsiz5 {
        &self.hctsiz5
    }
    #[doc = "0x1c0 - OTGFS host channel-6 characteristics register (OTGFS_HCCHAR6)"]
    #[inline(always)]
    pub const fn hcchar6(&self) -> &Hcchar6 {
        &self.hcchar6
    }
    #[doc = "0x1c8 - OTGFS host channel-6 interrupt register (OTGFS_HCINT6)"]
    #[inline(always)]
    pub const fn hcint6(&self) -> &Hcint6 {
        &self.hcint6
    }
    #[doc = "0x1cc - OTGFS host channel-6 mask register (OTGFS_HCINTMSK6)"]
    #[inline(always)]
    pub const fn hcintmsk6(&self) -> &Hcintmsk6 {
        &self.hcintmsk6
    }
    #[doc = "0x1d0 - OTGFS host channel-6 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz6(&self) -> &Hctsiz6 {
        &self.hctsiz6
    }
    #[doc = "0x1e0 - OTGFS host channel-7 characteristics register (OTGFS_HCCHAR7)"]
    #[inline(always)]
    pub const fn hcchar7(&self) -> &Hcchar7 {
        &self.hcchar7
    }
    #[doc = "0x1e8 - OTGFS host channel-7 interrupt register (OTGFS_HCINT7)"]
    #[inline(always)]
    pub const fn hcint7(&self) -> &Hcint7 {
        &self.hcint7
    }
    #[doc = "0x1ec - OTGFS host channel-7 mask register (OTGFS_HCINTMSK7)"]
    #[inline(always)]
    pub const fn hcintmsk7(&self) -> &Hcintmsk7 {
        &self.hcintmsk7
    }
    #[doc = "0x1f0 - OTGFS host channel-7 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz7(&self) -> &Hctsiz7 {
        &self.hctsiz7
    }
}
#[doc = "HCFG (rw) register accessor: OTGFS host configuration register (OTGFS_HCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`]
module"]
#[doc(alias = "HCFG")]
pub type Hcfg = crate::Reg<hcfg::HcfgSpec>;
#[doc = "OTGFS host configuration register (OTGFS_HCFG)"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: OTGFS Host frame interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`]
module"]
#[doc(alias = "HFIR")]
pub type Hfir = crate::Reg<hfir::HfirSpec>;
#[doc = "OTGFS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: OTGFS host frame number/frame time remaining register (OTGFS_HFNUM)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`]
module"]
#[doc(alias = "HFNUM")]
pub type Hfnum = crate::Reg<hfnum::HfnumSpec>;
#[doc = "OTGFS host frame number/frame time remaining register (OTGFS_HFNUM)"]
pub mod hfnum;
#[doc = "HPTXSTS (rw) register accessor: OTGFS_Host periodic transmit FIFO/queue status register (OTGFS_HPTXSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`]
module"]
#[doc(alias = "HPTXSTS")]
pub type Hptxsts = crate::Reg<hptxsts::HptxstsSpec>;
#[doc = "OTGFS_Host periodic transmit FIFO/queue status register (OTGFS_HPTXSTS)"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: OTGFS Host all channels interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`]
module"]
#[doc(alias = "HAINT")]
pub type Haint = crate::Reg<haint::HaintSpec>;
#[doc = "OTGFS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: OTGFS host all channels interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`]
module"]
#[doc(alias = "HAINTMSK")]
pub type Haintmsk = crate::Reg<haintmsk::HaintmskSpec>;
#[doc = "OTGFS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: OTGFS host port control and status register (OTGFS_HPRT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`]
module"]
#[doc(alias = "HPRT")]
pub type Hprt = crate::Reg<hprt::HprtSpec>;
#[doc = "OTGFS host port control and status register (OTGFS_HPRT)"]
pub mod hprt;
#[doc = "HCCHAR0 (rw) register accessor: OTGFS host channel-0 characteristics register (OTGFS_HCCHAR0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar0`]
module"]
#[doc(alias = "HCCHAR0")]
pub type Hcchar0 = crate::Reg<hcchar0::Hcchar0Spec>;
#[doc = "OTGFS host channel-0 characteristics register (OTGFS_HCCHAR0)"]
pub mod hcchar0;
#[doc = "HCCHAR1 (rw) register accessor: OTGFS host channel-1 characteristics register (OTGFS_HCCHAR1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar1`]
module"]
#[doc(alias = "HCCHAR1")]
pub type Hcchar1 = crate::Reg<hcchar1::Hcchar1Spec>;
#[doc = "OTGFS host channel-1 characteristics register (OTGFS_HCCHAR1)"]
pub mod hcchar1;
#[doc = "HCCHAR2 (rw) register accessor: OTGFS host channel-2 characteristics register (OTGFS_HCCHAR2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar2`]
module"]
#[doc(alias = "HCCHAR2")]
pub type Hcchar2 = crate::Reg<hcchar2::Hcchar2Spec>;
#[doc = "OTGFS host channel-2 characteristics register (OTGFS_HCCHAR2)"]
pub mod hcchar2;
#[doc = "HCCHAR3 (rw) register accessor: OTGFS host channel-3 characteristics register (OTGFS_HCCHAR3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar3`]
module"]
#[doc(alias = "HCCHAR3")]
pub type Hcchar3 = crate::Reg<hcchar3::Hcchar3Spec>;
#[doc = "OTGFS host channel-3 characteristics register (OTGFS_HCCHAR3)"]
pub mod hcchar3;
#[doc = "HCCHAR4 (rw) register accessor: OTGFS host channel-4 characteristics register (OTGFS_HCCHAR4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar4`]
module"]
#[doc(alias = "HCCHAR4")]
pub type Hcchar4 = crate::Reg<hcchar4::Hcchar4Spec>;
#[doc = "OTGFS host channel-4 characteristics register (OTGFS_HCCHAR4)"]
pub mod hcchar4;
#[doc = "HCCHAR5 (rw) register accessor: OTGFS host channel-5 characteristics register (OTGFS_HCCHAR5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar5`]
module"]
#[doc(alias = "HCCHAR5")]
pub type Hcchar5 = crate::Reg<hcchar5::Hcchar5Spec>;
#[doc = "OTGFS host channel-5 characteristics register (OTGFS_HCCHAR5)"]
pub mod hcchar5;
#[doc = "HCCHAR6 (rw) register accessor: OTGFS host channel-6 characteristics register (OTGFS_HCCHAR6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar6`]
module"]
#[doc(alias = "HCCHAR6")]
pub type Hcchar6 = crate::Reg<hcchar6::Hcchar6Spec>;
#[doc = "OTGFS host channel-6 characteristics register (OTGFS_HCCHAR6)"]
pub mod hcchar6;
#[doc = "HCCHAR7 (rw) register accessor: OTGFS host channel-7 characteristics register (OTGFS_HCCHAR7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar7`]
module"]
#[doc(alias = "HCCHAR7")]
pub type Hcchar7 = crate::Reg<hcchar7::Hcchar7Spec>;
#[doc = "OTGFS host channel-7 characteristics register (OTGFS_HCCHAR7)"]
pub mod hcchar7;
#[doc = "HCINT0 (rw) register accessor: OTGFS host channel-0 interrupt register (OTGFS_HCINT0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint0`]
module"]
#[doc(alias = "HCINT0")]
pub type Hcint0 = crate::Reg<hcint0::Hcint0Spec>;
#[doc = "OTGFS host channel-0 interrupt register (OTGFS_HCINT0)"]
pub mod hcint0;
#[doc = "HCINT1 (rw) register accessor: OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint1`]
module"]
#[doc(alias = "HCINT1")]
pub type Hcint1 = crate::Reg<hcint1::Hcint1Spec>;
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
pub mod hcint1;
#[doc = "HCINT2 (rw) register accessor: OTGFS host channel-2 interrupt register (OTGFS_HCINT2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint2`]
module"]
#[doc(alias = "HCINT2")]
pub type Hcint2 = crate::Reg<hcint2::Hcint2Spec>;
#[doc = "OTGFS host channel-2 interrupt register (OTGFS_HCINT2)"]
pub mod hcint2;
#[doc = "HCINT3 (rw) register accessor: OTGFS host channel-3 interrupt register (OTGFS_HCINT3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint3`]
module"]
#[doc(alias = "HCINT3")]
pub type Hcint3 = crate::Reg<hcint3::Hcint3Spec>;
#[doc = "OTGFS host channel-3 interrupt register (OTGFS_HCINT3)"]
pub mod hcint3;
#[doc = "HCINT4 (rw) register accessor: OTGFS host channel-4 interrupt register (OTGFS_HCINT4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint4`]
module"]
#[doc(alias = "HCINT4")]
pub type Hcint4 = crate::Reg<hcint4::Hcint4Spec>;
#[doc = "OTGFS host channel-4 interrupt register (OTGFS_HCINT4)"]
pub mod hcint4;
#[doc = "HCINT5 (rw) register accessor: OTGFS host channel-5 interrupt register (OTGFS_HCINT5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint5`]
module"]
#[doc(alias = "HCINT5")]
pub type Hcint5 = crate::Reg<hcint5::Hcint5Spec>;
#[doc = "OTGFS host channel-5 interrupt register (OTGFS_HCINT5)"]
pub mod hcint5;
#[doc = "HCINT6 (rw) register accessor: OTGFS host channel-6 interrupt register (OTGFS_HCINT6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint6`]
module"]
#[doc(alias = "HCINT6")]
pub type Hcint6 = crate::Reg<hcint6::Hcint6Spec>;
#[doc = "OTGFS host channel-6 interrupt register (OTGFS_HCINT6)"]
pub mod hcint6;
#[doc = "HCINT7 (rw) register accessor: OTGFS host channel-7 interrupt register (OTGFS_HCINT7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint7`]
module"]
#[doc(alias = "HCINT7")]
pub type Hcint7 = crate::Reg<hcint7::Hcint7Spec>;
#[doc = "OTGFS host channel-7 interrupt register (OTGFS_HCINT7)"]
pub mod hcint7;
#[doc = "HCINTMSK0 (rw) register accessor: OTGFS host channel-0 mask register (OTGFS_HCINTMSK0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk0`]
module"]
#[doc(alias = "HCINTMSK0")]
pub type Hcintmsk0 = crate::Reg<hcintmsk0::Hcintmsk0Spec>;
#[doc = "OTGFS host channel-0 mask register (OTGFS_HCINTMSK0)"]
pub mod hcintmsk0;
#[doc = "HCINTMSK1 (rw) register accessor: OTGFS host channel-1 mask register (OTGFS_HCINTMSK1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk1`]
module"]
#[doc(alias = "HCINTMSK1")]
pub type Hcintmsk1 = crate::Reg<hcintmsk1::Hcintmsk1Spec>;
#[doc = "OTGFS host channel-1 mask register (OTGFS_HCINTMSK1)"]
pub mod hcintmsk1;
#[doc = "HCINTMSK2 (rw) register accessor: OTGFS host channel-2 mask register (OTGFS_HCINTMSK2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk2`]
module"]
#[doc(alias = "HCINTMSK2")]
pub type Hcintmsk2 = crate::Reg<hcintmsk2::Hcintmsk2Spec>;
#[doc = "OTGFS host channel-2 mask register (OTGFS_HCINTMSK2)"]
pub mod hcintmsk2;
#[doc = "HCINTMSK3 (rw) register accessor: OTGFS host channel-3 mask register (OTGFS_HCINTMSK3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk3`]
module"]
#[doc(alias = "HCINTMSK3")]
pub type Hcintmsk3 = crate::Reg<hcintmsk3::Hcintmsk3Spec>;
#[doc = "OTGFS host channel-3 mask register (OTGFS_HCINTMSK3)"]
pub mod hcintmsk3;
#[doc = "HCINTMSK4 (rw) register accessor: OTGFS host channel-4 mask register (OTGFS_HCINTMSK4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk4`]
module"]
#[doc(alias = "HCINTMSK4")]
pub type Hcintmsk4 = crate::Reg<hcintmsk4::Hcintmsk4Spec>;
#[doc = "OTGFS host channel-4 mask register (OTGFS_HCINTMSK4)"]
pub mod hcintmsk4;
#[doc = "HCINTMSK5 (rw) register accessor: OTGFS host channel-5 mask register (OTGFS_HCINTMSK5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk5`]
module"]
#[doc(alias = "HCINTMSK5")]
pub type Hcintmsk5 = crate::Reg<hcintmsk5::Hcintmsk5Spec>;
#[doc = "OTGFS host channel-5 mask register (OTGFS_HCINTMSK5)"]
pub mod hcintmsk5;
#[doc = "HCINTMSK6 (rw) register accessor: OTGFS host channel-6 mask register (OTGFS_HCINTMSK6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk6`]
module"]
#[doc(alias = "HCINTMSK6")]
pub type Hcintmsk6 = crate::Reg<hcintmsk6::Hcintmsk6Spec>;
#[doc = "OTGFS host channel-6 mask register (OTGFS_HCINTMSK6)"]
pub mod hcintmsk6;
#[doc = "HCINTMSK7 (rw) register accessor: OTGFS host channel-7 mask register (OTGFS_HCINTMSK7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk7`]
module"]
#[doc(alias = "HCINTMSK7")]
pub type Hcintmsk7 = crate::Reg<hcintmsk7::Hcintmsk7Spec>;
#[doc = "OTGFS host channel-7 mask register (OTGFS_HCINTMSK7)"]
pub mod hcintmsk7;
#[doc = "HCTSIZ0 (rw) register accessor: OTGFS host channel-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz0`]
module"]
#[doc(alias = "HCTSIZ0")]
pub type Hctsiz0 = crate::Reg<hctsiz0::Hctsiz0Spec>;
#[doc = "OTGFS host channel-0 transfer size register"]
pub mod hctsiz0;
#[doc = "HCTSIZ1 (rw) register accessor: OTGFS host channel-1 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz1`]
module"]
#[doc(alias = "HCTSIZ1")]
pub type Hctsiz1 = crate::Reg<hctsiz1::Hctsiz1Spec>;
#[doc = "OTGFS host channel-1 transfer size register"]
pub mod hctsiz1;
#[doc = "HCTSIZ2 (rw) register accessor: OTGFS host channel-2 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz2`]
module"]
#[doc(alias = "HCTSIZ2")]
pub type Hctsiz2 = crate::Reg<hctsiz2::Hctsiz2Spec>;
#[doc = "OTGFS host channel-2 transfer size register"]
pub mod hctsiz2;
#[doc = "HCTSIZ3 (rw) register accessor: OTGFS host channel-3 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz3`]
module"]
#[doc(alias = "HCTSIZ3")]
pub type Hctsiz3 = crate::Reg<hctsiz3::Hctsiz3Spec>;
#[doc = "OTGFS host channel-3 transfer size register"]
pub mod hctsiz3;
#[doc = "HCTSIZ4 (rw) register accessor: OTGFS host channel-4 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz4`]
module"]
#[doc(alias = "HCTSIZ4")]
pub type Hctsiz4 = crate::Reg<hctsiz4::Hctsiz4Spec>;
#[doc = "OTGFS host channel-4 transfer size register"]
pub mod hctsiz4;
#[doc = "HCTSIZ5 (rw) register accessor: OTGFS host channel-5 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz5`]
module"]
#[doc(alias = "HCTSIZ5")]
pub type Hctsiz5 = crate::Reg<hctsiz5::Hctsiz5Spec>;
#[doc = "OTGFS host channel-5 transfer size register"]
pub mod hctsiz5;
#[doc = "HCTSIZ6 (rw) register accessor: OTGFS host channel-6 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz6`]
module"]
#[doc(alias = "HCTSIZ6")]
pub type Hctsiz6 = crate::Reg<hctsiz6::Hctsiz6Spec>;
#[doc = "OTGFS host channel-6 transfer size register"]
pub mod hctsiz6;
#[doc = "HCTSIZ7 (rw) register accessor: OTGFS host channel-7 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz7`]
module"]
#[doc(alias = "HCTSIZ7")]
pub type Hctsiz7 = crate::Reg<hctsiz7::Hctsiz7Spec>;
#[doc = "OTGFS host channel-7 transfer size register"]
pub mod hctsiz7;
