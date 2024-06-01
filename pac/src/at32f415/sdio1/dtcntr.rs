#[doc = "Register `DTCNTR` reader"]
pub type R = crate::R<DtcntrSpec>;
#[doc = "Field `CNT` reader - Data count value"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCNTR").field("cnt", &self.cnt()).finish()
    }
}
#[doc = "Bits 24:0 = DATACOUNT: Data count value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcntr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtcntrSpec;
impl crate::RegisterSpec for DtcntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtcntr::R`](R) reader structure"]
impl crate::Readable for DtcntrSpec {}
#[doc = "`reset()` method sets DTCNTR to value 0"]
impl crate::Resettable for DtcntrSpec {
    const RESET_VALUE: u32 = 0;
}
