#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gotgctl: Gotgctl,
    gotgint: Gotgint,
    gahbcfg: Gahbcfg,
    gusbcfg: Gusbcfg,
    grstctl: Grstctl,
    gintsts: Gintsts,
    gintmsk: Gintmsk,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved8: [u8; 0x04],
    grxfsiz: Grxfsiz,
    _reserved_9_dieptxf0: [u8; 0x04],
    gnptxsts: Gnptxsts,
    _reserved11: [u8; 0x08],
    gccfg: Gccfg,
    guid: Guid,
    _reserved13: [u8; 0xc0],
    hptxfsiz: Hptxfsiz,
    dieptxf1: Dieptxf1,
    dieptxf2: Dieptxf2,
    dieptxf3: Dieptxf3,
    dieptxf4: Dieptxf4,
    dieptxf5: Dieptxf5,
    dieptxf6: Dieptxf6,
    dieptxf7: Dieptxf7,
}
impl RegisterBlock {
    #[doc = "0x00 - OTGFS control and status register (OTGFS_GOTGCTL)"]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &Gotgctl {
        &self.gotgctl
    }
    #[doc = "0x04 - OTGFS interrupt register (OTGFS_GOTGINT)"]
    #[inline(always)]
    pub const fn gotgint(&self) -> &Gotgint {
        &self.gotgint
    }
    #[doc = "0x08 - OTGFS AHB configuration register (OTGFS_GAHBCFG)"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &Gahbcfg {
        &self.gahbcfg
    }
    #[doc = "0x0c - USB configuration register (OTGFS_GUSBCFG)"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &Gusbcfg {
        &self.gusbcfg
    }
    #[doc = "0x10 - OTGFS reset register (OTGFS_GRSTCTL)"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &Grstctl {
        &self.grstctl
    }
    #[doc = "0x14 - OTGFS core interrupt register (OTGFS_GINTSTS)"]
    #[inline(always)]
    pub const fn gintsts(&self) -> &Gintsts {
        &self.gintsts
    }
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &Gintmsk {
        &self.gintmsk
    }
    #[doc = "0x1c - OTGFS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GrxstsrHost {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - OTGFS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GrxstsrDevice {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x24 - OTGFS Receive FIFO size register (OTGFS_GRXFSIZ)"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &Grxfsiz {
        &self.grxfsiz
    }
    #[doc = "0x28 - OTGFS non-periodic transmit FIFO size register (Host mode)"]
    #[inline(always)]
    pub const fn gnptxfsiz(&self) -> &Gnptxfsiz {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - IN Endpoint TxFIFO 0 transmit FIFO size register (Device mode)"]
    #[inline(always)]
    pub const fn dieptxf0(&self) -> &Dieptxf0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - OTGFS non-periodic transmit FIFO/queue status register (OTGFS_GNPTXSTS)"]
    #[inline(always)]
    pub const fn gnptxsts(&self) -> &Gnptxsts {
        &self.gnptxsts
    }
    #[doc = "0x38 - OTGFS general core configuration register (OTGFS_GCCFG)"]
    #[inline(always)]
    pub const fn gccfg(&self) -> &Gccfg {
        &self.gccfg
    }
    #[doc = "0x3c - Product ID register"]
    #[inline(always)]
    pub const fn guid(&self) -> &Guid {
        &self.guid
    }
    #[doc = "0x100 - OTGFS Host periodic transmit FIFO size register (OTGFS_HPTXFSIZ)"]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &Hptxfsiz {
        &self.hptxfsiz
    }
    #[doc = "0x104 - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF1)"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &Dieptxf1 {
        &self.dieptxf1
    }
    #[doc = "0x108 - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF2)"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &Dieptxf2 {
        &self.dieptxf2
    }
    #[doc = "0x10c - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF3)"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &Dieptxf3 {
        &self.dieptxf3
    }
    #[doc = "0x110 - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF4)"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &Dieptxf4 {
        &self.dieptxf4
    }
    #[doc = "0x114 - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF5)"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &Dieptxf5 {
        &self.dieptxf5
    }
    #[doc = "0x118 - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF6)"]
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &Dieptxf6 {
        &self.dieptxf6
    }
    #[doc = "0x11c - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF7)"]
    #[inline(always)]
    pub const fn dieptxf7(&self) -> &Dieptxf7 {
        &self.dieptxf7
    }
}
#[doc = "GOTGCTL (rw) register accessor: OTGFS control and status register (OTGFS_GOTGCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`]
module"]
#[doc(alias = "GOTGCTL")]
pub type Gotgctl = crate::Reg<gotgctl::GotgctlSpec>;
#[doc = "OTGFS control and status register (OTGFS_GOTGCTL)"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: OTGFS interrupt register (OTGFS_GOTGINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`]
module"]
#[doc(alias = "GOTGINT")]
pub type Gotgint = crate::Reg<gotgint::GotgintSpec>;
#[doc = "OTGFS interrupt register (OTGFS_GOTGINT)"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: OTGFS AHB configuration register (OTGFS_GAHBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`]
module"]
#[doc(alias = "GAHBCFG")]
pub type Gahbcfg = crate::Reg<gahbcfg::GahbcfgSpec>;
#[doc = "OTGFS AHB configuration register (OTGFS_GAHBCFG)"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: USB configuration register (OTGFS_GUSBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`]
module"]
#[doc(alias = "GUSBCFG")]
pub type Gusbcfg = crate::Reg<gusbcfg::GusbcfgSpec>;
#[doc = "USB configuration register (OTGFS_GUSBCFG)"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: OTGFS reset register (OTGFS_GRSTCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
#[doc(alias = "GRSTCTL")]
pub type Grstctl = crate::Reg<grstctl::GrstctlSpec>;
#[doc = "OTGFS reset register (OTGFS_GRSTCTL)"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: OTGFS core interrupt register (OTGFS_GINTSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`]
module"]
#[doc(alias = "GINTSTS")]
pub type Gintsts = crate::Reg<gintsts::GintstsSpec>;
#[doc = "OTGFS core interrupt register (OTGFS_GINTSTS)"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`]
module"]
#[doc(alias = "GINTMSK")]
pub type Gintmsk = crate::Reg<gintmsk::GintmskSpec>;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod gintmsk;
#[doc = "GRXSTSR_Device (r) register accessor: OTGFS Receive status debug read(Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_device`]
module"]
#[doc(alias = "GRXSTSR_Device")]
pub type GrxstsrDevice = crate::Reg<grxstsr_device::GrxstsrDeviceSpec>;
#[doc = "OTGFS Receive status debug read(Device mode)"]
pub mod grxstsr_device;
#[doc = "GRXSTSR_Host (r) register accessor: OTGFS Receive status debug read(Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_host`]
module"]
#[doc(alias = "GRXSTSR_Host")]
pub type GrxstsrHost = crate::Reg<grxstsr_host::GrxstsrHostSpec>;
#[doc = "OTGFS Receive status debug read(Host mode)"]
pub mod grxstsr_host;
#[doc = "GRXFSIZ (rw) register accessor: OTGFS Receive FIFO size register (OTGFS_GRXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`]
module"]
#[doc(alias = "GRXFSIZ")]
pub type Grxfsiz = crate::Reg<grxfsiz::GrxfsizSpec>;
#[doc = "OTGFS Receive FIFO size register (OTGFS_GRXFSIZ)"]
pub mod grxfsiz;
#[doc = "DIEPTXF0 (rw) register accessor: IN Endpoint TxFIFO 0 transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf0`]
module"]
#[doc(alias = "DIEPTXF0")]
pub type Dieptxf0 = crate::Reg<dieptxf0::Dieptxf0Spec>;
#[doc = "IN Endpoint TxFIFO 0 transmit FIFO size register (Device mode)"]
pub mod dieptxf0;
#[doc = "GNPTXFSIZ (rw) register accessor: OTGFS non-periodic transmit FIFO size register (Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz`]
module"]
#[doc(alias = "GNPTXFSIZ")]
pub type Gnptxfsiz = crate::Reg<gnptxfsiz::GnptxfsizSpec>;
#[doc = "OTGFS non-periodic transmit FIFO size register (Host mode)"]
pub mod gnptxfsiz;
#[doc = "GNPTXSTS (r) register accessor: OTGFS non-periodic transmit FIFO/queue status register (OTGFS_GNPTXSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxsts`]
module"]
#[doc(alias = "GNPTXSTS")]
pub type Gnptxsts = crate::Reg<gnptxsts::GnptxstsSpec>;
#[doc = "OTGFS non-periodic transmit FIFO/queue status register (OTGFS_GNPTXSTS)"]
pub mod gnptxsts;
#[doc = "GCCFG (rw) register accessor: OTGFS general core configuration register (OTGFS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gccfg`]
module"]
#[doc(alias = "GCCFG")]
pub type Gccfg = crate::Reg<gccfg::GccfgSpec>;
#[doc = "OTGFS general core configuration register (OTGFS_GCCFG)"]
pub mod gccfg;
#[doc = "GUID (rw) register accessor: Product ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@guid`]
module"]
#[doc(alias = "GUID")]
pub type Guid = crate::Reg<guid::GuidSpec>;
#[doc = "Product ID register"]
pub mod guid;
#[doc = "HPTXFSIZ (rw) register accessor: OTGFS Host periodic transmit FIFO size register (OTGFS_HPTXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`]
module"]
#[doc(alias = "HPTXFSIZ")]
pub type Hptxfsiz = crate::Reg<hptxfsiz::HptxfsizSpec>;
#[doc = "OTGFS Host periodic transmit FIFO size register (OTGFS_HPTXFSIZ)"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`]
module"]
#[doc(alias = "DIEPTXF1")]
pub type Dieptxf1 = crate::Reg<dieptxf1::Dieptxf1Spec>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF1)"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`]
module"]
#[doc(alias = "DIEPTXF2")]
pub type Dieptxf2 = crate::Reg<dieptxf2::Dieptxf2Spec>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF2)"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`]
module"]
#[doc(alias = "DIEPTXF3")]
pub type Dieptxf3 = crate::Reg<dieptxf3::Dieptxf3Spec>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF3)"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`]
module"]
#[doc(alias = "DIEPTXF4")]
pub type Dieptxf4 = crate::Reg<dieptxf4::Dieptxf4Spec>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF4)"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf5`]
module"]
#[doc(alias = "DIEPTXF5")]
pub type Dieptxf5 = crate::Reg<dieptxf5::Dieptxf5Spec>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF5)"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf6`]
module"]
#[doc(alias = "DIEPTXF6")]
pub type Dieptxf6 = crate::Reg<dieptxf6::Dieptxf6Spec>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF6)"]
pub mod dieptxf6;
#[doc = "DIEPTXF7 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf7`]
module"]
#[doc(alias = "DIEPTXF7")]
pub type Dieptxf7 = crate::Reg<dieptxf7::Dieptxf7Spec>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF7)"]
pub mod dieptxf7;
