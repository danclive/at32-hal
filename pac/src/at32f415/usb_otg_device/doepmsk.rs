#[doc = "Register `DOEPMSK` reader"]
pub type R = crate::R<DoepmskSpec>;
#[doc = "Register `DOEPMSK` writer"]
pub type W = crate::W<DoepmskSpec>;
#[doc = "Field `XFERCMSK` reader - Transfer completed interrupt mask"]
pub type XfercmskR = crate::BitReader;
#[doc = "Field `XFERCMSK` writer - Transfer completed interrupt mask"]
pub type XfercmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDISMSK` reader - Endpoint disabled interrupt mask"]
pub type EptdismskR = crate::BitReader;
#[doc = "Field `EPTDISMSK` writer - Endpoint disabled interrupt mask"]
pub type EptdismskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPMSK` reader - SETUP phase done mask"]
pub type SetupmskR = crate::BitReader;
#[doc = "Field `SETUPMSK` writer - SETUP phase done mask"]
pub type SetupmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTEPDMSK` reader - OUT token received when endpoint disabled mask"]
pub type OuttepdmskR = crate::BitReader;
#[doc = "Field `OUTTEPDMSK` writer - OUT token received when endpoint disabled mask"]
pub type OuttepdmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSETUPMSK` reader - Back-to-back SETUP packets received mask"]
pub type B2bsetupmskR = crate::BitReader;
#[doc = "Field `B2BSETUPMSK` writer - Back-to-back SETUP packets received mask"]
pub type B2bsetupmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPERRMSK` reader - OUT packet error mask"]
pub type OutperrmskR = crate::BitReader;
#[doc = "Field `OUTPERRMSK` writer - OUT packet error mask"]
pub type OutperrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAOUTMSK` reader - BNA interrupt mask"]
pub type BnaoutmskR = crate::BitReader;
#[doc = "Field `BNAOUTMSK` writer - BNA interrupt mask"]
pub type BnaoutmskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfercmsk(&self) -> XfercmskR {
        XfercmskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn eptdismsk(&self) -> EptdismskR {
        EptdismskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    pub fn setupmsk(&self) -> SetupmskR {
        SetupmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    pub fn outtepdmsk(&self) -> OuttepdmskR {
        OuttepdmskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    pub fn b2bsetupmsk(&self) -> B2bsetupmskR {
        B2bsetupmskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn outperrmsk(&self) -> OutperrmskR {
        OutperrmskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bnaoutmsk(&self) -> BnaoutmskR {
        BnaoutmskR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPMSK")
            .field("xfercmsk", &self.xfercmsk())
            .field("eptdismsk", &self.eptdismsk())
            .field("setupmsk", &self.setupmsk())
            .field("outtepdmsk", &self.outtepdmsk())
            .field("b2bsetupmsk", &self.b2bsetupmsk())
            .field("outperrmsk", &self.outperrmsk())
            .field("bnaoutmsk", &self.bnaoutmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercmsk(&mut self) -> XfercmskW<DoepmskSpec> {
        XfercmskW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn eptdismsk(&mut self) -> EptdismskW<DoepmskSpec> {
        EptdismskW::new(self, 1)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    #[must_use]
    pub fn setupmsk(&mut self) -> SetupmskW<DoepmskSpec> {
        SetupmskW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    #[must_use]
    pub fn outtepdmsk(&mut self) -> OuttepdmskW<DoepmskSpec> {
        OuttepdmskW::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    #[must_use]
    pub fn b2bsetupmsk(&mut self) -> B2bsetupmskW<DoepmskSpec> {
        B2bsetupmskW::new(self, 6)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    #[must_use]
    pub fn outperrmsk(&mut self) -> OutperrmskW<DoepmskSpec> {
        OutperrmskW::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bnaoutmsk(&mut self) -> BnaoutmskW<DoepmskSpec> {
        BnaoutmskW::new(self, 9)
    }
}
#[doc = "OTGFS device OUT endpoint common interrupt mask register (OTGFS_DOEPMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoepmskSpec;
impl crate::RegisterSpec for DoepmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepmsk::R`](R) reader structure"]
impl crate::Readable for DoepmskSpec {}
#[doc = "`write(|w| ..)` method takes [`doepmsk::W`](W) writer structure"]
impl crate::Writable for DoepmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DoepmskSpec {
    const RESET_VALUE: u32 = 0;
}
