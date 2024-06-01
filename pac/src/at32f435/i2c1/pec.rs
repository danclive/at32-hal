#[doc = "Register `PEC` reader"]
pub type R = crate::R<PecSpec>;
#[doc = "Field `PECVAL` reader - PEC value"]
pub type PecvalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PEC value"]
    #[inline(always)]
    pub fn pecval(&self) -> PecvalR {
        PecvalR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEC")
            .field("pecval", &self.pecval())
            .finish()
    }
}
#[doc = "PEC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PecSpec;
impl crate::RegisterSpec for PecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pec::R`](R) reader structure"]
impl crate::Readable for PecSpec {}
#[doc = "`reset()` method sets PEC to value 0"]
impl crate::Resettable for PecSpec {
    const RESET_VALUE: u32 = 0;
}
