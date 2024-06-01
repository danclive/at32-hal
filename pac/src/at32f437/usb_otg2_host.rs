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
    _reserved39: [u8; 0x0c],
    hcchar8: Hcchar8,
    _reserved40: [u8; 0x04],
    hcint8: Hcint8,
    hcintmsk8: Hcintmsk8,
    hctsiz8: Hctsiz8,
    _reserved43: [u8; 0x0c],
    hcchar9: Hcchar9,
    _reserved44: [u8; 0x04],
    hcint9: Hcint9,
    hcintmsk9: Hcintmsk9,
    hctsiz9: Hctsiz9,
    _reserved47: [u8; 0x0c],
    hcchar10: Hcchar10,
    _reserved48: [u8; 0x04],
    hcint10: Hcint10,
    hcintmsk10: Hcintmsk10,
    hctsiz10: Hctsiz10,
    _reserved51: [u8; 0x0c],
    hcchar11: Hcchar11,
    _reserved52: [u8; 0x04],
    hcint11: Hcint11,
    hcintmsk11: Hcintmsk11,
    hctsiz11: Hctsiz11,
    _reserved55: [u8; 0x0c],
    hcchar12: Hcchar12,
    _reserved56: [u8; 0x04],
    hcint12: Hcint12,
    hcintmsk12: Hcintmsk12,
    hctsiz12: Hctsiz12,
    _reserved59: [u8; 0x0c],
    hcchar13: Hcchar13,
    _reserved60: [u8; 0x04],
    hcint13: Hcint13,
    hcintmsk13: Hcintmsk13,
    hctsiz13: Hctsiz13,
    _reserved63: [u8; 0x0c],
    hcchar14: Hcchar14,
    _reserved64: [u8; 0x04],
    hcint14: Hcint14,
    hcintmsk14: Hcintmsk14,
    hctsiz14: Hctsiz14,
    _reserved67: [u8; 0x0c],
    hcchar15: Hcchar15,
    _reserved68: [u8; 0x04],
    hcint15: Hcint15,
    hcintmsk15: Hcintmsk15,
    hctsiz15: Hctsiz15,
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
    #[doc = "0x200 - OTGFS host channel-8 characteristics register (OTGFS_HCCHAR8)"]
    #[inline(always)]
    pub const fn hcchar8(&self) -> &Hcchar8 {
        &self.hcchar8
    }
    #[doc = "0x208 - OTGFS host channel-8 interrupt register (OTGFS_HCINT8)"]
    #[inline(always)]
    pub const fn hcint8(&self) -> &Hcint8 {
        &self.hcint8
    }
    #[doc = "0x20c - OTGFS host channel-8 mask register (OTGFS_HCINTMSK8)"]
    #[inline(always)]
    pub const fn hcintmsk8(&self) -> &Hcintmsk8 {
        &self.hcintmsk8
    }
    #[doc = "0x210 - OTGFS host channel-8 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz8(&self) -> &Hctsiz8 {
        &self.hctsiz8
    }
    #[doc = "0x220 - OTGFS host channel-9 characteristics register (OTGFS_HCCHAR9)"]
    #[inline(always)]
    pub const fn hcchar9(&self) -> &Hcchar9 {
        &self.hcchar9
    }
    #[doc = "0x228 - OTGFS host channel-9 interrupt register (OTGFS_HCINT9)"]
    #[inline(always)]
    pub const fn hcint9(&self) -> &Hcint9 {
        &self.hcint9
    }
    #[doc = "0x22c - OTGFS host channel-9 mask register (OTGFS_HCINTMSK9)"]
    #[inline(always)]
    pub const fn hcintmsk9(&self) -> &Hcintmsk9 {
        &self.hcintmsk9
    }
    #[doc = "0x230 - OTGFS host channel-9 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz9(&self) -> &Hctsiz9 {
        &self.hctsiz9
    }
    #[doc = "0x240 - OTGFS host channel-10 characteristics register (OTGFS_HCCHAR10)"]
    #[inline(always)]
    pub const fn hcchar10(&self) -> &Hcchar10 {
        &self.hcchar10
    }
    #[doc = "0x248 - OTGFS host channel-10 interrupt register (OTGFS_HCINT10)"]
    #[inline(always)]
    pub const fn hcint10(&self) -> &Hcint10 {
        &self.hcint10
    }
    #[doc = "0x24c - OTGFS host channel-10 mask register (OTGFS_HCINTMSK10)"]
    #[inline(always)]
    pub const fn hcintmsk10(&self) -> &Hcintmsk10 {
        &self.hcintmsk10
    }
    #[doc = "0x250 - OTGFS host channel-10 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz10(&self) -> &Hctsiz10 {
        &self.hctsiz10
    }
    #[doc = "0x260 - OTGFS host channel-7 characteristics register (OTGFS_HCCHAR11)"]
    #[inline(always)]
    pub const fn hcchar11(&self) -> &Hcchar11 {
        &self.hcchar11
    }
    #[doc = "0x268 - OTGFS host channel-11 interrupt register (OTGFS_HCINT11)"]
    #[inline(always)]
    pub const fn hcint11(&self) -> &Hcint11 {
        &self.hcint11
    }
    #[doc = "0x26c - OTGFS host channel-11 mask register (OTGFS_HCINTMSK11)"]
    #[inline(always)]
    pub const fn hcintmsk11(&self) -> &Hcintmsk11 {
        &self.hcintmsk11
    }
    #[doc = "0x270 - OTGFS host channel-11 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz11(&self) -> &Hctsiz11 {
        &self.hctsiz11
    }
    #[doc = "0x280 - OTGFS host channel-12 characteristics register (OTGFS_HCCHAR12)"]
    #[inline(always)]
    pub const fn hcchar12(&self) -> &Hcchar12 {
        &self.hcchar12
    }
    #[doc = "0x288 - OTGFS host channel-12 interrupt register (OTGFS_HCINT12)"]
    #[inline(always)]
    pub const fn hcint12(&self) -> &Hcint12 {
        &self.hcint12
    }
    #[doc = "0x28c - OTGFS host channel-12 mask register (OTGFS_HCINTMSK12)"]
    #[inline(always)]
    pub const fn hcintmsk12(&self) -> &Hcintmsk12 {
        &self.hcintmsk12
    }
    #[doc = "0x290 - OTGFS host channel-12 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz12(&self) -> &Hctsiz12 {
        &self.hctsiz12
    }
    #[doc = "0x2a0 - OTGFS host channel-13 characteristics register (OTGFS_HCCHAR13)"]
    #[inline(always)]
    pub const fn hcchar13(&self) -> &Hcchar13 {
        &self.hcchar13
    }
    #[doc = "0x2a8 - OTGFS host channel-13 interrupt register (OTGFS_HCINT13)"]
    #[inline(always)]
    pub const fn hcint13(&self) -> &Hcint13 {
        &self.hcint13
    }
    #[doc = "0x2ac - OTGFS host channel-13 mask register (OTGFS_HCINTMSK13)"]
    #[inline(always)]
    pub const fn hcintmsk13(&self) -> &Hcintmsk13 {
        &self.hcintmsk13
    }
    #[doc = "0x2b0 - OTGFS host channel-13 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz13(&self) -> &Hctsiz13 {
        &self.hctsiz13
    }
    #[doc = "0x2c0 - OTGFS host channel-14 characteristics register (OTGFS_HCCHAR14)"]
    #[inline(always)]
    pub const fn hcchar14(&self) -> &Hcchar14 {
        &self.hcchar14
    }
    #[doc = "0x2c8 - OTGFS host channel-14 interrupt register (OTGFS_HCINT14)"]
    #[inline(always)]
    pub const fn hcint14(&self) -> &Hcint14 {
        &self.hcint14
    }
    #[doc = "0x2cc - OTGFS host channel-14 mask register (OTGFS_HCINTMSK14)"]
    #[inline(always)]
    pub const fn hcintmsk14(&self) -> &Hcintmsk14 {
        &self.hcintmsk14
    }
    #[doc = "0x2d0 - OTGFS host channel-14 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz14(&self) -> &Hctsiz14 {
        &self.hctsiz14
    }
    #[doc = "0x2e0 - OTGFS host channel-15 characteristics register (OTGFS_HCCHAR15)"]
    #[inline(always)]
    pub const fn hcchar15(&self) -> &Hcchar15 {
        &self.hcchar15
    }
    #[doc = "0x2e8 - OTGFS host channel-15 interrupt register (OTGFS_HCINT15)"]
    #[inline(always)]
    pub const fn hcint15(&self) -> &Hcint15 {
        &self.hcint15
    }
    #[doc = "0x2ec - OTGFS host channel-15 mask register (OTGFS_HCINTMSK15)"]
    #[inline(always)]
    pub const fn hcintmsk15(&self) -> &Hcintmsk15 {
        &self.hcintmsk15
    }
    #[doc = "0x2f0 - OTGFS host channel-15 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz15(&self) -> &Hctsiz15 {
        &self.hctsiz15
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
#[doc = "HCCHAR8 (rw) register accessor: OTGFS host channel-8 characteristics register (OTGFS_HCCHAR8)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar8`]
module"]
#[doc(alias = "HCCHAR8")]
pub type Hcchar8 = crate::Reg<hcchar8::Hcchar8Spec>;
#[doc = "OTGFS host channel-8 characteristics register (OTGFS_HCCHAR8)"]
pub mod hcchar8;
#[doc = "HCCHAR9 (rw) register accessor: OTGFS host channel-9 characteristics register (OTGFS_HCCHAR9)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar9`]
module"]
#[doc(alias = "HCCHAR9")]
pub type Hcchar9 = crate::Reg<hcchar9::Hcchar9Spec>;
#[doc = "OTGFS host channel-9 characteristics register (OTGFS_HCCHAR9)"]
pub mod hcchar9;
#[doc = "HCCHAR10 (rw) register accessor: OTGFS host channel-10 characteristics register (OTGFS_HCCHAR10)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar10`]
module"]
#[doc(alias = "HCCHAR10")]
pub type Hcchar10 = crate::Reg<hcchar10::Hcchar10Spec>;
#[doc = "OTGFS host channel-10 characteristics register (OTGFS_HCCHAR10)"]
pub mod hcchar10;
#[doc = "HCCHAR11 (rw) register accessor: OTGFS host channel-7 characteristics register (OTGFS_HCCHAR11)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar11`]
module"]
#[doc(alias = "HCCHAR11")]
pub type Hcchar11 = crate::Reg<hcchar11::Hcchar11Spec>;
#[doc = "OTGFS host channel-7 characteristics register (OTGFS_HCCHAR11)"]
pub mod hcchar11;
#[doc = "HCCHAR12 (rw) register accessor: OTGFS host channel-12 characteristics register (OTGFS_HCCHAR12)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar12`]
module"]
#[doc(alias = "HCCHAR12")]
pub type Hcchar12 = crate::Reg<hcchar12::Hcchar12Spec>;
#[doc = "OTGFS host channel-12 characteristics register (OTGFS_HCCHAR12)"]
pub mod hcchar12;
#[doc = "HCCHAR13 (rw) register accessor: OTGFS host channel-13 characteristics register (OTGFS_HCCHAR13)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar13`]
module"]
#[doc(alias = "HCCHAR13")]
pub type Hcchar13 = crate::Reg<hcchar13::Hcchar13Spec>;
#[doc = "OTGFS host channel-13 characteristics register (OTGFS_HCCHAR13)"]
pub mod hcchar13;
#[doc = "HCCHAR14 (rw) register accessor: OTGFS host channel-14 characteristics register (OTGFS_HCCHAR14)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar14`]
module"]
#[doc(alias = "HCCHAR14")]
pub type Hcchar14 = crate::Reg<hcchar14::Hcchar14Spec>;
#[doc = "OTGFS host channel-14 characteristics register (OTGFS_HCCHAR14)"]
pub mod hcchar14;
#[doc = "HCCHAR15 (rw) register accessor: OTGFS host channel-15 characteristics register (OTGFS_HCCHAR15)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar15`]
module"]
#[doc(alias = "HCCHAR15")]
pub type Hcchar15 = crate::Reg<hcchar15::Hcchar15Spec>;
#[doc = "OTGFS host channel-15 characteristics register (OTGFS_HCCHAR15)"]
pub mod hcchar15;
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
#[doc = "HCINT8 (rw) register accessor: OTGFS host channel-8 interrupt register (OTGFS_HCINT8)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint8`]
module"]
#[doc(alias = "HCINT8")]
pub type Hcint8 = crate::Reg<hcint8::Hcint8Spec>;
#[doc = "OTGFS host channel-8 interrupt register (OTGFS_HCINT8)"]
pub mod hcint8;
#[doc = "HCINT9 (rw) register accessor: OTGFS host channel-9 interrupt register (OTGFS_HCINT9)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint9`]
module"]
#[doc(alias = "HCINT9")]
pub type Hcint9 = crate::Reg<hcint9::Hcint9Spec>;
#[doc = "OTGFS host channel-9 interrupt register (OTGFS_HCINT9)"]
pub mod hcint9;
#[doc = "HCINT10 (rw) register accessor: OTGFS host channel-10 interrupt register (OTGFS_HCINT10)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint10`]
module"]
#[doc(alias = "HCINT10")]
pub type Hcint10 = crate::Reg<hcint10::Hcint10Spec>;
#[doc = "OTGFS host channel-10 interrupt register (OTGFS_HCINT10)"]
pub mod hcint10;
#[doc = "HCINT11 (rw) register accessor: OTGFS host channel-11 interrupt register (OTGFS_HCINT11)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint11`]
module"]
#[doc(alias = "HCINT11")]
pub type Hcint11 = crate::Reg<hcint11::Hcint11Spec>;
#[doc = "OTGFS host channel-11 interrupt register (OTGFS_HCINT11)"]
pub mod hcint11;
#[doc = "HCINT12 (rw) register accessor: OTGFS host channel-12 interrupt register (OTGFS_HCINT12)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint12`]
module"]
#[doc(alias = "HCINT12")]
pub type Hcint12 = crate::Reg<hcint12::Hcint12Spec>;
#[doc = "OTGFS host channel-12 interrupt register (OTGFS_HCINT12)"]
pub mod hcint12;
#[doc = "HCINT13 (rw) register accessor: OTGFS host channel-13 interrupt register (OTGFS_HCINT13)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint13`]
module"]
#[doc(alias = "HCINT13")]
pub type Hcint13 = crate::Reg<hcint13::Hcint13Spec>;
#[doc = "OTGFS host channel-13 interrupt register (OTGFS_HCINT13)"]
pub mod hcint13;
#[doc = "HCINT14 (rw) register accessor: OTGFS host channel-14 interrupt register (OTGFS_HCINT14)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint14`]
module"]
#[doc(alias = "HCINT14")]
pub type Hcint14 = crate::Reg<hcint14::Hcint14Spec>;
#[doc = "OTGFS host channel-14 interrupt register (OTGFS_HCINT14)"]
pub mod hcint14;
#[doc = "HCINT15 (rw) register accessor: OTGFS host channel-15 interrupt register (OTGFS_HCINT15)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint15`]
module"]
#[doc(alias = "HCINT15")]
pub type Hcint15 = crate::Reg<hcint15::Hcint15Spec>;
#[doc = "OTGFS host channel-15 interrupt register (OTGFS_HCINT15)"]
pub mod hcint15;
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
#[doc = "HCINTMSK8 (rw) register accessor: OTGFS host channel-8 mask register (OTGFS_HCINTMSK8)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk8`]
module"]
#[doc(alias = "HCINTMSK8")]
pub type Hcintmsk8 = crate::Reg<hcintmsk8::Hcintmsk8Spec>;
#[doc = "OTGFS host channel-8 mask register (OTGFS_HCINTMSK8)"]
pub mod hcintmsk8;
#[doc = "HCINTMSK9 (rw) register accessor: OTGFS host channel-9 mask register (OTGFS_HCINTMSK9)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk9`]
module"]
#[doc(alias = "HCINTMSK9")]
pub type Hcintmsk9 = crate::Reg<hcintmsk9::Hcintmsk9Spec>;
#[doc = "OTGFS host channel-9 mask register (OTGFS_HCINTMSK9)"]
pub mod hcintmsk9;
#[doc = "HCINTMSK10 (rw) register accessor: OTGFS host channel-10 mask register (OTGFS_HCINTMSK10)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk10`]
module"]
#[doc(alias = "HCINTMSK10")]
pub type Hcintmsk10 = crate::Reg<hcintmsk10::Hcintmsk10Spec>;
#[doc = "OTGFS host channel-10 mask register (OTGFS_HCINTMSK10)"]
pub mod hcintmsk10;
#[doc = "HCINTMSK11 (rw) register accessor: OTGFS host channel-11 mask register (OTGFS_HCINTMSK11)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk11`]
module"]
#[doc(alias = "HCINTMSK11")]
pub type Hcintmsk11 = crate::Reg<hcintmsk11::Hcintmsk11Spec>;
#[doc = "OTGFS host channel-11 mask register (OTGFS_HCINTMSK11)"]
pub mod hcintmsk11;
#[doc = "HCINTMSK12 (rw) register accessor: OTGFS host channel-12 mask register (OTGFS_HCINTMSK12)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk12`]
module"]
#[doc(alias = "HCINTMSK12")]
pub type Hcintmsk12 = crate::Reg<hcintmsk12::Hcintmsk12Spec>;
#[doc = "OTGFS host channel-12 mask register (OTGFS_HCINTMSK12)"]
pub mod hcintmsk12;
#[doc = "HCINTMSK13 (rw) register accessor: OTGFS host channel-13 mask register (OTGFS_HCINTMSK13)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk13`]
module"]
#[doc(alias = "HCINTMSK13")]
pub type Hcintmsk13 = crate::Reg<hcintmsk13::Hcintmsk13Spec>;
#[doc = "OTGFS host channel-13 mask register (OTGFS_HCINTMSK13)"]
pub mod hcintmsk13;
#[doc = "HCINTMSK14 (rw) register accessor: OTGFS host channel-14 mask register (OTGFS_HCINTMSK14)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk14`]
module"]
#[doc(alias = "HCINTMSK14")]
pub type Hcintmsk14 = crate::Reg<hcintmsk14::Hcintmsk14Spec>;
#[doc = "OTGFS host channel-14 mask register (OTGFS_HCINTMSK14)"]
pub mod hcintmsk14;
#[doc = "HCINTMSK15 (rw) register accessor: OTGFS host channel-15 mask register (OTGFS_HCINTMSK15)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk15`]
module"]
#[doc(alias = "HCINTMSK15")]
pub type Hcintmsk15 = crate::Reg<hcintmsk15::Hcintmsk15Spec>;
#[doc = "OTGFS host channel-15 mask register (OTGFS_HCINTMSK15)"]
pub mod hcintmsk15;
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
#[doc = "HCTSIZ8 (rw) register accessor: OTGFS host channel-8 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz8`]
module"]
#[doc(alias = "HCTSIZ8")]
pub type Hctsiz8 = crate::Reg<hctsiz8::Hctsiz8Spec>;
#[doc = "OTGFS host channel-8 transfer size register"]
pub mod hctsiz8;
#[doc = "HCTSIZ9 (rw) register accessor: OTGFS host channel-9 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz9`]
module"]
#[doc(alias = "HCTSIZ9")]
pub type Hctsiz9 = crate::Reg<hctsiz9::Hctsiz9Spec>;
#[doc = "OTGFS host channel-9 transfer size register"]
pub mod hctsiz9;
#[doc = "HCTSIZ10 (rw) register accessor: OTGFS host channel-10 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz10`]
module"]
#[doc(alias = "HCTSIZ10")]
pub type Hctsiz10 = crate::Reg<hctsiz10::Hctsiz10Spec>;
#[doc = "OTGFS host channel-10 transfer size register"]
pub mod hctsiz10;
#[doc = "HCTSIZ11 (rw) register accessor: OTGFS host channel-11 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz11`]
module"]
#[doc(alias = "HCTSIZ11")]
pub type Hctsiz11 = crate::Reg<hctsiz11::Hctsiz11Spec>;
#[doc = "OTGFS host channel-11 transfer size register"]
pub mod hctsiz11;
#[doc = "HCTSIZ12 (rw) register accessor: OTGFS host channel-12 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz12`]
module"]
#[doc(alias = "HCTSIZ12")]
pub type Hctsiz12 = crate::Reg<hctsiz12::Hctsiz12Spec>;
#[doc = "OTGFS host channel-12 transfer size register"]
pub mod hctsiz12;
#[doc = "HCTSIZ13 (rw) register accessor: OTGFS host channel-13 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz13`]
module"]
#[doc(alias = "HCTSIZ13")]
pub type Hctsiz13 = crate::Reg<hctsiz13::Hctsiz13Spec>;
#[doc = "OTGFS host channel-13 transfer size register"]
pub mod hctsiz13;
#[doc = "HCTSIZ14 (rw) register accessor: OTGFS host channel-14 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz14`]
module"]
#[doc(alias = "HCTSIZ14")]
pub type Hctsiz14 = crate::Reg<hctsiz14::Hctsiz14Spec>;
#[doc = "OTGFS host channel-14 transfer size register"]
pub mod hctsiz14;
#[doc = "HCTSIZ15 (rw) register accessor: OTGFS host channel-15 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz15`]
module"]
#[doc(alias = "HCTSIZ15")]
pub type Hctsiz15 = crate::Reg<hctsiz15::Hctsiz15Spec>;
#[doc = "OTGFS host channel-15 transfer size register"]
pub mod hctsiz15;
