#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    evtout: Evtout,
    remap: Remap,
    exintc1: Exintc1,
    exintc2: Exintc2,
    exintc3: Exintc3,
    exintc4: Exintc4,
    _reserved6: [u8; 0x04],
    remap2: Remap2,
    remap3: Remap3,
    remap4: Remap4,
    remap5: Remap5,
    remap6: Remap6,
    remap7: Remap7,
    remap8: Remap8,
}
impl RegisterBlock {
    #[doc = "0x00 - Event output register (IOMUX_EVTOUT)"]
    #[inline(always)]
    pub const fn evtout(&self) -> &Evtout {
        &self.evtout
    }
    #[doc = "0x04 - IO MUX remap register"]
    #[inline(always)]
    pub const fn remap(&self) -> &Remap {
        &self.remap
    }
    #[doc = "0x08 - External interrupt configuration register 1 (IOMUX_EXINTC1)"]
    #[inline(always)]
    pub const fn exintc1(&self) -> &Exintc1 {
        &self.exintc1
    }
    #[doc = "0x0c - External interrupt configuration register 2 (IOMUX_EXINTC2)"]
    #[inline(always)]
    pub const fn exintc2(&self) -> &Exintc2 {
        &self.exintc2
    }
    #[doc = "0x10 - External interrupt configuration register 3 (IOMUX_EXINTC3)"]
    #[inline(always)]
    pub const fn exintc3(&self) -> &Exintc3 {
        &self.exintc3
    }
    #[doc = "0x14 - External interrupt configuration register 4 (IOMUX_EXINTC4)"]
    #[inline(always)]
    pub const fn exintc4(&self) -> &Exintc4 {
        &self.exintc4
    }
    #[doc = "0x1c - IO MUX remap register 2"]
    #[inline(always)]
    pub const fn remap2(&self) -> &Remap2 {
        &self.remap2
    }
    #[doc = "0x20 - IO MUX remap register 3"]
    #[inline(always)]
    pub const fn remap3(&self) -> &Remap3 {
        &self.remap3
    }
    #[doc = "0x24 - IO MUX remap register 4"]
    #[inline(always)]
    pub const fn remap4(&self) -> &Remap4 {
        &self.remap4
    }
    #[doc = "0x28 - IO MUX remap register 5"]
    #[inline(always)]
    pub const fn remap5(&self) -> &Remap5 {
        &self.remap5
    }
    #[doc = "0x2c - IO MUX remap register 6"]
    #[inline(always)]
    pub const fn remap6(&self) -> &Remap6 {
        &self.remap6
    }
    #[doc = "0x30 - IO MUX remap register 7"]
    #[inline(always)]
    pub const fn remap7(&self) -> &Remap7 {
        &self.remap7
    }
    #[doc = "0x34 - IO MUX remap register 8"]
    #[inline(always)]
    pub const fn remap8(&self) -> &Remap8 {
        &self.remap8
    }
}
#[doc = "EVTOUT (rw) register accessor: Event output register (IOMUX_EVTOUT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtout`]
module"]
#[doc(alias = "EVTOUT")]
pub type Evtout = crate::Reg<evtout::EvtoutSpec>;
#[doc = "Event output register (IOMUX_EVTOUT)"]
pub mod evtout;
#[doc = "REMAP (rw) register accessor: IO MUX remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap`]
module"]
#[doc(alias = "REMAP")]
pub type Remap = crate::Reg<remap::RemapSpec>;
#[doc = "IO MUX remap register"]
pub mod remap;
#[doc = "EXINTC1 (rw) register accessor: External interrupt configuration register 1 (IOMUX_EXINTC1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc1`]
module"]
#[doc(alias = "EXINTC1")]
pub type Exintc1 = crate::Reg<exintc1::Exintc1Spec>;
#[doc = "External interrupt configuration register 1 (IOMUX_EXINTC1)"]
pub mod exintc1;
#[doc = "EXINTC2 (rw) register accessor: External interrupt configuration register 2 (IOMUX_EXINTC2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc2`]
module"]
#[doc(alias = "EXINTC2")]
pub type Exintc2 = crate::Reg<exintc2::Exintc2Spec>;
#[doc = "External interrupt configuration register 2 (IOMUX_EXINTC2)"]
pub mod exintc2;
#[doc = "EXINTC3 (rw) register accessor: External interrupt configuration register 3 (IOMUX_EXINTC3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc3`]
module"]
#[doc(alias = "EXINTC3")]
pub type Exintc3 = crate::Reg<exintc3::Exintc3Spec>;
#[doc = "External interrupt configuration register 3 (IOMUX_EXINTC3)"]
pub mod exintc3;
#[doc = "EXINTC4 (rw) register accessor: External interrupt configuration register 4 (IOMUX_EXINTC4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc4`]
module"]
#[doc(alias = "EXINTC4")]
pub type Exintc4 = crate::Reg<exintc4::Exintc4Spec>;
#[doc = "External interrupt configuration register 4 (IOMUX_EXINTC4)"]
pub mod exintc4;
#[doc = "REMAP2 (w) register accessor: IO MUX remap register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap2`]
module"]
#[doc(alias = "REMAP2")]
pub type Remap2 = crate::Reg<remap2::Remap2Spec>;
#[doc = "IO MUX remap register 2"]
pub mod remap2;
#[doc = "REMAP3 (rw) register accessor: IO MUX remap register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap3`]
module"]
#[doc(alias = "REMAP3")]
pub type Remap3 = crate::Reg<remap3::Remap3Spec>;
#[doc = "IO MUX remap register 3"]
pub mod remap3;
#[doc = "REMAP4 (rw) register accessor: IO MUX remap register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap4`]
module"]
#[doc(alias = "REMAP4")]
pub type Remap4 = crate::Reg<remap4::Remap4Spec>;
#[doc = "IO MUX remap register 4"]
pub mod remap4;
#[doc = "REMAP5 (rw) register accessor: IO MUX remap register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap5`]
module"]
#[doc(alias = "REMAP5")]
pub type Remap5 = crate::Reg<remap5::Remap5Spec>;
#[doc = "IO MUX remap register 5"]
pub mod remap5;
#[doc = "REMAP6 (rw) register accessor: IO MUX remap register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap6`]
module"]
#[doc(alias = "REMAP6")]
pub type Remap6 = crate::Reg<remap6::Remap6Spec>;
#[doc = "IO MUX remap register 6"]
pub mod remap6;
#[doc = "REMAP7 (rw) register accessor: IO MUX remap register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap7`]
module"]
#[doc(alias = "REMAP7")]
pub type Remap7 = crate::Reg<remap7::Remap7Spec>;
#[doc = "IO MUX remap register 7"]
pub mod remap7;
#[doc = "REMAP8 (rw) register accessor: IO MUX remap register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap8`]
module"]
#[doc(alias = "REMAP8")]
pub type Remap8 = crate::Reg<remap8::Remap8Spec>;
#[doc = "IO MUX remap register 8"]
pub mod remap8;
