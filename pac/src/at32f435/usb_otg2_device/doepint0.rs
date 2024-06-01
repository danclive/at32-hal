#[doc = "Register `DOEPINT0` reader"]
pub type R = crate::R<Doepint0Spec>;
#[doc = "Register `DOEPINT0` writer"]
pub type W = crate::W<Doepint0Spec>;
#[doc = "Field `XFERC` reader - Transfer completed interrupt"]
pub type XfercR = crate::BitReader;
#[doc = "Field `XFERC` writer - Transfer completed interrupt"]
pub type XfercW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDISD` reader - Endpoint disabled interrupt"]
pub type EptdisdR = crate::BitReader;
#[doc = "Field `EPTDISD` writer - Endpoint disabled interrupt"]
pub type EptdisdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP` reader - SETUP phase done"]
pub type SetupR = crate::BitReader;
#[doc = "Field `SETUP` writer - SETUP phase done"]
pub type SetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTEPD` reader - OUT token received when endpoint disabled"]
pub type OuttepdR = crate::BitReader;
#[doc = "Field `OUTTEPD` writer - OUT token received when endpoint disabled"]
pub type OuttepdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received"]
pub type B2bstupR = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received"]
pub type B2bstupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xferc(&self) -> XfercR {
        XfercR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn eptdisd(&self) -> EptdisdR {
        EptdisdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn outtepd(&self) -> OuttepdR {
        OuttepdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2bstupR {
        B2bstupR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT0")
            .field("xferc", &self.xferc())
            .field("eptdisd", &self.eptdisd())
            .field("setup", &self.setup())
            .field("outtepd", &self.outtepd())
            .field("b2bstup", &self.b2bstup())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xferc(&mut self) -> XfercW<Doepint0Spec> {
        XfercW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eptdisd(&mut self) -> EptdisdW<Doepint0Spec> {
        EptdisdW::new(self, 1)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SetupW<Doepint0Spec> {
        SetupW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    #[must_use]
    pub fn outtepd(&mut self) -> OuttepdW<Doepint0Spec> {
        OuttepdW::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    #[must_use]
    pub fn b2bstup(&mut self) -> B2bstupW<Doepint0Spec> {
        B2bstupW::new(self, 6)
    }
}
#[doc = "OTGFS device OUT endpoint-0 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepint0Spec;
impl crate::RegisterSpec for Doepint0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint0::R`](R) reader structure"]
impl crate::Readable for Doepint0Spec {}
#[doc = "`write(|w| ..)` method takes [`doepint0::W`](W) writer structure"]
impl crate::Writable for Doepint0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT0 to value 0x80"]
impl crate::Resettable for Doepint0Spec {
    const RESET_VALUE: u32 = 0x80;
}
