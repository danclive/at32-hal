#[doc = "Register `SWTRG` writer"]
pub type W = crate::W<SwtrgSpec>;
#[doc = "Field `D1SWTRG` writer - DAC1 software trigger"]
pub type D1swtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2SWTRG` writer - DAC2 software trigger"]
pub type D2swtrgW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SwtrgSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - DAC1 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn d1swtrg(&mut self) -> D1swtrgW<SwtrgSpec> {
        D1swtrgW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC2 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn d2swtrg(&mut self) -> D2swtrgW<SwtrgSpec> {
        D2swtrgW::new(self, 1)
    }
}
#[doc = "DAC software trigger register(DAC_SWTRIGR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrgSpec;
impl crate::RegisterSpec for SwtrgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swtrg::W`](W) writer structure"]
impl crate::Writable for SwtrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWTRG to value 0"]
impl crate::Resettable for SwtrgSpec {
    const RESET_VALUE: u32 = 0;
}
