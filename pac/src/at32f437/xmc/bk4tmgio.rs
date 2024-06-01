#[doc = "Register `BK4TMGIO` reader"]
pub type R = crate::R<Bk4tmgioSpec>;
#[doc = "Register `BK4TMGIO` writer"]
pub type W = crate::W<Bk4tmgioSpec>;
#[doc = "Field `IOST` reader - STP"]
pub type IostR = crate::FieldReader;
#[doc = "Field `IOST` writer - STP"]
pub type IostW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IOWT` reader - OP"]
pub type IowtR = crate::FieldReader;
#[doc = "Field `IOWT` writer - OP"]
pub type IowtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IOHT` reader - HLD"]
pub type IohtR = crate::FieldReader;
#[doc = "Field `IOHT` writer - HLD"]
pub type IohtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IODHIZT` reader - WRSTP"]
pub type IodhiztR = crate::FieldReader;
#[doc = "Field `IODHIZT` writer - WRSTP"]
pub type IodhiztW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - STP"]
    #[inline(always)]
    pub fn iost(&self) -> IostR {
        IostR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OP"]
    #[inline(always)]
    pub fn iowt(&self) -> IowtR {
        IowtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - HLD"]
    #[inline(always)]
    pub fn ioht(&self) -> IohtR {
        IohtR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - WRSTP"]
    #[inline(always)]
    pub fn iodhizt(&self) -> IodhiztR {
        IodhiztR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK4TMGIO")
            .field("iodhizt", &self.iodhizt())
            .field("ioht", &self.ioht())
            .field("iowt", &self.iowt())
            .field("iost", &self.iost())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - STP"]
    #[inline(always)]
    #[must_use]
    pub fn iost(&mut self) -> IostW<Bk4tmgioSpec> {
        IostW::new(self, 0)
    }
    #[doc = "Bits 8:15 - OP"]
    #[inline(always)]
    #[must_use]
    pub fn iowt(&mut self) -> IowtW<Bk4tmgioSpec> {
        IowtW::new(self, 8)
    }
    #[doc = "Bits 16:23 - HLD"]
    #[inline(always)]
    #[must_use]
    pub fn ioht(&mut self) -> IohtW<Bk4tmgioSpec> {
        IohtW::new(self, 16)
    }
    #[doc = "Bits 24:31 - WRSTP"]
    #[inline(always)]
    #[must_use]
    pub fn iodhizt(&mut self) -> IodhiztW<Bk4tmgioSpec> {
        IodhiztW::new(self, 24)
    }
}
#[doc = "I/O space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk4tmgioSpec;
impl crate::RegisterSpec for Bk4tmgioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4tmgio::R`](R) reader structure"]
impl crate::Readable for Bk4tmgioSpec {}
#[doc = "`write(|w| ..)` method takes [`bk4tmgio::W`](W) writer structure"]
impl crate::Writable for Bk4tmgioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK4TMGIO to value 0xfcfc_fcfc"]
impl crate::Resettable for Bk4tmgioSpec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
