#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `CFDIC` writer - Capture frame done interrupt clear"]
pub type CfdicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIC` writer - Data FIFO overrun interrupt clear"]
pub type OvricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESEIC` writer - Embedded synchronization error interrupt clear"]
pub type EseicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSIC` writer - Vertical synchronization interrupt clear"]
pub type VsicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIC` writer - Horizontal synchronization interrupt clear"]
pub type HsicW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Capture frame done interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cfdic(&mut self) -> CfdicW<IclrSpec> {
        CfdicW::new(self, 0)
    }
    #[doc = "Bit 1 - Data FIFO overrun interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovric(&mut self) -> OvricW<IclrSpec> {
        OvricW::new(self, 1)
    }
    #[doc = "Bit 2 - Embedded synchronization error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn eseic(&mut self) -> EseicW<IclrSpec> {
        EseicW::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical synchronization interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn vsic(&mut self) -> VsicW<IclrSpec> {
        VsicW::new(self, 3)
    }
    #[doc = "Bit 4 - Horizontal synchronization interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsic(&mut self) -> HsicW<IclrSpec> {
        HsicW::new(self, 4)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {
    const RESET_VALUE: u32 = 0;
}
