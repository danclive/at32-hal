#[doc = "Register `CCTRL` reader"]
pub type R = crate::R<CctrlSpec>;
#[doc = "Register `CCTRL` writer"]
pub type W = crate::W<CctrlSpec>;
#[doc = "Field `C1EN` reader - Channel 1 enable"]
pub type C1enR = crate::BitReader;
#[doc = "Field `C1EN` writer - Channel 1 enable"]
pub type C1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1P` reader - Channel 1 Polarity"]
pub type C1pR = crate::BitReader;
#[doc = "Field `C1P` writer - Channel 1 Polarity"]
pub type C1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2EN` reader - Channel 2 enable"]
pub type C2enR = crate::BitReader;
#[doc = "Field `C2EN` writer - Channel 2 enable"]
pub type C2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2P` reader - Channel 2 Polarity"]
pub type C2pR = crate::BitReader;
#[doc = "Field `C2P` writer - Channel 2 Polarity"]
pub type C2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3EN` reader - Channel 3 enable"]
pub type C3enR = crate::BitReader;
#[doc = "Field `C3EN` writer - Channel 3 enable"]
pub type C3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3P` reader - Channel 3 Polarity"]
pub type C3pR = crate::BitReader;
#[doc = "Field `C3P` writer - Channel 3 Polarity"]
pub type C3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4EN` reader - Channel 4 enable"]
pub type C4enR = crate::BitReader;
#[doc = "Field `C4EN` writer - Channel 4 enable"]
pub type C4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4P` reader - Channel 4 Polarity"]
pub type C4pR = crate::BitReader;
#[doc = "Field `C4P` writer - Channel 4 Polarity"]
pub type C4pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    pub fn c1en(&self) -> C1enR {
        C1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn c1p(&self) -> C1pR {
        C1pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 enable"]
    #[inline(always)]
    pub fn c2en(&self) -> C2enR {
        C2enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn c2p(&self) -> C2pR {
        C2pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 enable"]
    #[inline(always)]
    pub fn c3en(&self) -> C3enR {
        C3enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn c3p(&self) -> C3pR {
        C3pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 enable"]
    #[inline(always)]
    pub fn c4en(&self) -> C4enR {
        C4enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn c4p(&self) -> C4pR {
        C4pR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCTRL")
            .field("c4p", &self.c4p())
            .field("c4en", &self.c4en())
            .field("c3p", &self.c3p())
            .field("c3en", &self.c3en())
            .field("c2p", &self.c2p())
            .field("c2en", &self.c2en())
            .field("c1p", &self.c1p())
            .field("c1en", &self.c1en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1en(&mut self) -> C1enW<CctrlSpec> {
        C1enW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c1p(&mut self) -> C1pW<CctrlSpec> {
        C1pW::new(self, 1)
    }
    #[doc = "Bit 4 - Channel 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2en(&mut self) -> C2enW<CctrlSpec> {
        C2enW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 2 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c2p(&mut self) -> C2pW<CctrlSpec> {
        C2pW::new(self, 5)
    }
    #[doc = "Bit 8 - Channel 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3en(&mut self) -> C3enW<CctrlSpec> {
        C3enW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 3 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c3p(&mut self) -> C3pW<CctrlSpec> {
        C3pW::new(self, 9)
    }
    #[doc = "Bit 12 - Channel 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4en(&mut self) -> C4enW<CctrlSpec> {
        C4enW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 4 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c4p(&mut self) -> C4pW<CctrlSpec> {
        C4pW::new(self, 13)
    }
}
#[doc = "Channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CctrlSpec;
impl crate::RegisterSpec for CctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cctrl::R`](R) reader structure"]
impl crate::Readable for CctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cctrl::W`](W) writer structure"]
impl crate::Writable for CctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCTRL to value 0"]
impl crate::Resettable for CctrlSpec {
    const RESET_VALUE: u32 = 0;
}
