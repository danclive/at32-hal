#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `DMAUDR1` reader - DAC1 DMA underrun flag"]
pub type Dmaudr1R = crate::BitReader;
#[doc = "Field `DMAUDR1` writer - DAC1 DMA underrun flag"]
pub type Dmaudr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDR2` reader - DAC2 DMA underrun flag"]
pub type Dmaudr2R = crate::BitReader;
#[doc = "Field `DMAUDR2` writer - DAC2 DMA underrun flag"]
pub type Dmaudr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - DAC1 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr1(&self) -> Dmaudr1R {
        Dmaudr1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC2 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr2(&self) -> Dmaudr2R {
        Dmaudr2R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("dmaudr1", &self.dmaudr1())
            .field("dmaudr2", &self.dmaudr2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 13 - DAC1 DMA underrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> Dmaudr1W<StsSpec> {
        Dmaudr1W::new(self, 13)
    }
    #[doc = "Bit 29 - DAC2 DMA underrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr2(&mut self) -> Dmaudr2W<StsSpec> {
        Dmaudr2W::new(self, 29)
    }
}
#[doc = "DAC2 status register (DAC_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
