#[doc = "Register `PDT1` reader"]
pub type R = crate::R<Pdt1Spec>;
#[doc = "Field `PDT1` reader - Preempted data"]
pub type Pdt1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Preempted data"]
    #[inline(always)]
    pub fn pdt1(&self) -> Pdt1R {
        Pdt1R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDT1").field("pdt1", &self.pdt1()).finish()
    }
}
#[doc = "Preempted data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdt1Spec;
impl crate::RegisterSpec for Pdt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdt1::R`](R) reader structure"]
impl crate::Readable for Pdt1Spec {}
#[doc = "`reset()` method sets PDT1 to value 0"]
impl crate::Resettable for Pdt1Spec {
    const RESET_VALUE: u32 = 0;
}
