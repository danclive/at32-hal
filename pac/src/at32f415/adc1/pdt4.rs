#[doc = "Register `PDT4` reader"]
pub type R = crate::R<Pdt4Spec>;
#[doc = "Field `PDT4` reader - Preempted data"]
pub type Pdt4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Preempted data"]
    #[inline(always)]
    pub fn pdt4(&self) -> Pdt4R {
        Pdt4R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDT4").field("pdt4", &self.pdt4()).finish()
    }
}
#[doc = "Preempted data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdt4Spec;
impl crate::RegisterSpec for Pdt4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdt4::R`](R) reader structure"]
impl crate::Readable for Pdt4Spec {}
#[doc = "`reset()` method sets PDT4 to value 0"]
impl crate::Resettable for Pdt4Spec {
    const RESET_VALUE: u32 = 0;
}
