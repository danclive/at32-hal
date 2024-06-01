#[doc = "Register `PDT3` reader"]
pub type R = crate::R<Pdt3Spec>;
#[doc = "Field `PDT3` reader - Preempted data"]
pub type Pdt3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Preempted data"]
    #[inline(always)]
    pub fn pdt3(&self) -> Pdt3R {
        Pdt3R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDT3").field("pdt3", &self.pdt3()).finish()
    }
}
#[doc = "Preempted data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdt3Spec;
impl crate::RegisterSpec for Pdt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdt3::R`](R) reader structure"]
impl crate::Readable for Pdt3Spec {}
#[doc = "`reset()` method sets PDT3 to value 0"]
impl crate::Resettable for Pdt3Spec {
    const RESET_VALUE: u32 = 0;
}
