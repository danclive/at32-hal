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
#[doc = "Field `C1CEN` reader - Channel 1 complementary enable"]
pub type C1cenR = crate::BitReader;
#[doc = "Field `C1CEN` writer - Channel 1 complementary enable"]
pub type C1cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1CP` reader - Channel 1 complementary polarity"]
pub type C1cpR = crate::BitReader;
#[doc = "Field `C1CP` writer - Channel 1 complementary polarity"]
pub type C1cpW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Channel 1 complementary enable"]
    #[inline(always)]
    pub fn c1cen(&self) -> C1cenR {
        C1cenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 complementary polarity"]
    #[inline(always)]
    pub fn c1cp(&self) -> C1cpR {
        C1cpR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCTRL")
            .field("c1cp", &self.c1cp())
            .field("c1cen", &self.c1cen())
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
    #[doc = "Bit 2 - Channel 1 complementary enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1cen(&mut self) -> C1cenW<CctrlSpec> {
        C1cenW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 1 complementary polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c1cp(&mut self) -> C1cpW<CctrlSpec> {
        C1cpW::new(self, 3)
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
