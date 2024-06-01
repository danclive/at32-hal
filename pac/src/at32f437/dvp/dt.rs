#[doc = "Register `DT` reader"]
pub type R = crate::R<DtSpec>;
#[doc = "Field `DT` reader - Data Port"]
pub type DtR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Port"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT").field("dt", &self.dt()).finish()
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtSpec;
impl crate::RegisterSpec for DtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt::R`](R) reader structure"]
impl crate::Readable for DtSpec {}
#[doc = "`reset()` method sets DT to value 0"]
impl crate::Resettable for DtSpec {
    const RESET_VALUE: u32 = 0;
}
