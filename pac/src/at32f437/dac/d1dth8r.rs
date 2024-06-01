#[doc = "Register `D1DTH8R` reader"]
pub type R = crate::R<D1dth8rSpec>;
#[doc = "Register `D1DTH8R` writer"]
pub type W = crate::W<D1dth8rSpec>;
#[doc = "Field `D1DT8R` reader - DAC1 8-bit right-aligned data"]
pub type D1dt8rR = crate::FieldReader;
#[doc = "Field `D1DT8R` writer - DAC1 8-bit right-aligned data"]
pub type D1dt8rW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn d1dt8r(&self) -> D1dt8rR {
        D1dt8rR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1DTH8R")
            .field("d1dt8r", &self.d1dt8r())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn d1dt8r(&mut self) -> D1dt8rW<D1dth8rSpec> {
        D1dt8rW::new(self, 0)
    }
}
#[doc = "DAC1 8-bit right aligned data holding register (DAC_D1DTH8R)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1dth8r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1dth8r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1dth8rSpec;
impl crate::RegisterSpec for D1dth8rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1dth8r::R`](R) reader structure"]
impl crate::Readable for D1dth8rSpec {}
#[doc = "`write(|w| ..)` method takes [`d1dth8r::W`](W) writer structure"]
impl crate::Writable for D1dth8rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D1DTH8R to value 0"]
impl crate::Resettable for D1dth8rSpec {
    const RESET_VALUE: u32 = 0;
}
