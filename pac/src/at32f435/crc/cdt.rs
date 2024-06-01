#[doc = "Register `CDT` reader"]
pub type R = crate::R<CdtSpec>;
#[doc = "Register `CDT` writer"]
pub type W = crate::W<CdtSpec>;
#[doc = "Field `CDT` reader - Common Data"]
pub type CdtR = crate::BitReader;
#[doc = "Field `CDT` writer - Common Data"]
pub type CdtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Common Data"]
    #[inline(always)]
    pub fn cdt(&self) -> CdtR {
        CdtR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDT").field("cdt", &self.cdt()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Common Data"]
    #[inline(always)]
    #[must_use]
    pub fn cdt(&mut self) -> CdtW<CdtSpec> {
        CdtW::new(self, 0)
    }
}
#[doc = "Common data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdtSpec;
impl crate::RegisterSpec for CdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdt::R`](R) reader structure"]
impl crate::Readable for CdtSpec {}
#[doc = "`write(|w| ..)` method takes [`cdt::W`](W) writer structure"]
impl crate::Writable for CdtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDT to value 0"]
impl crate::Resettable for CdtSpec {
    const RESET_VALUE: u32 = 0;
}
