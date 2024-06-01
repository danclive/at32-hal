#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CtrlstsSpec>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CtrlstsSpec>;
#[doc = "Field `LICKEN` reader - Low speed internal clock enable"]
pub type LickenR = crate::BitReader;
#[doc = "Field `LICKEN` writer - Low speed internal clock enable"]
pub type LickenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LICKSTBL` reader - Low speed internal clock ready"]
pub type LickstblR = crate::BitReader;
#[doc = "Field `RSTFC` reader - Reset flag clear"]
pub type RstfcR = crate::BitReader;
#[doc = "Field `RSTFC` writer - Reset flag clear"]
pub type RstfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRSTF` reader - PIN reset flag"]
pub type NrstfR = crate::BitReader;
#[doc = "Field `NRSTF` writer - PIN reset flag"]
pub type NrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORRSTF` reader - POR/LVR reset flag"]
pub type PorrstfR = crate::BitReader;
#[doc = "Field `PORRSTF` writer - POR/LVR reset flag"]
pub type PorrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub type SwrstfR = crate::BitReader;
#[doc = "Field `SWRSTF` writer - Software reset flag"]
pub type SwrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTRSTF` reader - Watchdog timer reset flag"]
pub type WdtrstfR = crate::BitReader;
#[doc = "Field `WDTRSTF` writer - Watchdog timer reset flag"]
pub type WdtrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTRSTF` reader - Window watchdog timer reset flag"]
pub type WwdtrstfR = crate::BitReader;
#[doc = "Field `WWDTRSTF` writer - Window watchdog timer reset flag"]
pub type WwdtrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPRSTF` reader - Low-power reset flag"]
pub type LprstfR = crate::BitReader;
#[doc = "Field `LPRSTF` writer - Low-power reset flag"]
pub type LprstfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low speed internal clock enable"]
    #[inline(always)]
    pub fn licken(&self) -> LickenR {
        LickenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low speed internal clock ready"]
    #[inline(always)]
    pub fn lickstbl(&self) -> LickstblR {
        LickstblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&self) -> RstfcR {
        RstfcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn nrstf(&self) -> NrstfR {
        NrstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/LVR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PorrstfR {
        PorrstfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&self) -> SwrstfR {
        SwrstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Watchdog timer reset flag"]
    #[inline(always)]
    pub fn wdtrstf(&self) -> WdtrstfR {
        WdtrstfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdtrstf(&self) -> WwdtrstfR {
        WwdtrstfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&self) -> LprstfR {
        LprstfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS")
            .field("licken", &self.licken())
            .field("lickstbl", &self.lickstbl())
            .field("rstfc", &self.rstfc())
            .field("nrstf", &self.nrstf())
            .field("porrstf", &self.porrstf())
            .field("swrstf", &self.swrstf())
            .field("wdtrstf", &self.wdtrstf())
            .field("wwdtrstf", &self.wwdtrstf())
            .field("lprstf", &self.lprstf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Low speed internal clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn licken(&mut self) -> LickenW<CtrlstsSpec> {
        LickenW::new(self, 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstfc(&mut self) -> RstfcW<CtrlstsSpec> {
        RstfcW::new(self, 24)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn nrstf(&mut self) -> NrstfW<CtrlstsSpec> {
        NrstfW::new(self, 26)
    }
    #[doc = "Bit 27 - POR/LVR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PorrstfW<CtrlstsSpec> {
        PorrstfW::new(self, 27)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn swrstf(&mut self) -> SwrstfW<CtrlstsSpec> {
        SwrstfW::new(self, 28)
    }
    #[doc = "Bit 29 - Watchdog timer reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrstf(&mut self) -> WdtrstfW<CtrlstsSpec> {
        WdtrstfW::new(self, 29)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wwdtrstf(&mut self) -> WwdtrstfW<CtrlstsSpec> {
        WwdtrstfW::new(self, 30)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn lprstf(&mut self) -> LprstfW<CtrlstsSpec> {
        LprstfW::new(self, 31)
    }
}
#[doc = "Control/status register (CRM_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlstsSpec;
impl crate::RegisterSpec for CtrlstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CtrlstsSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CtrlstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLSTS to value 0x0c00_0000"]
impl crate::Resettable for CtrlstsSpec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
