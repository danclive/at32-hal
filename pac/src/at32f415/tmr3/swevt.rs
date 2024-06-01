#[doc = "Register `SWEVT` reader"]
pub type R = crate::R<SwevtSpec>;
#[doc = "Register `SWEVT` writer"]
pub type W = crate::W<SwevtSpec>;
#[doc = "Field `OVFSWTR` reader - Overflow event triggered by software"]
pub type OvfswtrR = crate::BitReader;
#[doc = "Field `OVFSWTR` writer - Overflow event triggered by software"]
pub type OvfswtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1SWTR` reader - Channel 1 event triggered by software"]
pub type C1swtrR = crate::BitReader;
#[doc = "Field `C1SWTR` writer - Channel 1 event triggered by software"]
pub type C1swtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2SWTR` reader - Channel 2 event triggered by software"]
pub type C2swtrR = crate::BitReader;
#[doc = "Field `C2SWTR` writer - Channel 2 event triggered by software"]
pub type C2swtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3SWTR` reader - Channel 3 event triggered by software"]
pub type C3swtrR = crate::BitReader;
#[doc = "Field `C3SWTR` writer - Channel 3 event triggered by software"]
pub type C3swtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4SWTR` reader - Channel 4 event triggered by software"]
pub type C4swtrR = crate::BitReader;
#[doc = "Field `C4SWTR` writer - Channel 4 event triggered by software"]
pub type C4swtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSWTR` reader - Trigger event triggered by software"]
pub type TrgswtrR = crate::BitReader;
#[doc = "Field `TRGSWTR` writer - Trigger event triggered by software"]
pub type TrgswtrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow event triggered by software"]
    #[inline(always)]
    pub fn ovfswtr(&self) -> OvfswtrR {
        OvfswtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 event triggered by software"]
    #[inline(always)]
    pub fn c1swtr(&self) -> C1swtrR {
        C1swtrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 event triggered by software"]
    #[inline(always)]
    pub fn c2swtr(&self) -> C2swtrR {
        C2swtrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 event triggered by software"]
    #[inline(always)]
    pub fn c3swtr(&self) -> C3swtrR {
        C3swtrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 event triggered by software"]
    #[inline(always)]
    pub fn c4swtr(&self) -> C4swtrR {
        C4swtrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    pub fn trgswtr(&self) -> TrgswtrR {
        TrgswtrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWEVT")
            .field("trgswtr", &self.trgswtr())
            .field("c4swtr", &self.c4swtr())
            .field("c3swtr", &self.c3swtr())
            .field("c2swtr", &self.c2swtr())
            .field("c1swtr", &self.c1swtr())
            .field("ovfswtr", &self.ovfswtr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn ovfswtr(&mut self) -> OvfswtrW<SwevtSpec> {
        OvfswtrW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c1swtr(&mut self) -> C1swtrW<SwevtSpec> {
        C1swtrW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c2swtr(&mut self) -> C2swtrW<SwevtSpec> {
        C2swtrW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c3swtr(&mut self) -> C3swtrW<SwevtSpec> {
        C3swtrW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c4swtr(&mut self) -> C4swtrW<SwevtSpec> {
        C4swtrW::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn trgswtr(&mut self) -> TrgswtrW<SwevtSpec> {
        TrgswtrW::new(self, 6)
    }
}
#[doc = "Software event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swevt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevtSpec;
impl crate::RegisterSpec for SwevtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swevt::R`](R) reader structure"]
impl crate::Readable for SwevtSpec {}
#[doc = "`write(|w| ..)` method takes [`swevt::W`](W) writer structure"]
impl crate::Writable for SwevtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SwevtSpec {
    const RESET_VALUE: u32 = 0;
}
