#[doc = "Register `BK2TMGSP` reader"]
pub type R = crate::R<Bk2tmgspSpec>;
#[doc = "Register `BK2TMGSP` writer"]
pub type W = crate::W<Bk2tmgspSpec>;
#[doc = "Field `SPST` reader - special memory setup time"]
pub type SpstR = crate::FieldReader;
#[doc = "Field `SPST` writer - special memory setup time"]
pub type SpstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPWT` reader - special memory wait time"]
pub type SpwtR = crate::FieldReader;
#[doc = "Field `SPWT` writer - special memory wait time"]
pub type SpwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPHT` reader - special memory hold time"]
pub type SphtR = crate::FieldReader;
#[doc = "Field `SPHT` writer - special memory hold time"]
pub type SphtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPDHIZT` reader - special memory databus High resistance time"]
pub type SpdhiztR = crate::FieldReader;
#[doc = "Field `SPDHIZT` writer - special memory databus High resistance time"]
pub type SpdhiztW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    pub fn spst(&self) -> SpstR {
        SpstR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    pub fn spwt(&self) -> SpwtR {
        SpwtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    pub fn spht(&self) -> SphtR {
        SphtR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    pub fn spdhizt(&self) -> SpdhiztR {
        SpdhiztR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK2TMGSP")
            .field("spdhizt", &self.spdhizt())
            .field("spht", &self.spht())
            .field("spwt", &self.spwt())
            .field("spst", &self.spst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn spst(&mut self) -> SpstW<Bk2tmgspSpec> {
        SpstW::new(self, 0)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn spwt(&mut self) -> SpwtW<Bk2tmgspSpec> {
        SpwtW::new(self, 8)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn spht(&mut self) -> SphtW<Bk2tmgspSpec> {
        SphtW::new(self, 16)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    #[must_use]
    pub fn spdhizt(&mut self) -> SpdhiztW<Bk2tmgspSpec> {
        SpdhiztW::new(self, 24)
    }
}
#[doc = "special memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2tmgsp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2tmgsp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk2tmgspSpec;
impl crate::RegisterSpec for Bk2tmgspSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk2tmgsp::R`](R) reader structure"]
impl crate::Readable for Bk2tmgspSpec {}
#[doc = "`write(|w| ..)` method takes [`bk2tmgsp::W`](W) writer structure"]
impl crate::Writable for Bk2tmgspSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK2TMGSP to value 0xfcfc_fcfc"]
impl crate::Resettable for Bk2tmgspSpec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
