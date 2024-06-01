#[doc = "Register `BK4IS` reader"]
pub type R = crate::R<Bk4isSpec>;
#[doc = "Register `BK4IS` writer"]
pub type W = crate::W<Bk4isSpec>;
#[doc = "Field `RES` reader - Rising edge capture status"]
pub type ResR = crate::BitReader;
#[doc = "Field `RES` writer - Rising edge capture status"]
pub type ResW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLS` reader - High-level status"]
pub type HlsR = crate::BitReader;
#[doc = "Field `HLS` writer - High-level status"]
pub type HlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Falling edge status"]
pub type FesR = crate::BitReader;
#[doc = "Field `FES` writer - Falling edge status"]
pub type FesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REIEN` reader - Rising edge interrupt enable"]
pub type ReienR = crate::BitReader;
#[doc = "Field `REIEN` writer - Rising edge interrupt enable"]
pub type ReienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLIEN` reader - High-level interrupt enable"]
pub type HlienR = crate::BitReader;
#[doc = "Field `HLIEN` writer - High-level interrupt enable"]
pub type HlienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIEN` reader - Falling edge interrupt enable"]
pub type FeienR = crate::BitReader;
#[doc = "Field `FEIEN` writer - Falling edge interrupt enable"]
pub type FeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOE` reader - FIFO empty"]
pub type FifoeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Rising edge capture status"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High-level status"]
    #[inline(always)]
    pub fn hls(&self) -> HlsR {
        HlsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge status"]
    #[inline(always)]
    pub fn fes(&self) -> FesR {
        FesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising edge interrupt enable"]
    #[inline(always)]
    pub fn reien(&self) -> ReienR {
        ReienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High-level interrupt enable"]
    #[inline(always)]
    pub fn hlien(&self) -> HlienR {
        HlienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling edge interrupt enable"]
    #[inline(always)]
    pub fn feien(&self) -> FeienR {
        FeienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO empty"]
    #[inline(always)]
    pub fn fifoe(&self) -> FifoeR {
        FifoeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK4IS")
            .field("fifoe", &self.fifoe())
            .field("feien", &self.feien())
            .field("hlien", &self.hlien())
            .field("reien", &self.reien())
            .field("fes", &self.fes())
            .field("hls", &self.hls())
            .field("res", &self.res())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge capture status"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<Bk4isSpec> {
        ResW::new(self, 0)
    }
    #[doc = "Bit 1 - High-level status"]
    #[inline(always)]
    #[must_use]
    pub fn hls(&mut self) -> HlsW<Bk4isSpec> {
        HlsW::new(self, 1)
    }
    #[doc = "Bit 2 - Falling edge status"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FesW<Bk4isSpec> {
        FesW::new(self, 2)
    }
    #[doc = "Bit 3 - Rising edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn reien(&mut self) -> ReienW<Bk4isSpec> {
        ReienW::new(self, 3)
    }
    #[doc = "Bit 4 - High-level interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hlien(&mut self) -> HlienW<Bk4isSpec> {
        HlienW::new(self, 4)
    }
    #[doc = "Bit 5 - Falling edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn feien(&mut self) -> FeienW<Bk4isSpec> {
        FeienW::new(self, 5)
    }
}
#[doc = "FIFO status and interrupt register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4is::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4is::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk4isSpec;
impl crate::RegisterSpec for Bk4isSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4is::R`](R) reader structure"]
impl crate::Readable for Bk4isSpec {}
#[doc = "`write(|w| ..)` method takes [`bk4is::W`](W) writer structure"]
impl crate::Writable for Bk4isSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK4IS to value 0x40"]
impl crate::Resettable for Bk4isSpec {
    const RESET_VALUE: u32 = 0x40;
}
