#[doc = "Register `RSTS` reader"]
pub type R = crate::R<RstsSpec>;
#[doc = "Field `SPISTS` reader - SPI read status"]
pub type SpistsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SPI read status"]
    #[inline(always)]
    pub fn spists(&self) -> SpistsR {
        SpistsR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTS")
            .field("spists", &self.spists())
            .finish()
    }
}
#[doc = "SPI read status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstsSpec;
impl crate::RegisterSpec for RstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsts::R`](R) reader structure"]
impl crate::Readable for RstsSpec {}
#[doc = "`reset()` method sets RSTS to value 0"]
impl crate::Resettable for RstsSpec {
    const RESET_VALUE: u32 = 0;
}
