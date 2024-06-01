#[doc = "Register `D1DTH12R` reader"]
pub type R = crate::R<D1dth12rSpec>;
#[doc = "Register `D1DTH12R` writer"]
pub type W = crate::W<D1dth12rSpec>;
#[doc = "Field `D1DT12R` reader - DAC1 12-bit right-aligned data"]
pub type D1dt12rR = crate::FieldReader<u16>;
#[doc = "Field `D1DT12R` writer - DAC1 12-bit right-aligned data"]
pub type D1dt12rW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn d1dt12r(&self) -> D1dt12rR {
        D1dt12rR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1DTH12R")
            .field("d1dt12r", &self.d1dt12r())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn d1dt12r(&mut self) -> D1dt12rW<D1dth12rSpec> {
        D1dt12rW::new(self, 0)
    }
}
#[doc = "DAC1 12-bit right-aligned data holding register(DAC_D1DTH12R)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1dth12r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1dth12r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1dth12rSpec;
impl crate::RegisterSpec for D1dth12rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1dth12r::R`](R) reader structure"]
impl crate::Readable for D1dth12rSpec {}
#[doc = "`write(|w| ..)` method takes [`d1dth12r::W`](W) writer structure"]
impl crate::Writable for D1dth12rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D1DTH12R to value 0"]
impl crate::Resettable for D1dth12rSpec {
    const RESET_VALUE: u32 = 0;
}
