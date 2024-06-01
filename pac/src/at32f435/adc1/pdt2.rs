#[doc = "Register `PDT2` reader"]
pub type R = crate::R<Pdt2Spec>;
#[doc = "Field `PDT2` reader - Preempted data"]
pub type Pdt2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Preempted data"]
    #[inline(always)]
    pub fn pdt2(&self) -> Pdt2R {
        Pdt2R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDT2").field("pdt2", &self.pdt2()).finish()
    }
}
#[doc = "Preempted data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdt2Spec;
impl crate::RegisterSpec for Pdt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdt2::R`](R) reader structure"]
impl crate::Readable for Pdt2Spec {}
#[doc = "`reset()` method sets PDT2 to value 0"]
impl crate::Resettable for Pdt2Spec {
    const RESET_VALUE: u32 = 0;
}
