#[doc = "Register `C4DT` reader"]
pub type R = crate::R<C4dtSpec>;
#[doc = "Register `C4DT` writer"]
pub type W = crate::W<C4dtSpec>;
#[doc = "Field `C4DT` reader - Channel 4 data register"]
pub type C4dtR = crate::FieldReader<u32>;
#[doc = "Field `C4DT` writer - Channel 4 data register"]
pub type C4dtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 4 data register"]
    #[inline(always)]
    pub fn c4dt(&self) -> C4dtR {
        C4dtR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C4DT").field("c4dt", &self.c4dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 4 data register"]
    #[inline(always)]
    #[must_use]
    pub fn c4dt(&mut self) -> C4dtW<C4dtSpec> {
        C4dtW::new(self, 0)
    }
}
#[doc = "Channel 4 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C4dtSpec;
impl crate::RegisterSpec for C4dtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c4dt::R`](R) reader structure"]
impl crate::Readable for C4dtSpec {}
#[doc = "`write(|w| ..)` method takes [`c4dt::W`](W) writer structure"]
impl crate::Writable for C4dtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C4DT to value 0"]
impl crate::Resettable for C4dtSpec {
    const RESET_VALUE: u32 = 0;
}
