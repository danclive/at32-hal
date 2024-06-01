#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmabm: Dmabm,
    dmatpd: Dmatpd,
    dmarpd: Dmarpd,
    dmardladdr: Dmardladdr,
    dmatdladdr: Dmatdladdr,
    dmasts: Dmasts,
    dmaopm: Dmaopm,
    dmaie: Dmaie,
    dmamfbocnt: Dmamfbocnt,
    _reserved9: [u8; 0x24],
    dmactd: Dmactd,
    dmacrd: Dmacrd,
    dmactbaddr: Dmactbaddr,
    dmacrbaddr: Dmacrbaddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus mode register"]
    #[inline(always)]
    pub const fn dmabm(&self) -> &Dmabm {
        &self.dmabm
    }
    #[doc = "0x04 - Ethernet DMA transmit poll demand register"]
    #[inline(always)]
    pub const fn dmatpd(&self) -> &Dmatpd {
        &self.dmatpd
    }
    #[doc = "0x08 - EHERNET DMA receive poll demand register"]
    #[inline(always)]
    pub const fn dmarpd(&self) -> &Dmarpd {
        &self.dmarpd
    }
    #[doc = "0x0c - Ethernet DMA receive descriptor list address register"]
    #[inline(always)]
    pub const fn dmardladdr(&self) -> &Dmardladdr {
        &self.dmardladdr
    }
    #[doc = "0x10 - Ethernet DMA transmit descriptor list address register"]
    #[inline(always)]
    pub const fn dmatdladdr(&self) -> &Dmatdladdr {
        &self.dmatdladdr
    }
    #[doc = "0x14 - Ethernet DMA status register"]
    #[inline(always)]
    pub const fn dmasts(&self) -> &Dmasts {
        &self.dmasts
    }
    #[doc = "0x18 - Ethernet DMA operation mode register"]
    #[inline(always)]
    pub const fn dmaopm(&self) -> &Dmaopm {
        &self.dmaopm
    }
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    #[inline(always)]
    pub const fn dmaie(&self) -> &Dmaie {
        &self.dmaie
    }
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    #[inline(always)]
    pub const fn dmamfbocnt(&self) -> &Dmamfbocnt {
        &self.dmamfbocnt
    }
    #[doc = "0x48 - Ethernet DMA current host transmit descriptor register"]
    #[inline(always)]
    pub const fn dmactd(&self) -> &Dmactd {
        &self.dmactd
    }
    #[doc = "0x4c - Ethernet DMA current host receive descriptor register"]
    #[inline(always)]
    pub const fn dmacrd(&self) -> &Dmacrd {
        &self.dmacrd
    }
    #[doc = "0x50 - Ethernet DMA current host transmit buffer address register"]
    #[inline(always)]
    pub const fn dmactbaddr(&self) -> &Dmactbaddr {
        &self.dmactbaddr
    }
    #[doc = "0x54 - Ethernet DMA current host receive buffer address register"]
    #[inline(always)]
    pub const fn dmacrbaddr(&self) -> &Dmacrbaddr {
        &self.dmacrbaddr
    }
}
#[doc = "DMABM (rw) register accessor: Ethernet DMA bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmabm`]
module"]
#[doc(alias = "DMABM")]
pub type Dmabm = crate::Reg<dmabm::DmabmSpec>;
#[doc = "Ethernet DMA bus mode register"]
pub mod dmabm;
#[doc = "DMATPD (rw) register accessor: Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatpd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatpd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatpd`]
module"]
#[doc(alias = "DMATPD")]
pub type Dmatpd = crate::Reg<dmatpd::DmatpdSpec>;
#[doc = "Ethernet DMA transmit poll demand register"]
pub mod dmatpd;
#[doc = "DMARPD (rw) register accessor: EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarpd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarpd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarpd`]
module"]
#[doc(alias = "DMARPD")]
pub type Dmarpd = crate::Reg<dmarpd::DmarpdSpec>;
#[doc = "EHERNET DMA receive poll demand register"]
pub mod dmarpd;
#[doc = "DMARDLADDR (rw) register accessor: Ethernet DMA receive descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmardladdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmardladdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmardladdr`]
module"]
#[doc(alias = "DMARDLADDR")]
pub type Dmardladdr = crate::Reg<dmardladdr::DmardladdrSpec>;
#[doc = "Ethernet DMA receive descriptor list address register"]
pub mod dmardladdr;
#[doc = "DMATDLADDR (rw) register accessor: Ethernet DMA transmit descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatdladdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdladdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatdladdr`]
module"]
#[doc(alias = "DMATDLADDR")]
pub type Dmatdladdr = crate::Reg<dmatdladdr::DmatdladdrSpec>;
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub mod dmatdladdr;
#[doc = "DMASTS (rw) register accessor: Ethernet DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasts`]
module"]
#[doc(alias = "DMASTS")]
pub type Dmasts = crate::Reg<dmasts::DmastsSpec>;
#[doc = "Ethernet DMA status register"]
pub mod dmasts;
#[doc = "DMAOPM (rw) register accessor: Ethernet DMA operation mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaopm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaopm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaopm`]
module"]
#[doc(alias = "DMAOPM")]
pub type Dmaopm = crate::Reg<dmaopm::DmaopmSpec>;
#[doc = "Ethernet DMA operation mode register"]
pub mod dmaopm;
#[doc = "DMAIE (rw) register accessor: Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaie`]
module"]
#[doc(alias = "DMAIE")]
pub type Dmaie = crate::Reg<dmaie::DmaieSpec>;
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dmaie;
#[doc = "DMAMFBOCNT (r) register accessor: Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamfbocnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamfbocnt`]
module"]
#[doc(alias = "DMAMFBOCNT")]
pub type Dmamfbocnt = crate::Reg<dmamfbocnt::DmamfbocntSpec>;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dmamfbocnt;
#[doc = "DMACTD (r) register accessor: Ethernet DMA current host transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactd`]
module"]
#[doc(alias = "DMACTD")]
pub type Dmactd = crate::Reg<dmactd::DmactdSpec>;
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub mod dmactd;
#[doc = "DMACRD (r) register accessor: Ethernet DMA current host receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrd`]
module"]
#[doc(alias = "DMACRD")]
pub type Dmacrd = crate::Reg<dmacrd::DmacrdSpec>;
#[doc = "Ethernet DMA current host receive descriptor register"]
pub mod dmacrd;
#[doc = "DMACTBADDR (r) register accessor: Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactbaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactbaddr`]
module"]
#[doc(alias = "DMACTBADDR")]
pub type Dmactbaddr = crate::Reg<dmactbaddr::DmactbaddrSpec>;
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub mod dmactbaddr;
#[doc = "DMACRBADDR (r) register accessor: Ethernet DMA current host receive buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrbaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrbaddr`]
module"]
#[doc(alias = "DMACRBADDR")]
pub type Dmacrbaddr = crate::Reg<dmacrbaddr::DmacrbaddrSpec>;
#[doc = "Ethernet DMA current host receive buffer address register"]
pub mod dmacrbaddr;
