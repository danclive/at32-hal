#[doc = "Register `EPPS1` reader"]
pub type R = crate::R<Epps1Spec>;
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
        f.debug_struct("EPPS1").field("epps", &self.epps()).finish()
    }
}
#[doc = "Erase/program protection status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Epps1Spec;
impl crate::RegisterSpec for Epps1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epps1::R`](R) reader structure"]
impl crate::Readable for Epps1Spec {}
#[doc = "`reset()` method sets EPPS1 to value 0xffff_ffff"]
impl crate::Resettable for Epps1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
