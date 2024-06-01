#[doc = "Register `TSSBS` reader"]
pub type R = crate::R<TssbsSpec>;
#[doc = "Field `SBS` reader - Sub second value"]
pub type SbsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn sbs(&self) -> SbsR {
        SbsR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSSBS").field("sbs", &self.sbs()).finish()
    }
}
#[doc = "timestamp sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tssbs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TssbsSpec;
impl crate::RegisterSpec for TssbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tssbs::R`](R) reader structure"]
impl crate::Readable for TssbsSpec {}
#[doc = "`reset()` method sets TSSBS to value 0"]
impl crate::Resettable for TssbsSpec {
    const RESET_VALUE: u32 = 0;
}
