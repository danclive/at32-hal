#[doc = "Register `MMCCTRL` reader"]
pub type R = crate::R<MmcctrlSpec>;
#[doc = "Register `MMCCTRL` writer"]
pub type W = crate::W<MmcctrlSpec>;
#[doc = "Field `RC` reader - Reset counter"]
pub type RcR = crate::BitReader;
#[doc = "Field `RC` writer - Reset counter"]
pub type RcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCR` reader - Stop counter rollover"]
pub type ScrR = crate::BitReader;
#[doc = "Field `SCR` writer - Stop counter rollover"]
pub type ScrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RR` reader - Reset on read"]
pub type RrR = crate::BitReader;
#[doc = "Field `RR` writer - Reset on read"]
pub type RrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMC` reader - Freeze MMC counter"]
pub type FmcR = crate::BitReader;
#[doc = "Field `FMC` writer - Freeze MMC counter"]
pub type FmcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset counter"]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop counter rollover"]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn rr(&self) -> RrR {
        RrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - Freeze MMC counter"]
    #[inline(always)]
    pub fn fmc(&self) -> FmcR {
        FmcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCCTRL")
            .field("rc", &self.rc())
            .field("scr", &self.scr())
            .field("rr", &self.rr())
            .field("fmc", &self.fmc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reset counter"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RcW<MmcctrlSpec> {
        RcW::new(self, 0)
    }
    #[doc = "Bit 1 - Stop counter rollover"]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> ScrW<MmcctrlSpec> {
        ScrW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    #[must_use]
    pub fn rr(&mut self) -> RrW<MmcctrlSpec> {
        RrW::new(self, 2)
    }
    #[doc = "Bit 31 - Freeze MMC counter"]
    #[inline(always)]
    #[must_use]
    pub fn fmc(&mut self) -> FmcW<MmcctrlSpec> {
        FmcW::new(self, 31)
    }
}
#[doc = "Ethernet MMC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcctrlSpec;
impl crate::RegisterSpec for MmcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcctrl::R`](R) reader structure"]
impl crate::Readable for MmcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmcctrl::W`](W) writer structure"]
impl crate::Writable for MmcctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCCTRL to value 0"]
impl crate::Resettable for MmcctrlSpec {
    const RESET_VALUE: u32 = 0;
}
