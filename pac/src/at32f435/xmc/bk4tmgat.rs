#[doc = "Register `BK4TMGAT` reader"]
pub type R = crate::R<Bk4tmgatSpec>;
#[doc = "Register `BK4TMGAT` writer"]
pub type W = crate::W<Bk4tmgatSpec>;
#[doc = "Field `ATST` reader - special memory setup time"]
pub type AtstR = crate::FieldReader;
#[doc = "Field `ATST` writer - special memory setup time"]
pub type AtstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATWT` reader - special memory wait time"]
pub type AtwtR = crate::FieldReader;
#[doc = "Field `ATWT` writer - special memory wait time"]
pub type AtwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATHT` reader - special memory hold time"]
pub type AthtR = crate::FieldReader;
#[doc = "Field `ATHT` writer - special memory hold time"]
pub type AthtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATDHIZT` reader - special memory databus High resistance time"]
pub type AtdhiztR = crate::FieldReader;
#[doc = "Field `ATDHIZT` writer - special memory databus High resistance time"]
pub type AtdhiztW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    pub fn atst(&self) -> AtstR {
        AtstR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    pub fn atwt(&self) -> AtwtR {
        AtwtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    pub fn atht(&self) -> AthtR {
        AthtR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    pub fn atdhizt(&self) -> AtdhiztR {
        AtdhiztR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK4TMGAT")
            .field("atdhizt", &self.atdhizt())
            .field("atht", &self.atht())
            .field("atwt", &self.atwt())
            .field("atst", &self.atst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn atst(&mut self) -> AtstW<Bk4tmgatSpec> {
        AtstW::new(self, 0)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn atwt(&mut self) -> AtwtW<Bk4tmgatSpec> {
        AtwtW::new(self, 8)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn atht(&mut self) -> AthtW<Bk4tmgatSpec> {
        AthtW::new(self, 16)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    #[must_use]
    pub fn atdhizt(&mut self) -> AtdhiztW<Bk4tmgatSpec> {
        AtdhiztW::new(self, 24)
    }
}
#[doc = "special memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk4tmgatSpec;
impl crate::RegisterSpec for Bk4tmgatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4tmgat::R`](R) reader structure"]
impl crate::Readable for Bk4tmgatSpec {}
#[doc = "`write(|w| ..)` method takes [`bk4tmgat::W`](W) writer structure"]
impl crate::Writable for Bk4tmgatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK4TMGAT to value 0xfcfc_fcfc"]
impl crate::Resettable for Bk4tmgatSpec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
