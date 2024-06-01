#[doc = "Register `S8LLP` reader"]
pub type R = crate::R<S8llpSpec>;
#[doc = "Register `S8LLP` writer"]
pub type W = crate::W<S8llpSpec>;
#[doc = "Field `LLP` reader - Link list pointer"]
pub type LlpR = crate::FieldReader<u32>;
#[doc = "Field `LLP` writer - Link list pointer"]
pub type LlpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Link list pointer"]
    #[inline(always)]
    pub fn llp(&self) -> LlpR {
        LlpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S8LLP").field("llp", &self.llp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Link list pointer"]
    #[inline(always)]
    #[must_use]
    pub fn llp(&mut self) -> LlpW<S8llpSpec> {
        LlpW::new(self, 0)
    }
}
#[doc = "Stream 8 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8llp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8llp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S8llpSpec;
impl crate::RegisterSpec for S8llpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s8llp::R`](R) reader structure"]
impl crate::Readable for S8llpSpec {}
#[doc = "`write(|w| ..)` method takes [`s8llp::W`](W) writer structure"]
impl crate::Writable for S8llpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S8LLP to value 0"]
impl crate::Resettable for S8llpSpec {
    const RESET_VALUE: u32 = 0;
}
