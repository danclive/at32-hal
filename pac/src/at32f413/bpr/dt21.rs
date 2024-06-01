#[doc = "Register `DT21` reader"]
pub type R = crate::R<Dt21Spec>;
#[doc = "Register `DT21` writer"]
pub type W = crate::W<Dt21Spec>;
#[doc = "Field `DT21` reader - BPR data21"]
pub type Dt21R = crate::FieldReader<u16>;
#[doc = "Field `DT21` writer - BPR data21"]
pub type Dt21W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data21"]
    #[inline(always)]
    pub fn dt21(&self) -> Dt21R {
        Dt21R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT21").field("dt21", &self.dt21()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data21"]
    #[inline(always)]
    #[must_use]
    pub fn dt21(&mut self) -> Dt21W<Dt21Spec> {
        Dt21W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt21Spec;
impl crate::RegisterSpec for Dt21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt21::R`](R) reader structure"]
impl crate::Readable for Dt21Spec {}
#[doc = "`write(|w| ..)` method takes [`dt21::W`](W) writer structure"]
impl crate::Writable for Dt21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT21 to value 0"]
impl crate::Resettable for Dt21Spec {
    const RESET_VALUE: u32 = 0;
}
