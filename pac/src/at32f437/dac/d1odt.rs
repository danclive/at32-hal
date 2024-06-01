#[doc = "Register `D1ODT` reader"]
pub type R = crate::R<D1odtSpec>;
#[doc = "Field `D1ODT` reader - DAC1 data output"]
pub type D1odtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 data output"]
    #[inline(always)]
    pub fn d1odt(&self) -> D1odtR {
        D1odtR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1ODT")
            .field("d1odt", &self.d1odt())
            .finish()
    }
}
#[doc = "DAC1 data output register (DAC_D1ODT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1odt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1odtSpec;
impl crate::RegisterSpec for D1odtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1odt::R`](R) reader structure"]
impl crate::Readable for D1odtSpec {}
#[doc = "`reset()` method sets D1ODT to value 0"]
impl crate::Resettable for D1odtSpec {
    const RESET_VALUE: u32 = 0;
}
