#[doc = "Register `SUR` reader"]
pub type R = crate::R<SurSpec>;
#[doc = "Register `SUR` writer"]
pub type W = crate::W<SurSpec>;
#[doc = "Field `FMSU` reader - Frame start unmask"]
pub type FmsuR = crate::FieldReader;
#[doc = "Field `FMSU` writer - Frame start unmask"]
pub type FmsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LNSU` reader - Line start unmask"]
pub type LnsuR = crate::FieldReader;
#[doc = "Field `LNSU` writer - Line start unmask"]
pub type LnsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LNEU` reader - Line end unmask"]
pub type LneuR = crate::FieldReader;
#[doc = "Field `LNEU` writer - Line end unmask"]
pub type LneuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FMEU` reader - Frame end unmask"]
pub type FmeuR = crate::FieldReader;
#[doc = "Field `FMEU` writer - Frame end unmask"]
pub type FmeuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame start unmask"]
    #[inline(always)]
    pub fn fmsu(&self) -> FmsuR {
        FmsuR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start unmask"]
    #[inline(always)]
    pub fn lnsu(&self) -> LnsuR {
        LnsuR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end unmask"]
    #[inline(always)]
    pub fn lneu(&self) -> LneuR {
        LneuR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end unmask"]
    #[inline(always)]
    pub fn fmeu(&self) -> FmeuR {
        FmeuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUR")
            .field("fmeu", &self.fmeu())
            .field("lneu", &self.lneu())
            .field("lnsu", &self.lnsu())
            .field("fmsu", &self.fmsu())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start unmask"]
    #[inline(always)]
    #[must_use]
    pub fn fmsu(&mut self) -> FmsuW<SurSpec> {
        FmsuW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Line start unmask"]
    #[inline(always)]
    #[must_use]
    pub fn lnsu(&mut self) -> LnsuW<SurSpec> {
        LnsuW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Line end unmask"]
    #[inline(always)]
    #[must_use]
    pub fn lneu(&mut self) -> LneuW<SurSpec> {
        LneuW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Frame end unmask"]
    #[inline(always)]
    #[must_use]
    pub fn fmeu(&mut self) -> FmeuW<SurSpec> {
        FmeuW::new(self, 24)
    }
}
#[doc = "Synchronization unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SurSpec;
impl crate::RegisterSpec for SurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sur::R`](R) reader structure"]
impl crate::Readable for SurSpec {}
#[doc = "`write(|w| ..)` method takes [`sur::W`](W) writer structure"]
impl crate::Writable for SurSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUR to value 0"]
impl crate::Resettable for SurSpec {
    const RESET_VALUE: u32 = 0;
}
