#[doc = "Register `DT39` reader"]
pub type R = crate::R<Dt39Spec>;
#[doc = "Register `DT39` writer"]
pub type W = crate::W<Dt39Spec>;
#[doc = "Field `DT39` reader - BPR data39"]
pub type Dt39R = crate::FieldReader<u16>;
#[doc = "Field `DT39` writer - BPR data39"]
pub type Dt39W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data39"]
    #[inline(always)]
    pub fn dt39(&self) -> Dt39R {
        Dt39R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT39").field("dt39", &self.dt39()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data39"]
    #[inline(always)]
    #[must_use]
    pub fn dt39(&mut self) -> Dt39W<Dt39Spec> {
        Dt39W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt39Spec;
impl crate::RegisterSpec for Dt39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt39::R`](R) reader structure"]
impl crate::Readable for Dt39Spec {}
#[doc = "`write(|w| ..)` method takes [`dt39::W`](W) writer structure"]
impl crate::Writable for Dt39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT39 to value 0"]
impl crate::Resettable for Dt39Spec {
    const RESET_VALUE: u32 = 0;
}
