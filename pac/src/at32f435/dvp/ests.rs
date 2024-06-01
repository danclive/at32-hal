#[doc = "Register `ESTS` reader"]
pub type R = crate::R<EstsSpec>;
#[doc = "Field `CFDES` reader - Capture frame done event status"]
pub type CfdesR = crate::BitReader;
#[doc = "Field `OVRES` reader - Data FIFO overrun event status"]
pub type OvresR = crate::BitReader;
#[doc = "Field `ESEES` reader - Embedded synchronization error event status"]
pub type EseesR = crate::BitReader;
#[doc = "Field `VSES` reader - Vertical synchronization event status"]
pub type VsesR = crate::BitReader;
#[doc = "Field `HSES` reader - Horizontal synchronization event status"]
pub type HsesR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Capture frame done event status"]
    #[inline(always)]
    pub fn cfdes(&self) -> CfdesR {
        CfdesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FIFO overrun event status"]
    #[inline(always)]
    pub fn ovres(&self) -> OvresR {
        OvresR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded synchronization error event status"]
    #[inline(always)]
    pub fn esees(&self) -> EseesR {
        EseesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical synchronization event status"]
    #[inline(always)]
    pub fn vses(&self) -> VsesR {
        VsesR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Horizontal synchronization event status"]
    #[inline(always)]
    pub fn hses(&self) -> HsesR {
        HsesR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESTS")
            .field("hses", &self.hses())
            .field("vses", &self.vses())
            .field("esees", &self.esees())
            .field("ovres", &self.ovres())
            .field("cfdes", &self.cfdes())
            .finish()
    }
}
#[doc = "Event status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ests::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EstsSpec;
impl crate::RegisterSpec for EstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ests::R`](R) reader structure"]
impl crate::Readable for EstsSpec {}
#[doc = "`reset()` method sets ESTS to value 0"]
impl crate::Resettable for EstsSpec {
    const RESET_VALUE: u32 = 0;
}
