#[doc = "Register `BK4TMGCM` reader"]
pub type R = crate::R<Bk4tmgcmSpec>;
#[doc = "Register `BK4TMGCM` writer"]
pub type W = crate::W<Bk4tmgcmSpec>;
#[doc = "Field `CMST` reader - Regular memory setup time"]
pub type CmstR = crate::FieldReader;
#[doc = "Field `CMST` writer - Regular memory setup time"]
pub type CmstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMWT` reader - Regular memory wait time"]
pub type CmwtR = crate::FieldReader;
#[doc = "Field `CMWT` writer - Regular memory wait time"]
pub type CmwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMHT` reader - Regular memory hold time"]
pub type CmhtR = crate::FieldReader;
#[doc = "Field `CMHT` writer - Regular memory hold time"]
pub type CmhtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMDHIZT` reader - Regular memory databus High resistance time"]
pub type CmdhiztR = crate::FieldReader;
#[doc = "Field `CMDHIZT` writer - Regular memory databus High resistance time"]
pub type CmdhiztW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    pub fn cmst(&self) -> CmstR {
        CmstR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    pub fn cmwt(&self) -> CmwtR {
        CmwtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    pub fn cmht(&self) -> CmhtR {
        CmhtR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    pub fn cmdhizt(&self) -> CmdhiztR {
        CmdhiztR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK4TMGCM")
            .field("cmdhizt", &self.cmdhizt())
            .field("cmht", &self.cmht())
            .field("cmwt", &self.cmwt())
            .field("cmst", &self.cmst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn cmst(&mut self) -> CmstW<Bk4tmgcmSpec> {
        CmstW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn cmwt(&mut self) -> CmwtW<Bk4tmgcmSpec> {
        CmwtW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn cmht(&mut self) -> CmhtW<Bk4tmgcmSpec> {
        CmhtW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    #[must_use]
    pub fn cmdhizt(&mut self) -> CmdhiztW<Bk4tmgcmSpec> {
        CmdhiztW::new(self, 24)
    }
}
#[doc = "Regular memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgcm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgcm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk4tmgcmSpec;
impl crate::RegisterSpec for Bk4tmgcmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4tmgcm::R`](R) reader structure"]
impl crate::Readable for Bk4tmgcmSpec {}
#[doc = "`write(|w| ..)` method takes [`bk4tmgcm::W`](W) writer structure"]
impl crate::Writable for Bk4tmgcmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK4TMGCM to value 0xfcfc_fcfc"]
impl crate::Resettable for Bk4tmgcmSpec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
