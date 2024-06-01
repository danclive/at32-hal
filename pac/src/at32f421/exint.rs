#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    inten: Inten,
    evten: Evten,
    polcfg1: Polcfg1,
    polcfg2: Polcfg2,
    swtrg: Swtrg,
    intsts: Intsts,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt enable register (EXTINT_INTEN)"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x04 - Event enable register (EXTINT_EVTEN)"]
    #[inline(always)]
    pub const fn evten(&self) -> &Evten {
        &self.evten
    }
    #[doc = "0x08 - Rising polarity configuration register(EXTINT_POLCFG1)"]
    #[inline(always)]
    pub const fn polcfg1(&self) -> &Polcfg1 {
        &self.polcfg1
    }
    #[doc = "0x0c - Falling polarity configuration register(EXTINT_POLCFG2)"]
    #[inline(always)]
    pub const fn polcfg2(&self) -> &Polcfg2 {
        &self.polcfg2
    }
    #[doc = "0x10 - Software triggle register (EXTINT_SWIE)"]
    #[inline(always)]
    pub const fn swtrg(&self) -> &Swtrg {
        &self.swtrg
    }
    #[doc = "0x14 - Interrupt status register (EXTINT_INTSTS)"]
    #[inline(always)]
    pub const fn intsts(&self) -> &Intsts {
        &self.intsts
    }
}
#[doc = "INTEN (rw) register accessor: Interrupt enable register (EXTINT_INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt enable register (EXTINT_INTEN)"]
pub mod inten;
#[doc = "EVTEN (rw) register accessor: Event enable register (EXTINT_EVTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evten`]
module"]
#[doc(alias = "EVTEN")]
pub type Evten = crate::Reg<evten::EvtenSpec>;
#[doc = "Event enable register (EXTINT_EVTEN)"]
pub mod evten;
#[doc = "POLCFG1 (rw) register accessor: Rising polarity configuration register(EXTINT_POLCFG1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polcfg1`]
module"]
#[doc(alias = "POLCFG1")]
pub type Polcfg1 = crate::Reg<polcfg1::Polcfg1Spec>;
#[doc = "Rising polarity configuration register(EXTINT_POLCFG1)"]
pub mod polcfg1;
#[doc = "POLCFG2 (rw) register accessor: Falling polarity configuration register(EXTINT_POLCFG2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polcfg2`]
module"]
#[doc(alias = "POLCFG2")]
pub type Polcfg2 = crate::Reg<polcfg2::Polcfg2Spec>;
#[doc = "Falling polarity configuration register(EXTINT_POLCFG2)"]
pub mod polcfg2;
#[doc = "SWTRG (rw) register accessor: Software triggle register (EXTINT_SWIE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrg`]
module"]
#[doc(alias = "SWTRG")]
pub type Swtrg = crate::Reg<swtrg::SwtrgSpec>;
#[doc = "Software triggle register (EXTINT_SWIE)"]
pub mod swtrg;
#[doc = "INTSTS (rw) register accessor: Interrupt status register (EXTINT_INTSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsts`]
module"]
#[doc(alias = "INTSTS")]
pub type Intsts = crate::Reg<intsts::IntstsSpec>;
#[doc = "Interrupt status register (EXTINT_INTSTS)"]
pub mod intsts;
