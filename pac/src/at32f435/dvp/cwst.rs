#[doc = "Register `CWST` reader"]
pub type R = crate::R<CwstSpec>;
#[doc = "Register `CWST` writer"]
pub type W = crate::W<CwstSpec>;
#[doc = "Field `CHSTR` reader - Cropping window horizontal start pixel"]
pub type ChstrR = crate::FieldReader<u16>;
#[doc = "Field `CHSTR` writer - Cropping window horizontal start pixel"]
pub type ChstrW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `CVSTR` reader - Cropping window vertical start line"]
pub type CvstrR = crate::FieldReader<u16>;
#[doc = "Field `CVSTR` writer - Cropping window vertical start line"]
pub type CvstrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:13 - Cropping window horizontal start pixel"]
    #[inline(always)]
    pub fn chstr(&self) -> ChstrR {
        ChstrR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - Cropping window vertical start line"]
    #[inline(always)]
    pub fn cvstr(&self) -> CvstrR {
        CvstrR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CWST")
            .field("cvstr", &self.cvstr())
            .field("chstr", &self.chstr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Cropping window horizontal start pixel"]
    #[inline(always)]
    #[must_use]
    pub fn chstr(&mut self) -> ChstrW<CwstSpec> {
        ChstrW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Cropping window vertical start line"]
    #[inline(always)]
    #[must_use]
    pub fn cvstr(&mut self) -> CvstrW<CwstSpec> {
        CvstrW::new(self, 16)
    }
}
#[doc = "Crop window start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwstSpec;
impl crate::RegisterSpec for CwstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwst::R`](R) reader structure"]
impl crate::Readable for CwstSpec {}
#[doc = "`write(|w| ..)` method takes [`cwst::W`](W) writer structure"]
impl crate::Writable for CwstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWST to value 0"]
impl crate::Resettable for CwstSpec {
    const RESET_VALUE: u32 = 0;
}
