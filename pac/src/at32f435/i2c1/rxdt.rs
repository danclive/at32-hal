#[doc = "Register `RXDT` reader"]
pub type R = crate::R<RxdtSpec>;
#[doc = "Field `DT` reader - Receive data register"]
pub type DtR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive data register"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDT").field("dt", &self.dt()).finish()
    }
}
#[doc = "Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdtSpec;
impl crate::RegisterSpec for RxdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdt::R`](R) reader structure"]
impl crate::Readable for RxdtSpec {}
#[doc = "`reset()` method sets RXDT to value 0"]
impl crate::Resettable for RxdtSpec {
    const RESET_VALUE: u32 = 0;
}
