#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `FMSC` reader - Frame start code"]
pub type FmscR = crate::FieldReader;
#[doc = "Field `FMSC` writer - Frame start code"]
pub type FmscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LNSC` reader - Line start code"]
pub type LnscR = crate::FieldReader;
#[doc = "Field `LNSC` writer - Line start code"]
pub type LnscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LNEC` reader - Line end code"]
pub type LnecR = crate::FieldReader;
#[doc = "Field `LNEC` writer - Line end code"]
pub type LnecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FMEC` reader - Frame end code"]
pub type FmecR = crate::FieldReader;
#[doc = "Field `FMEC` writer - Frame end code"]
pub type FmecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame start code"]
    #[inline(always)]
    pub fn fmsc(&self) -> FmscR {
        FmscR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start code"]
    #[inline(always)]
    pub fn lnsc(&self) -> LnscR {
        LnscR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end code"]
    #[inline(always)]
    pub fn lnec(&self) -> LnecR {
        LnecR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end code"]
    #[inline(always)]
    pub fn fmec(&self) -> FmecR {
        FmecR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("fmec", &self.fmec())
            .field("lnec", &self.lnec())
            .field("lnsc", &self.lnsc())
            .field("fmsc", &self.fmsc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start code"]
    #[inline(always)]
    #[must_use]
    pub fn fmsc(&mut self) -> FmscW<ScrSpec> {
        FmscW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Line start code"]
    #[inline(always)]
    #[must_use]
    pub fn lnsc(&mut self) -> LnscW<ScrSpec> {
        LnscW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Line end code"]
    #[inline(always)]
    #[must_use]
    pub fn lnec(&mut self) -> LnecW<ScrSpec> {
        LnecW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Frame end code"]
    #[inline(always)]
    #[must_use]
    pub fn fmec(&mut self) -> FmecW<ScrSpec> {
        FmecW::new(self, 24)
    }
}
#[doc = "Synchronization code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {
    const RESET_VALUE: u32 = 0;
}
