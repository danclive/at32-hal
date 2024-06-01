#[doc = "Register `IDCODE` reader"]
pub type R = crate::R<IdcodeSpec>;
#[doc = "Field `PID` reader - Product ID"]
pub type PidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Product ID"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDCODE").field("pid", &self.pid()).finish()
    }
}
#[doc = "DEBUG IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdcodeSpec;
impl crate::RegisterSpec for IdcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcode::R`](R) reader structure"]
impl crate::Readable for IdcodeSpec {}
#[doc = "`reset()` method sets IDCODE to value 0"]
impl crate::Resettable for IdcodeSpec {
    const RESET_VALUE: u32 = 0;
}
