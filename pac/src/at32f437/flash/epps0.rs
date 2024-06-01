#[doc = "Register `EPPS0` reader"]
pub type R = crate::R<Epps0Spec>;
#[doc = "Field `EPPS` reader - Erase/program protection status"]
pub type EppsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Erase/program protection status"]
    #[inline(always)]
    pub fn epps(&self) -> EppsR {
        EppsR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPPS0").field("epps", &self.epps()).finish()
    }
}
#[doc = "Erase/program protection status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Epps0Spec;
impl crate::RegisterSpec for Epps0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epps0::R`](R) reader structure"]
impl crate::Readable for Epps0Spec {}
#[doc = "`reset()` method sets EPPS0 to value 0xffff_ffff"]
impl crate::Resettable for Epps0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
