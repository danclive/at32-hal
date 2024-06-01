#[doc = "Register `C1DT` reader"]
pub type R = crate::R<C1dtSpec>;
#[doc = "Register `C1DT` writer"]
pub type W = crate::W<C1dtSpec>;
#[doc = "Field `C1DT` reader - Channel 1 data register"]
pub type C1dtR = crate::FieldReader<u32>;
#[doc = "Field `C1DT` writer - Channel 1 data register"]
pub type C1dtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 1 data register"]
    #[inline(always)]
    pub fn c1dt(&self) -> C1dtR {
        C1dtR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1DT").field("c1dt", &self.c1dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 1 data register"]
    #[inline(always)]
    #[must_use]
    pub fn c1dt(&mut self) -> C1dtW<C1dtSpec> {
        C1dtW::new(self, 0)
    }
}
#[doc = "Channel 1 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1dtSpec;
impl crate::RegisterSpec for C1dtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1dt::R`](R) reader structure"]
impl crate::Readable for C1dtSpec {}
#[doc = "`write(|w| ..)` method takes [`c1dt::W`](W) writer structure"]
impl crate::Writable for C1dtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1DT to value 0"]
impl crate::Resettable for C1dtSpec {
    const RESET_VALUE: u32 = 0;
}
