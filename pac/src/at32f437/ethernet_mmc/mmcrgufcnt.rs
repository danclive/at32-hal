#[doc = "Register `MMCRGUFCNT` reader"]
pub type R = crate::R<MmcrgufcntSpec>;
#[doc = "Field `RGUFC` reader - Received good unicast frames counter"]
pub type RgufcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received good unicast frames counter"]
    #[inline(always)]
    pub fn rgufc(&self) -> RgufcR {
        RgufcR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRGUFCNT")
            .field("rgufc", &self.rgufc())
            .finish()
    }
}
#[doc = "MMC received good unicast frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrgufcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcrgufcntSpec;
impl crate::RegisterSpec for MmcrgufcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrgufcnt::R`](R) reader structure"]
impl crate::Readable for MmcrgufcntSpec {}
#[doc = "`reset()` method sets MMCRGUFCNT to value 0"]
impl crate::Resettable for MmcrgufcntSpec {
    const RESET_VALUE: u32 = 0;
}
