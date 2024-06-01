#[doc = "Register `MMCRFCECNT` reader"]
pub type R = crate::R<MmcrfcecntSpec>;
#[doc = "Field `RFCEC` reader - Received frames CRC error counter"]
pub type RfcecR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames CRC error counter"]
    #[inline(always)]
    pub fn rfcec(&self) -> RfcecR {
        RfcecR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRFCECNT")
            .field("rfcec", &self.rfcec())
            .finish()
    }
}
#[doc = "Ethernet MMC received frames with CRC error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfcecnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcrfcecntSpec;
impl crate::RegisterSpec for MmcrfcecntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrfcecnt::R`](R) reader structure"]
impl crate::Readable for MmcrfcecntSpec {}
#[doc = "`reset()` method sets MMCRFCECNT to value 0"]
impl crate::Resettable for MmcrfcecntSpec {
    const RESET_VALUE: u32 = 0;
}
