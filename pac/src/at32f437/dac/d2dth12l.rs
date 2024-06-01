#[doc = "Register `D2DTH12L` reader"]
pub type R = crate::R<D2dth12lSpec>;
#[doc = "Register `D2DTH12L` writer"]
pub type W = crate::W<D2dth12lSpec>;
#[doc = "Field `D2DT12L` reader - DAC2 12-bit left-aligned data"]
pub type D2dt12lR = crate::FieldReader<u16>;
#[doc = "Field `D2DT12L` writer - DAC2 12-bit left-aligned data"]
pub type D2dt12lW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC2 12-bit left-aligned data"]
    #[inline(always)]
    pub fn d2dt12l(&self) -> D2dt12lR {
        D2dt12lR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D2DTH12L")
            .field("d2dt12l", &self.d2dt12l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC2 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn d2dt12l(&mut self) -> D2dt12lW<D2dth12lSpec> {
        D2dt12lW::new(self, 4)
    }
}
#[doc = "DAC2 12-bit left aligned data holding register (DAC_D2DTH12L)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2dth12l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2dth12l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2dth12lSpec;
impl crate::RegisterSpec for D2dth12lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2dth12l::R`](R) reader structure"]
impl crate::Readable for D2dth12lSpec {}
#[doc = "`write(|w| ..)` method takes [`d2dth12l::W`](W) writer structure"]
impl crate::Writable for D2dth12lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D2DTH12L to value 0"]
impl crate::Resettable for D2dth12lSpec {
    const RESET_VALUE: u32 = 0;
}
