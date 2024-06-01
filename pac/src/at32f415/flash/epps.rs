#[doc = "Register `EPPS` reader"]
pub type R = crate::R<EppsSpec>;
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
        f.debug_struct("EPPS").field("epps", &self.epps()).finish()
    }
}
#[doc = "Erase/program protection status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EppsSpec;
impl crate::RegisterSpec for EppsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epps::R`](R) reader structure"]
impl crate::Readable for EppsSpec {}
#[doc = "`reset()` method sets EPPS to value 0xffff_ffff"]
impl crate::Resettable for EppsSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
