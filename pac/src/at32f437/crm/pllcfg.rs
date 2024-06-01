#[doc = "Register `PLLCFG` reader"]
pub type R = crate::R<PllcfgSpec>;
#[doc = "Register `PLLCFG` writer"]
pub type W = crate::W<PllcfgSpec>;
#[doc = "Field `PLL_MS` reader - PLL pre-division"]
pub type PllMsR = crate::FieldReader;
#[doc = "Field `PLL_MS` writer - PLL pre-division"]
pub type PllMsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLL_NS` reader - PLL frequency multiplication factor"]
pub type PllNsR = crate::FieldReader<u16>;
#[doc = "Field `PLL_NS` writer - PLL frequency multiplication factor"]
pub type PllNsW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLL_FR` reader - PLL post-division"]
pub type PllFrR = crate::FieldReader;
#[doc = "Field `PLL_FR` writer - PLL post-division"]
pub type PllFrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLRCS` reader - PLL reference clock select"]
pub type PllrcsR = crate::BitReader;
#[doc = "Field `PLLRCS` writer - PLL reference clock select"]
pub type PllrcsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - PLL pre-division"]
    #[inline(always)]
    pub fn pll_ms(&self) -> PllMsR {
        PllMsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:14 - PLL frequency multiplication factor"]
    #[inline(always)]
    pub fn pll_ns(&self) -> PllNsR {
        PllNsR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:18 - PLL post-division"]
    #[inline(always)]
    pub fn pll_fr(&self) -> PllFrR {
        PllFrR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 22 - PLL reference clock select"]
    #[inline(always)]
    pub fn pllrcs(&self) -> PllrcsR {
        PllrcsR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFG")
            .field("pll_ms", &self.pll_ms())
            .field("pll_ns", &self.pll_ns())
            .field("pll_fr", &self.pll_fr())
            .field("pllrcs", &self.pllrcs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - PLL pre-division"]
    #[inline(always)]
    #[must_use]
    pub fn pll_ms(&mut self) -> PllMsW<PllcfgSpec> {
        PllMsW::new(self, 0)
    }
    #[doc = "Bits 6:14 - PLL frequency multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pll_ns(&mut self) -> PllNsW<PllcfgSpec> {
        PllNsW::new(self, 6)
    }
    #[doc = "Bits 16:18 - PLL post-division"]
    #[inline(always)]
    #[must_use]
    pub fn pll_fr(&mut self) -> PllFrW<PllcfgSpec> {
        PllFrW::new(self, 16)
    }
    #[doc = "Bit 22 - PLL reference clock select"]
    #[inline(always)]
    #[must_use]
    pub fn pllrcs(&mut self) -> PllrcsW<PllcfgSpec> {
        PllrcsW::new(self, 22)
    }
}
#[doc = "PLL configuration register (CRM_PLLCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcfgSpec;
impl crate::RegisterSpec for PllcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfg::R`](R) reader structure"]
impl crate::Readable for PllcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcfg::W`](W) writer structure"]
impl crate::Writable for PllcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCFG to value 0x0003_3002"]
impl crate::Resettable for PllcfgSpec {
    const RESET_VALUE: u32 = 0x0003_3002;
}
