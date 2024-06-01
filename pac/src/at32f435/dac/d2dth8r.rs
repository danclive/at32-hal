#[doc = "Register `D2DTH8R` reader"]
pub type R = crate::R<D2dth8rSpec>;
#[doc = "Register `D2DTH8R` writer"]
pub type W = crate::W<D2dth8rSpec>;
#[doc = "Field `D2DT8R` reader - DAC2 8-bit right-aligned data"]
pub type D2dt8rR = crate::FieldReader;
#[doc = "Field `D2DT8R` writer - DAC2 8-bit right-aligned data"]
pub type D2dt8rW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn d2dt8r(&self) -> D2dt8rR {
        D2dt8rR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D2DTH8R")
            .field("d2dt8r", &self.d2dt8r())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC2 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn d2dt8r(&mut self) -> D2dt8rW<D2dth8rSpec> {
        D2dt8rW::new(self, 0)
    }
}
#[doc = "DAC2 8-bit right-aligned data holding register (DAC_D2DTH8R)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2dth8r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2dth8r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2dth8rSpec;
impl crate::RegisterSpec for D2dth8rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2dth8r::R`](R) reader structure"]
impl crate::Readable for D2dth8rSpec {}
#[doc = "`write(|w| ..)` method takes [`d2dth8r::W`](W) writer structure"]
impl crate::Writable for D2dth8rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D2DTH8R to value 0"]
impl crate::Resettable for D2dth8rSpec {
    const RESET_VALUE: u32 = 0;
}
