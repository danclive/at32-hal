#[doc = "Register `DT14` reader"]
pub type R = crate::R<Dt14Spec>;
#[doc = "Register `DT14` writer"]
pub type W = crate::W<Dt14Spec>;
#[doc = "Field `DT14` reader - BPR data14"]
pub type Dt14R = crate::FieldReader<u16>;
#[doc = "Field `DT14` writer - BPR data14"]
pub type Dt14W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data14"]
    #[inline(always)]
    pub fn dt14(&self) -> Dt14R {
        Dt14R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT14").field("dt14", &self.dt14()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data14"]
    #[inline(always)]
    #[must_use]
    pub fn dt14(&mut self) -> Dt14W<Dt14Spec> {
        Dt14W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt14Spec;
impl crate::RegisterSpec for Dt14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt14::R`](R) reader structure"]
impl crate::Readable for Dt14Spec {}
#[doc = "`write(|w| ..)` method takes [`dt14::W`](W) writer structure"]
impl crate::Writable for Dt14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT14 to value 0"]
impl crate::Resettable for Dt14Spec {
    const RESET_VALUE: u32 = 0;
}
