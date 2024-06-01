#[doc = "Register `CRC_CHKR` reader"]
pub type R = crate::R<CrcChkrSpec>;
#[doc = "Field `CRC_CHKR` reader - CRC check result"]
pub type CrcChkrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CRC check result"]
    #[inline(always)]
    pub fn crc_chkr(&self) -> CrcChkrR {
        CrcChkrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_CHKR")
            .field("crc_chkr", &self.crc_chkr())
            .finish()
    }
}
#[doc = "FLASH CRC check result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_chkr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcChkrSpec;
impl crate::RegisterSpec for CrcChkrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_chkr::R`](R) reader structure"]
impl crate::Readable for CrcChkrSpec {}
#[doc = "`reset()` method sets CRC_CHKR to value 0"]
impl crate::Resettable for CrcChkrSpec {
    const RESET_VALUE: u32 = 0;
}
