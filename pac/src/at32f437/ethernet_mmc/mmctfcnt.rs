#[doc = "Register `MMCTFCNT` reader"]
pub type R = crate::R<MmctfcntSpec>;
#[doc = "Field `TGFC` reader - Transmitted good frames counter"]
pub type TgfcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames counter"]
    #[inline(always)]
    pub fn tgfc(&self) -> TgfcR {
        TgfcR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTFCNT")
            .field("tgfc", &self.tgfc())
            .finish()
    }
}
#[doc = "Ethernet MMC transmitted good frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctfcntSpec;
impl crate::RegisterSpec for MmctfcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctfcnt::R`](R) reader structure"]
impl crate::Readable for MmctfcntSpec {}
#[doc = "`reset()` method sets MMCTFCNT to value 0"]
impl crate::Resettable for MmctfcntSpec {
    const RESET_VALUE: u32 = 0;
}
