#[doc = "Register `MMCTFSCC` reader"]
pub type R = crate::R<MmctfsccSpec>;
#[doc = "Field `TGFSCC` reader - Transmitted good frames single collision counter"]
pub type TgfsccR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames single collision counter"]
    #[inline(always)]
    pub fn tgfscc(&self) -> TgfsccR {
        TgfsccR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTFSCC")
            .field("tgfscc", &self.tgfscc())
            .finish()
    }
}
#[doc = "Ethernet MMC transmitted good frames after a single collision counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfscc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctfsccSpec;
impl crate::RegisterSpec for MmctfsccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctfscc::R`](R) reader structure"]
impl crate::Readable for MmctfsccSpec {}
#[doc = "`reset()` method sets MMCTFSCC to value 0"]
impl crate::Resettable for MmctfsccSpec {
    const RESET_VALUE: u32 = 0;
}
