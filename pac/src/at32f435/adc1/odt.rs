#[doc = "Register `ODT` reader"]
pub type R = crate::R<OdtSpec>;
#[doc = "Field `ODT` reader - Conversion data of ordinary channel"]
pub type OdtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Conversion data of ordinary channel"]
    #[inline(always)]
    pub fn odt(&self) -> OdtR {
        OdtR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODT").field("odt", &self.odt()).finish()
    }
}
#[doc = "Ordinary data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdtSpec;
impl crate::RegisterSpec for OdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odt::R`](R) reader structure"]
impl crate::Readable for OdtSpec {}
#[doc = "`reset()` method sets ODT to value 0"]
impl crate::Resettable for OdtSpec {
    const RESET_VALUE: u32 = 0;
}
