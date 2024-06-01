#[doc = "Register `PTPTSH` reader"]
pub type R = crate::R<PtptshSpec>;
#[doc = "Field `TS` reader - Timestamp second"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp second"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSH").field("ts", &self.ts()).finish()
    }
}
#[doc = "Ethernet PTP time stamp high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptshSpec;
impl crate::RegisterSpec for PtptshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptsh::R`](R) reader structure"]
impl crate::Readable for PtptshSpec {}
#[doc = "`reset()` method sets PTPTSH to value 0"]
impl crate::Resettable for PtptshSpec {
    const RESET_VALUE: u32 = 0;
}
