#[doc = "Register `D2ODT` reader"]
pub type R = crate::R<D2odtSpec>;
#[doc = "Field `D2ODT` reader - DAC2 data output"]
pub type D2odtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC2 data output"]
    #[inline(always)]
    pub fn d2odt(&self) -> D2odtR {
        D2odtR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D2ODT")
            .field("d2odt", &self.d2odt())
            .finish()
    }
}
#[doc = "DAC2 data output register (DAC_D2ODT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2odt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2odtSpec;
impl crate::RegisterSpec for D2odtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2odt::R`](R) reader structure"]
impl crate::Readable for D2odtSpec {}
#[doc = "`reset()` method sets D2ODT to value 0"]
impl crate::Resettable for D2odtSpec {
    const RESET_VALUE: u32 = 0;
}
