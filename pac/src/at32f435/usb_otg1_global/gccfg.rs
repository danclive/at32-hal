#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GccfgSpec>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GccfgSpec>;
#[doc = "Field `PWRDOWN` reader - Power down"]
pub type PwrdownR = crate::BitReader;
#[doc = "Field `PWRDOWN` writer - Power down"]
pub type PwrdownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_MODE` reader - Low power mode"]
pub type LpModeR = crate::BitReader;
#[doc = "Field `LP_MODE` writer - Low power mode"]
pub type LpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFOUTEN` reader - SOF output enable"]
pub type SofoutenR = crate::BitReader;
#[doc = "Field `SOFOUTEN` writer - SOF output enable"]
pub type SofoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSIG` reader - VBUS Ignored"]
pub type VbusigR = crate::BitReader;
#[doc = "Field `VBUSIG` writer - VBUS Ignored"]
pub type VbusigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdown(&self) -> PwrdownR {
        PwrdownR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Low power mode"]
    #[inline(always)]
    pub fn lp_mode(&self) -> LpModeR {
        LpModeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofouten(&self) -> SofoutenR {
        SofoutenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBUS Ignored"]
    #[inline(always)]
    pub fn vbusig(&self) -> VbusigR {
        VbusigR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCCFG")
            .field("pwrdown", &self.pwrdown())
            .field("lp_mode", &self.lp_mode())
            .field("sofouten", &self.sofouten())
            .field("vbusig", &self.vbusig())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdown(&mut self) -> PwrdownW<GccfgSpec> {
        PwrdownW::new(self, 16)
    }
    #[doc = "Bit 17 - Low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lp_mode(&mut self) -> LpModeW<GccfgSpec> {
        LpModeW::new(self, 17)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofouten(&mut self) -> SofoutenW<GccfgSpec> {
        SofoutenW::new(self, 20)
    }
    #[doc = "Bit 21 - VBUS Ignored"]
    #[inline(always)]
    #[must_use]
    pub fn vbusig(&mut self) -> VbusigW<GccfgSpec> {
        VbusigW::new(self, 21)
    }
}
#[doc = "OTGFS general core configuration register (OTGFS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GccfgSpec;
impl crate::RegisterSpec for GccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccfg::R`](R) reader structure"]
impl crate::Readable for GccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gccfg::W`](W) writer structure"]
impl crate::Writable for GccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GccfgSpec {
    const RESET_VALUE: u32 = 0;
}
