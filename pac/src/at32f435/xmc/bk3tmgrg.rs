#[doc = "Register `BK3TMGRG` reader"]
pub type R = crate::R<Bk3tmgrgSpec>;
#[doc = "Register `BK3TMGRG` writer"]
pub type W = crate::W<Bk3tmgrgSpec>;
#[doc = "Field `RGST` reader - Regular memory setup time"]
pub type RgstR = crate::FieldReader;
#[doc = "Field `RGST` writer - Regular memory setup time"]
pub type RgstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGWT` reader - Regular memory wait time"]
pub type RgwtR = crate::FieldReader;
#[doc = "Field `RGWT` writer - Regular memory wait time"]
pub type RgwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGHT` reader - Regular memory hold time"]
pub type RghtR = crate::FieldReader;
#[doc = "Field `RGHT` writer - Regular memory hold time"]
pub type RghtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGDHIZT` reader - Regular memory databus High resistance time"]
pub type RgdhiztR = crate::FieldReader;
#[doc = "Field `RGDHIZT` writer - Regular memory databus High resistance time"]
pub type RgdhiztW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    pub fn rgst(&self) -> RgstR {
        RgstR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    pub fn rgwt(&self) -> RgwtR {
        RgwtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    pub fn rght(&self) -> RghtR {
        RghtR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    pub fn rgdhizt(&self) -> RgdhiztR {
        RgdhiztR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK3TMGRG")
            .field("rgdhizt", &self.rgdhizt())
            .field("rght", &self.rght())
            .field("rgwt", &self.rgwt())
            .field("rgst", &self.rgst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn rgst(&mut self) -> RgstW<Bk3tmgrgSpec> {
        RgstW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn rgwt(&mut self) -> RgwtW<Bk3tmgrgSpec> {
        RgwtW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn rght(&mut self) -> RghtW<Bk3tmgrgSpec> {
        RghtW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    #[must_use]
    pub fn rgdhizt(&mut self) -> RgdhiztW<Bk3tmgrgSpec> {
        RgdhiztW::new(self, 24)
    }
}
#[doc = "Regular memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3tmgrg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3tmgrg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk3tmgrgSpec;
impl crate::RegisterSpec for Bk3tmgrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk3tmgrg::R`](R) reader structure"]
impl crate::Readable for Bk3tmgrgSpec {}
#[doc = "`write(|w| ..)` method takes [`bk3tmgrg::W`](W) writer structure"]
impl crate::Writable for Bk3tmgrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK3TMGRG to value 0xfcfc_fcfc"]
impl crate::Resettable for Bk3tmgrgSpec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
