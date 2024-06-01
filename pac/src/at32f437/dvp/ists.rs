#[doc = "Register `ISTS` reader"]
pub type R = crate::R<IstsSpec>;
#[doc = "Field `CFDIS` reader - Capture frame done interrupt status"]
pub type CfdisR = crate::BitReader;
#[doc = "Field `OVRIS` reader - Data FIFO overrun interrupt status"]
pub type OvrisR = crate::BitReader;
#[doc = "Field `ESEIS` reader - Embedded synchronization error interrupt status"]
pub type EseisR = crate::BitReader;
#[doc = "Field `VSIS` reader - Vertical synchronization interrupt status"]
pub type VsisR = crate::BitReader;
#[doc = "Field `HSIS` reader - Horizontal synchronization interrupt status"]
pub type HsisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Capture frame done interrupt status"]
    #[inline(always)]
    pub fn cfdis(&self) -> CfdisR {
        CfdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FIFO overrun interrupt status"]
    #[inline(always)]
    pub fn ovris(&self) -> OvrisR {
        OvrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded synchronization error interrupt status"]
    #[inline(always)]
    pub fn eseis(&self) -> EseisR {
        EseisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical synchronization interrupt status"]
    #[inline(always)]
    pub fn vsis(&self) -> VsisR {
        VsisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Horizontal synchronization interrupt status"]
    #[inline(always)]
    pub fn hsis(&self) -> HsisR {
        HsisR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTS")
            .field("hsis", &self.hsis())
            .field("vsis", &self.vsis())
            .field("eseis", &self.eseis())
            .field("ovris", &self.ovris())
            .field("cfdis", &self.cfdis())
            .finish()
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ists::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstsSpec;
impl crate::RegisterSpec for IstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ists::R`](R) reader structure"]
impl crate::Readable for IstsSpec {}
#[doc = "`reset()` method sets ISTS to value 0"]
impl crate::Resettable for IstsSpec {
    const RESET_VALUE: u32 = 0;
}
