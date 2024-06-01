#[doc = "Register `DT11` reader"]
pub type R = crate::R<Dt11Spec>;
#[doc = "Register `DT11` writer"]
pub type W = crate::W<Dt11Spec>;
#[doc = "Field `DT11` reader - BPR data11"]
pub type Dt11R = crate::FieldReader<u16>;
#[doc = "Field `DT11` writer - BPR data11"]
pub type Dt11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data11"]
    #[inline(always)]
    pub fn dt11(&self) -> Dt11R {
        Dt11R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT11").field("dt11", &self.dt11()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data11"]
    #[inline(always)]
    #[must_use]
    pub fn dt11(&mut self) -> Dt11W<Dt11Spec> {
        Dt11W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt11Spec;
impl crate::RegisterSpec for Dt11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt11::R`](R) reader structure"]
impl crate::Readable for Dt11Spec {}
#[doc = "`write(|w| ..)` method takes [`dt11::W`](W) writer structure"]
impl crate::Writable for Dt11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT11 to value 0"]
impl crate::Resettable for Dt11Spec {
    const RESET_VALUE: u32 = 0;
}
