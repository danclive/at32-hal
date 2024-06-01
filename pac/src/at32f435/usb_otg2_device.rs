#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dcfg: Dcfg,
    dctl: Dctl,
    dsts: Dsts,
    _reserved3: [u8; 0x04],
    diepmsk: Diepmsk,
    doepmsk: Doepmsk,
    daint: Daint,
    daintmsk: Daintmsk,
    _reserved7: [u8; 0x14],
    diepempmsk: Diepempmsk,
    _reserved8: [u8; 0xc8],
    diepctl0: Diepctl0,
    _reserved9: [u8; 0x04],
    diepint0: Diepint0,
    _reserved10: [u8; 0x04],
    dieptsiz0: Dieptsiz0,
    _reserved11: [u8; 0x04],
    dtxfsts0: Dtxfsts0,
    _reserved12: [u8; 0x04],
    diepctl1: Diepctl1,
    _reserved13: [u8; 0x04],
    diepint1: Diepint1,
    _reserved14: [u8; 0x04],
    dieptsiz1: Dieptsiz1,
    _reserved15: [u8; 0x04],
    dtxfsts1: Dtxfsts1,
    _reserved16: [u8; 0x04],
    diepctl2: Diepctl2,
    _reserved17: [u8; 0x04],
    diepint2: Diepint2,
    _reserved18: [u8; 0x04],
    dieptsiz2: Dieptsiz2,
    _reserved19: [u8; 0x04],
    dtxfsts2: Dtxfsts2,
    _reserved20: [u8; 0x04],
    diepctl3: Diepctl3,
    _reserved21: [u8; 0x04],
    diepint3: Diepint3,
    _reserved22: [u8; 0x04],
    dieptsiz3: Dieptsiz3,
    _reserved23: [u8; 0x04],
    dtxfsts3: Dtxfsts3,
    _reserved24: [u8; 0x04],
    diepctl4: Diepctl4,
    _reserved25: [u8; 0x04],
    diepint4: Diepint4,
    _reserved26: [u8; 0x04],
    dieptsiz4: Dieptsiz4,
    _reserved27: [u8; 0x04],
    dtxfsts4: Dtxfsts4,
    _reserved28: [u8; 0x04],
    diepctl5: Diepctl5,
    _reserved29: [u8; 0x04],
    diepint5: Diepint5,
    _reserved30: [u8; 0x04],
    dieptsiz5: Dieptsiz5,
    _reserved31: [u8; 0x04],
    dtxfsts5: Dtxfsts5,
    _reserved32: [u8; 0x04],
    diepctl6: Diepctl6,
    _reserved33: [u8; 0x04],
    diepint6: Diepint6,
    _reserved34: [u8; 0x04],
    dieptsiz6: Dieptsiz6,
    _reserved35: [u8; 0x04],
    dtxfsts6: Dtxfsts6,
    _reserved36: [u8; 0x04],
    diepctl7: Diepctl7,
    _reserved37: [u8; 0x04],
    diepint7: Diepint7,
    _reserved38: [u8; 0x04],
    dieptsiz7: Dieptsiz7,
    _reserved39: [u8; 0x04],
    dtxfsts7: Dtxfsts7,
    _reserved40: [u8; 0x0104],
    doepctl0: Doepctl0,
    _reserved41: [u8; 0x04],
    doepint0: Doepint0,
    _reserved42: [u8; 0x04],
    doeptsiz0: Doeptsiz0,
    _reserved43: [u8; 0x0c],
    doepctl1: Doepctl1,
    _reserved44: [u8; 0x04],
    doepint1: Doepint1,
    _reserved45: [u8; 0x04],
    doeptsiz1: Doeptsiz1,
    _reserved46: [u8; 0x0c],
    doepctl2: Doepctl2,
    _reserved47: [u8; 0x04],
    doepint2: Doepint2,
    _reserved48: [u8; 0x04],
    doeptsiz2: Doeptsiz2,
    _reserved49: [u8; 0x0c],
    doepctl3: Doepctl3,
    _reserved50: [u8; 0x04],
    doepint3: Doepint3,
    _reserved51: [u8; 0x04],
    doeptsiz3: Doeptsiz3,
    _reserved52: [u8; 0x0c],
    doepctl4: Doepctl4,
    _reserved53: [u8; 0x04],
    doepint4: Doepint4,
    _reserved54: [u8; 0x04],
    doeptsiz4: Doeptsiz4,
    _reserved55: [u8; 0x0c],
    doepctl5: Doepctl5,
    _reserved56: [u8; 0x04],
    doepint5: Doepint5,
    _reserved57: [u8; 0x04],
    doeptsiz5: Doeptsiz5,
    _reserved58: [u8; 0x0c],
    doepctl6: Doepctl6,
    _reserved59: [u8; 0x04],
    doepint6: Doepint6,
    _reserved60: [u8; 0x04],
    doeptsiz6: Doeptsiz6,
    _reserved61: [u8; 0x0c],
    doepctl7: Doepctl7,
    _reserved62: [u8; 0x04],
    doepint7: Doepint7,
    _reserved63: [u8; 0x04],
    doeptsiz7: Doeptsiz7,
}
impl RegisterBlock {
    #[doc = "0x00 - OTGFS device configuration register (OTGFS_DCFG)"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &Dcfg {
        &self.dcfg
    }
    #[doc = "0x04 - OTGFS device control register (OTGFS_DCTL)"]
    #[inline(always)]
    pub const fn dctl(&self) -> &Dctl {
        &self.dctl
    }
    #[doc = "0x08 - OTGFS device status register (OTGFS_DSTS)"]
    #[inline(always)]
    pub const fn dsts(&self) -> &Dsts {
        &self.dsts
    }
    #[doc = "0x10 - OTGFS device IN endpoint common interrupt mask register (OTGFS_DIEPMSK)"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &Diepmsk {
        &self.diepmsk
    }
    #[doc = "0x14 - OTGFS device OUT endpoint common interrupt mask register (OTGFS_DOEPMSK)"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &Doepmsk {
        &self.doepmsk
    }
    #[doc = "0x18 - OTGFS device all endpoints interrupt register (OTGFS_DAINT)"]
    #[inline(always)]
    pub const fn daint(&self) -> &Daint {
        &self.daint
    }
    #[doc = "0x1c - OTGFS all endpoints interrupt mask register (OTGFS_DAINTMSK)"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &Daintmsk {
        &self.daintmsk
    }
    #[doc = "0x34 - OTGFS device IN endpoint FIFO empty interrupt mask register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &Diepempmsk {
        &self.diepempmsk
    }
    #[doc = "0x100 - OTGFS device control IN endpoint 0 control register (OTGFS_DIEPCTL0)"]
    #[inline(always)]
    pub const fn diepctl0(&self) -> &Diepctl0 {
        &self.diepctl0
    }
    #[doc = "0x108 - OTGFS device IN endpoint-0 interrupt register"]
    #[inline(always)]
    pub const fn diepint0(&self) -> &Diepint0 {
        &self.diepint0
    }
    #[doc = "0x110 - OTGFS device IN endpoint-0 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz0(&self) -> &Dieptsiz0 {
        &self.dieptsiz0
    }
    #[doc = "0x118 - OTGFS device IN endpoint-0 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts0(&self) -> &Dtxfsts0 {
        &self.dtxfsts0
    }
    #[doc = "0x120 - OTGFS device IN endpoint-1 control register"]
    #[inline(always)]
    pub const fn diepctl1(&self) -> &Diepctl1 {
        &self.diepctl1
    }
    #[doc = "0x128 - OTGFS device IN endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn diepint1(&self) -> &Diepint1 {
        &self.diepint1
    }
    #[doc = "0x130 - OTGFS device IN endpoint-1 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz1(&self) -> &Dieptsiz1 {
        &self.dieptsiz1
    }
    #[doc = "0x138 - OTGFS device IN endpoint-1 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts1(&self) -> &Dtxfsts1 {
        &self.dtxfsts1
    }
    #[doc = "0x140 - OTGFS device IN endpoint-2 control register"]
    #[inline(always)]
    pub const fn diepctl2(&self) -> &Diepctl2 {
        &self.diepctl2
    }
    #[doc = "0x148 - OTGFS device IN endpoint-2 interrupt register"]
    #[inline(always)]
    pub const fn diepint2(&self) -> &Diepint2 {
        &self.diepint2
    }
    #[doc = "0x150 - OTGFS device IN endpoint-2 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz2(&self) -> &Dieptsiz2 {
        &self.dieptsiz2
    }
    #[doc = "0x158 - OTGFS device IN endpoint-2 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts2(&self) -> &Dtxfsts2 {
        &self.dtxfsts2
    }
    #[doc = "0x160 - OTGFS device IN endpoint-3 control register"]
    #[inline(always)]
    pub const fn diepctl3(&self) -> &Diepctl3 {
        &self.diepctl3
    }
    #[doc = "0x168 - OTGFS device IN endpoint-3 interrupt register"]
    #[inline(always)]
    pub const fn diepint3(&self) -> &Diepint3 {
        &self.diepint3
    }
    #[doc = "0x170 - OTG device IN endpoint-3 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz3(&self) -> &Dieptsiz3 {
        &self.dieptsiz3
    }
    #[doc = "0x178 - OTGFS device IN endpoint-3 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts3(&self) -> &Dtxfsts3 {
        &self.dtxfsts3
    }
    #[doc = "0x180 - OTGFS device IN endpoint-4 control register"]
    #[inline(always)]
    pub const fn diepctl4(&self) -> &Diepctl4 {
        &self.diepctl4
    }
    #[doc = "0x188 - OTGFS device IN endpoint-4 interrupt register"]
    #[inline(always)]
    pub const fn diepint4(&self) -> &Diepint4 {
        &self.diepint4
    }
    #[doc = "0x190 - OTG device IN endpoint-4 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz4(&self) -> &Dieptsiz4 {
        &self.dieptsiz4
    }
    #[doc = "0x198 - OTGFS device IN endpoint-4 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts4(&self) -> &Dtxfsts4 {
        &self.dtxfsts4
    }
    #[doc = "0x1a0 - OTGFS device IN endpoint-5 control register"]
    #[inline(always)]
    pub const fn diepctl5(&self) -> &Diepctl5 {
        &self.diepctl5
    }
    #[doc = "0x1a8 - OTGFS device IN endpoint-5 interrupt register"]
    #[inline(always)]
    pub const fn diepint5(&self) -> &Diepint5 {
        &self.diepint5
    }
    #[doc = "0x1b0 - OTG device IN endpoint-5 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz5(&self) -> &Dieptsiz5 {
        &self.dieptsiz5
    }
    #[doc = "0x1b8 - OTGFS device IN endpoint-5 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts5(&self) -> &Dtxfsts5 {
        &self.dtxfsts5
    }
    #[doc = "0x1c0 - OTGFS device IN endpoint-6 control register"]
    #[inline(always)]
    pub const fn diepctl6(&self) -> &Diepctl6 {
        &self.diepctl6
    }
    #[doc = "0x1c8 - OTGFS device IN endpoint-6 interrupt register"]
    #[inline(always)]
    pub const fn diepint6(&self) -> &Diepint6 {
        &self.diepint6
    }
    #[doc = "0x1d0 - OTG device IN endpoint-6 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz6(&self) -> &Dieptsiz6 {
        &self.dieptsiz6
    }
    #[doc = "0x1d8 - OTGFS device IN endpoint-6 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts6(&self) -> &Dtxfsts6 {
        &self.dtxfsts6
    }
    #[doc = "0x1e0 - OTGFS device IN endpoint-7 control register"]
    #[inline(always)]
    pub const fn diepctl7(&self) -> &Diepctl7 {
        &self.diepctl7
    }
    #[doc = "0x1e8 - OTGFS device IN endpoint-7 interrupt register"]
    #[inline(always)]
    pub const fn diepint7(&self) -> &Diepint7 {
        &self.diepint7
    }
    #[doc = "0x1f0 - OTG device IN endpoint-7 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz7(&self) -> &Dieptsiz7 {
        &self.dieptsiz7
    }
    #[doc = "0x1f8 - OTGFS device IN endpoint-7 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts7(&self) -> &Dtxfsts7 {
        &self.dtxfsts7
    }
    #[doc = "0x300 - OTGFS device OUT endpoint-0 control register"]
    #[inline(always)]
    pub const fn doepctl0(&self) -> &Doepctl0 {
        &self.doepctl0
    }
    #[doc = "0x308 - OTGFS device OUT endpoint-0 interrupt register"]
    #[inline(always)]
    pub const fn doepint0(&self) -> &Doepint0 {
        &self.doepint0
    }
    #[doc = "0x310 - OTGFS device OUT endpoint-0 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz0(&self) -> &Doeptsiz0 {
        &self.doeptsiz0
    }
    #[doc = "0x320 - OTGFS device OUT endpoint-1 control register"]
    #[inline(always)]
    pub const fn doepctl1(&self) -> &Doepctl1 {
        &self.doepctl1
    }
    #[doc = "0x328 - OTGFS device OUT endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn doepint1(&self) -> &Doepint1 {
        &self.doepint1
    }
    #[doc = "0x330 - OTGFS device OUT endpoint-1 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz1(&self) -> &Doeptsiz1 {
        &self.doeptsiz1
    }
    #[doc = "0x340 - OTGFS device OUT endpoint-2 control register"]
    #[inline(always)]
    pub const fn doepctl2(&self) -> &Doepctl2 {
        &self.doepctl2
    }
    #[doc = "0x348 - OTGFS device OUT endpoint-2 interrupt register"]
    #[inline(always)]
    pub const fn doepint2(&self) -> &Doepint2 {
        &self.doepint2
    }
    #[doc = "0x350 - OTGFS device OUT endpoint-2 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz2(&self) -> &Doeptsiz2 {
        &self.doeptsiz2
    }
    #[doc = "0x360 - OTGFS device OUT endpoint-3 control register"]
    #[inline(always)]
    pub const fn doepctl3(&self) -> &Doepctl3 {
        &self.doepctl3
    }
    #[doc = "0x368 - OTGFS device OUT endpoint-3 interrupt register"]
    #[inline(always)]
    pub const fn doepint3(&self) -> &Doepint3 {
        &self.doepint3
    }
    #[doc = "0x370 - OTGFS device OUT endpoint-3 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz3(&self) -> &Doeptsiz3 {
        &self.doeptsiz3
    }
    #[doc = "0x380 - OTGFS device OUT endpoint-4 control register"]
    #[inline(always)]
    pub const fn doepctl4(&self) -> &Doepctl4 {
        &self.doepctl4
    }
    #[doc = "0x388 - OTGFS device OUT endpoint-4 interrupt register"]
    #[inline(always)]
    pub const fn doepint4(&self) -> &Doepint4 {
        &self.doepint4
    }
    #[doc = "0x390 - OTGFS device OUT endpoint-4 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz4(&self) -> &Doeptsiz4 {
        &self.doeptsiz4
    }
    #[doc = "0x3a0 - OTGFS device OUT endpoint-5 control register"]
    #[inline(always)]
    pub const fn doepctl5(&self) -> &Doepctl5 {
        &self.doepctl5
    }
    #[doc = "0x3a8 - OTGFS device OUT endpoint-5 interrupt register"]
    #[inline(always)]
    pub const fn doepint5(&self) -> &Doepint5 {
        &self.doepint5
    }
    #[doc = "0x3b0 - OTGFS device OUT endpoint-5 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz5(&self) -> &Doeptsiz5 {
        &self.doeptsiz5
    }
    #[doc = "0x3c0 - OTGFS device OUT endpoint-6 control register"]
    #[inline(always)]
    pub const fn doepctl6(&self) -> &Doepctl6 {
        &self.doepctl6
    }
    #[doc = "0x3c8 - OTGFS device OUT endpoint-6 interrupt register"]
    #[inline(always)]
    pub const fn doepint6(&self) -> &Doepint6 {
        &self.doepint6
    }
    #[doc = "0x3d0 - OTGFS device OUT endpoint-6 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz6(&self) -> &Doeptsiz6 {
        &self.doeptsiz6
    }
    #[doc = "0x3e0 - OTGFS device OUT endpoint-7 control register"]
    #[inline(always)]
    pub const fn doepctl7(&self) -> &Doepctl7 {
        &self.doepctl7
    }
    #[doc = "0x3e8 - OTGFS device OUT endpoint-7 interrupt register"]
    #[inline(always)]
    pub const fn doepint7(&self) -> &Doepint7 {
        &self.doepint7
    }
    #[doc = "0x3f0 - OTGFS device OUT endpoint-7 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz7(&self) -> &Doeptsiz7 {
        &self.doeptsiz7
    }
}
#[doc = "DCFG (rw) register accessor: OTGFS device configuration register (OTGFS_DCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
#[doc(alias = "DCFG")]
pub type Dcfg = crate::Reg<dcfg::DcfgSpec>;
#[doc = "OTGFS device configuration register (OTGFS_DCFG)"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: OTGFS device control register (OTGFS_DCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
#[doc(alias = "DCTL")]
pub type Dctl = crate::Reg<dctl::DctlSpec>;
#[doc = "OTGFS device control register (OTGFS_DCTL)"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: OTGFS device status register (OTGFS_DSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
#[doc(alias = "DSTS")]
pub type Dsts = crate::Reg<dsts::DstsSpec>;
#[doc = "OTGFS device status register (OTGFS_DSTS)"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: OTGFS device IN endpoint common interrupt mask register (OTGFS_DIEPMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
#[doc(alias = "DIEPMSK")]
pub type Diepmsk = crate::Reg<diepmsk::DiepmskSpec>;
#[doc = "OTGFS device IN endpoint common interrupt mask register (OTGFS_DIEPMSK)"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: OTGFS device OUT endpoint common interrupt mask register (OTGFS_DOEPMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
#[doc(alias = "DOEPMSK")]
pub type Doepmsk = crate::Reg<doepmsk::DoepmskSpec>;
#[doc = "OTGFS device OUT endpoint common interrupt mask register (OTGFS_DOEPMSK)"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: OTGFS device all endpoints interrupt register (OTGFS_DAINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
#[doc(alias = "DAINT")]
pub type Daint = crate::Reg<daint::DaintSpec>;
#[doc = "OTGFS device all endpoints interrupt register (OTGFS_DAINT)"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: OTGFS all endpoints interrupt mask register (OTGFS_DAINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
#[doc(alias = "DAINTMSK")]
pub type Daintmsk = crate::Reg<daintmsk::DaintmskSpec>;
#[doc = "OTGFS all endpoints interrupt mask register (OTGFS_DAINTMSK)"]
pub mod daintmsk;
#[doc = "DIEPEMPMSK (rw) register accessor: OTGFS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
#[doc(alias = "DIEPEMPMSK")]
pub type Diepempmsk = crate::Reg<diepempmsk::DiepempmskSpec>;
#[doc = "OTGFS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DIEPCTL0 (rw) register accessor: OTGFS device control IN endpoint 0 control register (OTGFS_DIEPCTL0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl0`]
module"]
#[doc(alias = "DIEPCTL0")]
pub type Diepctl0 = crate::Reg<diepctl0::Diepctl0Spec>;
#[doc = "OTGFS device control IN endpoint 0 control register (OTGFS_DIEPCTL0)"]
pub mod diepctl0;
#[doc = "DIEPCTL1 (rw) register accessor: OTGFS device IN endpoint-1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl1`]
module"]
#[doc(alias = "DIEPCTL1")]
pub type Diepctl1 = crate::Reg<diepctl1::Diepctl1Spec>;
#[doc = "OTGFS device IN endpoint-1 control register"]
pub mod diepctl1;
#[doc = "DIEPCTL2 (rw) register accessor: OTGFS device IN endpoint-2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl2`]
module"]
#[doc(alias = "DIEPCTL2")]
pub type Diepctl2 = crate::Reg<diepctl2::Diepctl2Spec>;
#[doc = "OTGFS device IN endpoint-2 control register"]
pub mod diepctl2;
#[doc = "DIEPCTL3 (rw) register accessor: OTGFS device IN endpoint-3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl3`]
module"]
#[doc(alias = "DIEPCTL3")]
pub type Diepctl3 = crate::Reg<diepctl3::Diepctl3Spec>;
#[doc = "OTGFS device IN endpoint-3 control register"]
pub mod diepctl3;
#[doc = "DIEPCTL4 (rw) register accessor: OTGFS device IN endpoint-4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl4`]
module"]
#[doc(alias = "DIEPCTL4")]
pub type Diepctl4 = crate::Reg<diepctl4::Diepctl4Spec>;
#[doc = "OTGFS device IN endpoint-4 control register"]
pub mod diepctl4;
#[doc = "DIEPCTL5 (rw) register accessor: OTGFS device IN endpoint-5 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl5`]
module"]
#[doc(alias = "DIEPCTL5")]
pub type Diepctl5 = crate::Reg<diepctl5::Diepctl5Spec>;
#[doc = "OTGFS device IN endpoint-5 control register"]
pub mod diepctl5;
#[doc = "DIEPCTL6 (rw) register accessor: OTGFS device IN endpoint-6 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl6`]
module"]
#[doc(alias = "DIEPCTL6")]
pub type Diepctl6 = crate::Reg<diepctl6::Diepctl6Spec>;
#[doc = "OTGFS device IN endpoint-6 control register"]
pub mod diepctl6;
#[doc = "DIEPCTL7 (rw) register accessor: OTGFS device IN endpoint-7 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl7`]
module"]
#[doc(alias = "DIEPCTL7")]
pub type Diepctl7 = crate::Reg<diepctl7::Diepctl7Spec>;
#[doc = "OTGFS device IN endpoint-7 control register"]
pub mod diepctl7;
#[doc = "DOEPCTL0 (rw) register accessor: OTGFS device OUT endpoint-0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl0`]
module"]
#[doc(alias = "DOEPCTL0")]
pub type Doepctl0 = crate::Reg<doepctl0::Doepctl0Spec>;
#[doc = "OTGFS device OUT endpoint-0 control register"]
pub mod doepctl0;
#[doc = "DOEPCTL1 (rw) register accessor: OTGFS device OUT endpoint-1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl1`]
module"]
#[doc(alias = "DOEPCTL1")]
pub type Doepctl1 = crate::Reg<doepctl1::Doepctl1Spec>;
#[doc = "OTGFS device OUT endpoint-1 control register"]
pub mod doepctl1;
#[doc = "DOEPCTL2 (rw) register accessor: OTGFS device OUT endpoint-2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl2`]
module"]
#[doc(alias = "DOEPCTL2")]
pub type Doepctl2 = crate::Reg<doepctl2::Doepctl2Spec>;
#[doc = "OTGFS device OUT endpoint-2 control register"]
pub mod doepctl2;
#[doc = "DOEPCTL3 (rw) register accessor: OTGFS device OUT endpoint-3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl3`]
module"]
#[doc(alias = "DOEPCTL3")]
pub type Doepctl3 = crate::Reg<doepctl3::Doepctl3Spec>;
#[doc = "OTGFS device OUT endpoint-3 control register"]
pub mod doepctl3;
#[doc = "DOEPCTL4 (rw) register accessor: OTGFS device OUT endpoint-4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl4`]
module"]
#[doc(alias = "DOEPCTL4")]
pub type Doepctl4 = crate::Reg<doepctl4::Doepctl4Spec>;
#[doc = "OTGFS device OUT endpoint-4 control register"]
pub mod doepctl4;
#[doc = "DOEPCTL5 (rw) register accessor: OTGFS device OUT endpoint-5 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl5`]
module"]
#[doc(alias = "DOEPCTL5")]
pub type Doepctl5 = crate::Reg<doepctl5::Doepctl5Spec>;
#[doc = "OTGFS device OUT endpoint-5 control register"]
pub mod doepctl5;
#[doc = "DOEPCTL6 (rw) register accessor: OTGFS device OUT endpoint-6 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl6`]
module"]
#[doc(alias = "DOEPCTL6")]
pub type Doepctl6 = crate::Reg<doepctl6::Doepctl6Spec>;
#[doc = "OTGFS device OUT endpoint-6 control register"]
pub mod doepctl6;
#[doc = "DOEPCTL7 (rw) register accessor: OTGFS device OUT endpoint-7 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl7`]
module"]
#[doc(alias = "DOEPCTL7")]
pub type Doepctl7 = crate::Reg<doepctl7::Doepctl7Spec>;
#[doc = "OTGFS device OUT endpoint-7 control register"]
pub mod doepctl7;
#[doc = "DIEPINT0 (rw) register accessor: OTGFS device IN endpoint-0 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint0`]
module"]
#[doc(alias = "DIEPINT0")]
pub type Diepint0 = crate::Reg<diepint0::Diepint0Spec>;
#[doc = "OTGFS device IN endpoint-0 interrupt register"]
pub mod diepint0;
#[doc = "DIEPINT1 (rw) register accessor: OTGFS device IN endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint1`]
module"]
#[doc(alias = "DIEPINT1")]
pub type Diepint1 = crate::Reg<diepint1::Diepint1Spec>;
#[doc = "OTGFS device IN endpoint-1 interrupt register"]
pub mod diepint1;
#[doc = "DIEPINT2 (rw) register accessor: OTGFS device IN endpoint-2 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint2`]
module"]
#[doc(alias = "DIEPINT2")]
pub type Diepint2 = crate::Reg<diepint2::Diepint2Spec>;
#[doc = "OTGFS device IN endpoint-2 interrupt register"]
pub mod diepint2;
#[doc = "DIEPINT3 (rw) register accessor: OTGFS device IN endpoint-3 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint3`]
module"]
#[doc(alias = "DIEPINT3")]
pub type Diepint3 = crate::Reg<diepint3::Diepint3Spec>;
#[doc = "OTGFS device IN endpoint-3 interrupt register"]
pub mod diepint3;
#[doc = "DIEPINT4 (rw) register accessor: OTGFS device IN endpoint-4 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint4`]
module"]
#[doc(alias = "DIEPINT4")]
pub type Diepint4 = crate::Reg<diepint4::Diepint4Spec>;
#[doc = "OTGFS device IN endpoint-4 interrupt register"]
pub mod diepint4;
#[doc = "DIEPINT5 (rw) register accessor: OTGFS device IN endpoint-5 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint5`]
module"]
#[doc(alias = "DIEPINT5")]
pub type Diepint5 = crate::Reg<diepint5::Diepint5Spec>;
#[doc = "OTGFS device IN endpoint-5 interrupt register"]
pub mod diepint5;
#[doc = "DIEPINT6 (rw) register accessor: OTGFS device IN endpoint-6 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint6`]
module"]
#[doc(alias = "DIEPINT6")]
pub type Diepint6 = crate::Reg<diepint6::Diepint6Spec>;
#[doc = "OTGFS device IN endpoint-6 interrupt register"]
pub mod diepint6;
#[doc = "DIEPINT7 (rw) register accessor: OTGFS device IN endpoint-7 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint7`]
module"]
#[doc(alias = "DIEPINT7")]
pub type Diepint7 = crate::Reg<diepint7::Diepint7Spec>;
#[doc = "OTGFS device IN endpoint-7 interrupt register"]
pub mod diepint7;
#[doc = "DOEPINT0 (rw) register accessor: OTGFS device OUT endpoint-0 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint0`]
module"]
#[doc(alias = "DOEPINT0")]
pub type Doepint0 = crate::Reg<doepint0::Doepint0Spec>;
#[doc = "OTGFS device OUT endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "DOEPINT1 (rw) register accessor: OTGFS device OUT endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint1`]
module"]
#[doc(alias = "DOEPINT1")]
pub type Doepint1 = crate::Reg<doepint1::Doepint1Spec>;
#[doc = "OTGFS device OUT endpoint-1 interrupt register"]
pub mod doepint1;
#[doc = "DOEPINT2 (rw) register accessor: OTGFS device OUT endpoint-2 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint2`]
module"]
#[doc(alias = "DOEPINT2")]
pub type Doepint2 = crate::Reg<doepint2::Doepint2Spec>;
#[doc = "OTGFS device OUT endpoint-2 interrupt register"]
pub mod doepint2;
#[doc = "DOEPINT3 (rw) register accessor: OTGFS device OUT endpoint-3 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint3`]
module"]
#[doc(alias = "DOEPINT3")]
pub type Doepint3 = crate::Reg<doepint3::Doepint3Spec>;
#[doc = "OTGFS device OUT endpoint-3 interrupt register"]
pub mod doepint3;
#[doc = "DOEPINT4 (rw) register accessor: OTGFS device OUT endpoint-4 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint4`]
module"]
#[doc(alias = "DOEPINT4")]
pub type Doepint4 = crate::Reg<doepint4::Doepint4Spec>;
#[doc = "OTGFS device OUT endpoint-4 interrupt register"]
pub mod doepint4;
#[doc = "DOEPINT5 (rw) register accessor: OTGFS device OUT endpoint-5 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint5`]
module"]
#[doc(alias = "DOEPINT5")]
pub type Doepint5 = crate::Reg<doepint5::Doepint5Spec>;
#[doc = "OTGFS device OUT endpoint-5 interrupt register"]
pub mod doepint5;
#[doc = "DOEPINT6 (rw) register accessor: OTGFS device OUT endpoint-6 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint6`]
module"]
#[doc(alias = "DOEPINT6")]
pub type Doepint6 = crate::Reg<doepint6::Doepint6Spec>;
#[doc = "OTGFS device OUT endpoint-6 interrupt register"]
pub mod doepint6;
#[doc = "DOEPINT7 (rw) register accessor: OTGFS device OUT endpoint-7 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint7`]
module"]
#[doc(alias = "DOEPINT7")]
pub type Doepint7 = crate::Reg<doepint7::Doepint7Spec>;
#[doc = "OTGFS device OUT endpoint-7 interrupt register"]
pub mod doepint7;
#[doc = "DIEPTSIZ0 (rw) register accessor: OTGFS device IN endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz0`]
module"]
#[doc(alias = "DIEPTSIZ0")]
pub type Dieptsiz0 = crate::Reg<dieptsiz0::Dieptsiz0Spec>;
#[doc = "OTGFS device IN endpoint-0 transfer size register"]
pub mod dieptsiz0;
#[doc = "DOEPTSIZ0 (rw) register accessor: OTGFS device OUT endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz0`]
module"]
#[doc(alias = "DOEPTSIZ0")]
pub type Doeptsiz0 = crate::Reg<doeptsiz0::Doeptsiz0Spec>;
#[doc = "OTGFS device OUT endpoint-0 transfer size register"]
pub mod doeptsiz0;
#[doc = "DIEPTSIZ1 (rw) register accessor: OTGFS device IN endpoint-1 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz1`]
module"]
#[doc(alias = "DIEPTSIZ1")]
pub type Dieptsiz1 = crate::Reg<dieptsiz1::Dieptsiz1Spec>;
#[doc = "OTGFS device IN endpoint-1 transfer size register"]
pub mod dieptsiz1;
#[doc = "DIEPTSIZ2 (rw) register accessor: OTGFS device IN endpoint-2 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz2`]
module"]
#[doc(alias = "DIEPTSIZ2")]
pub type Dieptsiz2 = crate::Reg<dieptsiz2::Dieptsiz2Spec>;
#[doc = "OTGFS device IN endpoint-2 transfer size register"]
pub mod dieptsiz2;
#[doc = "DIEPTSIZ3 (rw) register accessor: OTG device IN endpoint-3 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz3`]
module"]
#[doc(alias = "DIEPTSIZ3")]
pub type Dieptsiz3 = crate::Reg<dieptsiz3::Dieptsiz3Spec>;
#[doc = "OTG device IN endpoint-3 transfer size register"]
pub mod dieptsiz3;
#[doc = "DIEPTSIZ4 (rw) register accessor: OTG device IN endpoint-4 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz4`]
module"]
#[doc(alias = "DIEPTSIZ4")]
pub type Dieptsiz4 = crate::Reg<dieptsiz4::Dieptsiz4Spec>;
#[doc = "OTG device IN endpoint-4 transfer size register"]
pub mod dieptsiz4;
#[doc = "DIEPTSIZ5 (rw) register accessor: OTG device IN endpoint-5 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz5`]
module"]
#[doc(alias = "DIEPTSIZ5")]
pub type Dieptsiz5 = crate::Reg<dieptsiz5::Dieptsiz5Spec>;
#[doc = "OTG device IN endpoint-5 transfer size register"]
pub mod dieptsiz5;
#[doc = "DIEPTSIZ6 (rw) register accessor: OTG device IN endpoint-6 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz6`]
module"]
#[doc(alias = "DIEPTSIZ6")]
pub type Dieptsiz6 = crate::Reg<dieptsiz6::Dieptsiz6Spec>;
#[doc = "OTG device IN endpoint-6 transfer size register"]
pub mod dieptsiz6;
#[doc = "DIEPTSIZ7 (rw) register accessor: OTG device IN endpoint-7 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz7`]
module"]
#[doc(alias = "DIEPTSIZ7")]
pub type Dieptsiz7 = crate::Reg<dieptsiz7::Dieptsiz7Spec>;
#[doc = "OTG device IN endpoint-7 transfer size register"]
pub mod dieptsiz7;
#[doc = "DTXFSTS0 (r) register accessor: OTGFS device IN endpoint-0 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts0`]
module"]
#[doc(alias = "DTXFSTS0")]
pub type Dtxfsts0 = crate::Reg<dtxfsts0::Dtxfsts0Spec>;
#[doc = "OTGFS device IN endpoint-0 transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "DTXFSTS1 (r) register accessor: OTGFS device IN endpoint-1 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts1`]
module"]
#[doc(alias = "DTXFSTS1")]
pub type Dtxfsts1 = crate::Reg<dtxfsts1::Dtxfsts1Spec>;
#[doc = "OTGFS device IN endpoint-1 transmit FIFO status register"]
pub mod dtxfsts1;
#[doc = "DTXFSTS2 (r) register accessor: OTGFS device IN endpoint-2 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts2`]
module"]
#[doc(alias = "DTXFSTS2")]
pub type Dtxfsts2 = crate::Reg<dtxfsts2::Dtxfsts2Spec>;
#[doc = "OTGFS device IN endpoint-2 transmit FIFO status register"]
pub mod dtxfsts2;
#[doc = "DTXFSTS3 (r) register accessor: OTGFS device IN endpoint-3 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts3`]
module"]
#[doc(alias = "DTXFSTS3")]
pub type Dtxfsts3 = crate::Reg<dtxfsts3::Dtxfsts3Spec>;
#[doc = "OTGFS device IN endpoint-3 transmit FIFO status register"]
pub mod dtxfsts3;
#[doc = "DTXFSTS4 (r) register accessor: OTGFS device IN endpoint-4 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts4`]
module"]
#[doc(alias = "DTXFSTS4")]
pub type Dtxfsts4 = crate::Reg<dtxfsts4::Dtxfsts4Spec>;
#[doc = "OTGFS device IN endpoint-4 transmit FIFO status register"]
pub mod dtxfsts4;
#[doc = "DTXFSTS5 (r) register accessor: OTGFS device IN endpoint-5 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts5`]
module"]
#[doc(alias = "DTXFSTS5")]
pub type Dtxfsts5 = crate::Reg<dtxfsts5::Dtxfsts5Spec>;
#[doc = "OTGFS device IN endpoint-5 transmit FIFO status register"]
pub mod dtxfsts5;
#[doc = "DTXFSTS6 (r) register accessor: OTGFS device IN endpoint-6 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts6`]
module"]
#[doc(alias = "DTXFSTS6")]
pub type Dtxfsts6 = crate::Reg<dtxfsts6::Dtxfsts6Spec>;
#[doc = "OTGFS device IN endpoint-6 transmit FIFO status register"]
pub mod dtxfsts6;
#[doc = "DTXFSTS7 (r) register accessor: OTGFS device IN endpoint-7 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts7`]
module"]
#[doc(alias = "DTXFSTS7")]
pub type Dtxfsts7 = crate::Reg<dtxfsts7::Dtxfsts7Spec>;
#[doc = "OTGFS device IN endpoint-7 transmit FIFO status register"]
pub mod dtxfsts7;
#[doc = "DOEPTSIZ1 (rw) register accessor: OTGFS device OUT endpoint-1 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz1`]
module"]
#[doc(alias = "DOEPTSIZ1")]
pub type Doeptsiz1 = crate::Reg<doeptsiz1::Doeptsiz1Spec>;
#[doc = "OTGFS device OUT endpoint-1 transfer size register"]
pub mod doeptsiz1;
#[doc = "DOEPTSIZ2 (rw) register accessor: OTGFS device OUT endpoint-2 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz2`]
module"]
#[doc(alias = "DOEPTSIZ2")]
pub type Doeptsiz2 = crate::Reg<doeptsiz2::Doeptsiz2Spec>;
#[doc = "OTGFS device OUT endpoint-2 transfer size register"]
pub mod doeptsiz2;
#[doc = "DOEPTSIZ3 (rw) register accessor: OTGFS device OUT endpoint-3 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz3`]
module"]
#[doc(alias = "DOEPTSIZ3")]
pub type Doeptsiz3 = crate::Reg<doeptsiz3::Doeptsiz3Spec>;
#[doc = "OTGFS device OUT endpoint-3 transfer size register"]
pub mod doeptsiz3;
#[doc = "DOEPTSIZ4 (rw) register accessor: OTGFS device OUT endpoint-4 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz4`]
module"]
#[doc(alias = "DOEPTSIZ4")]
pub type Doeptsiz4 = crate::Reg<doeptsiz4::Doeptsiz4Spec>;
#[doc = "OTGFS device OUT endpoint-4 transfer size register"]
pub mod doeptsiz4;
#[doc = "DOEPTSIZ5 (rw) register accessor: OTGFS device OUT endpoint-5 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz5`]
module"]
#[doc(alias = "DOEPTSIZ5")]
pub type Doeptsiz5 = crate::Reg<doeptsiz5::Doeptsiz5Spec>;
#[doc = "OTGFS device OUT endpoint-5 transfer size register"]
pub mod doeptsiz5;
#[doc = "DOEPTSIZ6 (rw) register accessor: OTGFS device OUT endpoint-6 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz6`]
module"]
#[doc(alias = "DOEPTSIZ6")]
pub type Doeptsiz6 = crate::Reg<doeptsiz6::Doeptsiz6Spec>;
#[doc = "OTGFS device OUT endpoint-6 transfer size register"]
pub mod doeptsiz6;
#[doc = "DOEPTSIZ7 (rw) register accessor: OTGFS device OUT endpoint-7 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz7`]
module"]
#[doc(alias = "DOEPTSIZ7")]
pub type Doeptsiz7 = crate::Reg<doeptsiz7::Doeptsiz7Spec>;
#[doc = "OTGFS device OUT endpoint-7 transfer size register"]
pub mod doeptsiz7;
