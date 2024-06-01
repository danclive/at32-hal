#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SLEEP_DEBUG` reader - SLEEP_DEBUG"]
pub type SleepDebugR = crate::BitReader;
#[doc = "Field `SLEEP_DEBUG` writer - SLEEP_DEBUG"]
pub type SleepDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPSLEEP_DEBUG` reader - DEEPSLEEP_DEBUG"]
pub type DeepsleepDebugR = crate::BitReader;
#[doc = "Field `DEEPSLEEP_DEBUG` writer - DEEPSLEEP_DEBUG"]
pub type DeepsleepDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STANDBY_DEBUG` reader - STANDBY_DEBUG"]
pub type StandbyDebugR = crate::BitReader;
#[doc = "Field `STANDBY_DEBUG` writer - STANDBY_DEBUG"]
pub type StandbyDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_PAUSE` reader - WDT_PAUSE"]
pub type WdtPauseR = crate::BitReader;
#[doc = "Field `WDT_PAUSE` writer - WDT_PAUSE"]
pub type WdtPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT_PAUSE` reader - WWDT_PAUSE"]
pub type WwdtPauseR = crate::BitReader;
#[doc = "Field `WWDT_PAUSE` writer - WWDT_PAUSE"]
pub type WwdtPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1_PAUSE` reader - TMR1_PAUSE"]
pub type Tmr1PauseR = crate::BitReader;
#[doc = "Field `TMR1_PAUSE` writer - TMR1_PAUSE"]
pub type Tmr1PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3_PAUSE` reader - TMR3_PAUSE"]
pub type Tmr3PauseR = crate::BitReader;
#[doc = "Field `TMR3_PAUSE` writer - TMR3_PAUSE"]
pub type Tmr3PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERTC_PAUSE` reader - ERTC_PAUSE"]
pub type ErtcPauseR = crate::BitReader;
#[doc = "Field `ERTC_PAUSE` writer - ERTC_PAUSE"]
pub type ErtcPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` reader - I2C1_SMBUS_TIMEOUT"]
pub type I2c1SmbusTimeoutR = crate::BitReader;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` writer - I2C1_SMBUS_TIMEOUT"]
pub type I2c1SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` reader - I2C2_SMBUS_TIMEOUT"]
pub type I2c2SmbusTimeoutR = crate::BitReader;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` writer - I2C2_SMBUS_TIMEOUT"]
pub type I2c2SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6_PAUSE` reader - TMR6_PAUSE"]
pub type Tmr6PauseR = crate::BitReader;
#[doc = "Field `TMR6_PAUSE` writer - TMR6_PAUSE"]
pub type Tmr6PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERTC_512_PAUSE` reader - ERTC_512_PAUSE"]
pub type Ertc512PauseR = crate::BitReader;
#[doc = "Field `ERTC_512_PAUSE` writer - ERTC_512_PAUSE"]
pub type Ertc512PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR15_PAUSE` reader - TMR15_PAUSE"]
pub type Tmr15PauseR = crate::BitReader;
#[doc = "Field `TMR15_PAUSE` writer - TMR15_PAUSE"]
pub type Tmr15PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR16_PAUSE` reader - TMR16_PAUSE"]
pub type Tmr16PauseR = crate::BitReader;
#[doc = "Field `TMR16_PAUSE` writer - TMR16_PAUSE"]
pub type Tmr16PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR17_PAUSE` reader - TMR17_PAUSE"]
pub type Tmr17PauseR = crate::BitReader;
#[doc = "Field `TMR17_PAUSE` writer - TMR17_PAUSE"]
pub type Tmr17PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14_PAUSE` reader - TMR14_PAUSE"]
pub type Tmr14PauseR = crate::BitReader;
#[doc = "Field `TMR14_PAUSE` writer - TMR14_PAUSE"]
pub type Tmr14PauseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SLEEP_DEBUG"]
    #[inline(always)]
    pub fn sleep_debug(&self) -> SleepDebugR {
        SleepDebugR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DEEPSLEEP_DEBUG"]
    #[inline(always)]
    pub fn deepsleep_debug(&self) -> DeepsleepDebugR {
        DeepsleepDebugR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STANDBY_DEBUG"]
    #[inline(always)]
    pub fn standby_debug(&self) -> StandbyDebugR {
        StandbyDebugR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT_PAUSE"]
    #[inline(always)]
    pub fn wdt_pause(&self) -> WdtPauseR {
        WdtPauseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WWDT_PAUSE"]
    #[inline(always)]
    pub fn wwdt_pause(&self) -> WwdtPauseR {
        WwdtPauseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TMR1_PAUSE"]
    #[inline(always)]
    pub fn tmr1_pause(&self) -> Tmr1PauseR {
        Tmr1PauseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - TMR3_PAUSE"]
    #[inline(always)]
    pub fn tmr3_pause(&self) -> Tmr3PauseR {
        Tmr3PauseR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - ERTC_PAUSE"]
    #[inline(always)]
    pub fn ertc_pause(&self) -> ErtcPauseR {
        ErtcPauseR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&self) -> I2c1SmbusTimeoutR {
        I2c1SmbusTimeoutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&self) -> I2c2SmbusTimeoutR {
        I2c2SmbusTimeoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - TMR6_PAUSE"]
    #[inline(always)]
    pub fn tmr6_pause(&self) -> Tmr6PauseR {
        Tmr6PauseR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - ERTC_512_PAUSE"]
    #[inline(always)]
    pub fn ertc_512_pause(&self) -> Ertc512PauseR {
        Ertc512PauseR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TMR15_PAUSE"]
    #[inline(always)]
    pub fn tmr15_pause(&self) -> Tmr15PauseR {
        Tmr15PauseR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TMR16_PAUSE"]
    #[inline(always)]
    pub fn tmr16_pause(&self) -> Tmr16PauseR {
        Tmr16PauseR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TMR17_PAUSE"]
    #[inline(always)]
    pub fn tmr17_pause(&self) -> Tmr17PauseR {
        Tmr17PauseR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - TMR14_PAUSE"]
    #[inline(always)]
    pub fn tmr14_pause(&self) -> Tmr14PauseR {
        Tmr14PauseR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("sleep_debug", &self.sleep_debug())
            .field("deepsleep_debug", &self.deepsleep_debug())
            .field("standby_debug", &self.standby_debug())
            .field("wdt_pause", &self.wdt_pause())
            .field("wwdt_pause", &self.wwdt_pause())
            .field("tmr1_pause", &self.tmr1_pause())
            .field("tmr3_pause", &self.tmr3_pause())
            .field("ertc_pause", &self.ertc_pause())
            .field("i2c1_smbus_timeout", &self.i2c1_smbus_timeout())
            .field("i2c2_smbus_timeout", &self.i2c2_smbus_timeout())
            .field("tmr6_pause", &self.tmr6_pause())
            .field("ertc_512_pause", &self.ertc_512_pause())
            .field("tmr15_pause", &self.tmr15_pause())
            .field("tmr16_pause", &self.tmr16_pause())
            .field("tmr17_pause", &self.tmr17_pause())
            .field("tmr14_pause", &self.tmr14_pause())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SLEEP_DEBUG"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_debug(&mut self) -> SleepDebugW<CtrlSpec> {
        SleepDebugW::new(self, 0)
    }
    #[doc = "Bit 1 - DEEPSLEEP_DEBUG"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep_debug(&mut self) -> DeepsleepDebugW<CtrlSpec> {
        DeepsleepDebugW::new(self, 1)
    }
    #[doc = "Bit 2 - STANDBY_DEBUG"]
    #[inline(always)]
    #[must_use]
    pub fn standby_debug(&mut self) -> StandbyDebugW<CtrlSpec> {
        StandbyDebugW::new(self, 2)
    }
    #[doc = "Bit 8 - WDT_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_pause(&mut self) -> WdtPauseW<CtrlSpec> {
        WdtPauseW::new(self, 8)
    }
    #[doc = "Bit 9 - WWDT_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt_pause(&mut self) -> WwdtPauseW<CtrlSpec> {
        WwdtPauseW::new(self, 9)
    }
    #[doc = "Bit 10 - TMR1_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_pause(&mut self) -> Tmr1PauseW<CtrlSpec> {
        Tmr1PauseW::new(self, 10)
    }
    #[doc = "Bit 12 - TMR3_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_pause(&mut self) -> Tmr3PauseW<CtrlSpec> {
        Tmr3PauseW::new(self, 12)
    }
    #[doc = "Bit 14 - ERTC_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn ertc_pause(&mut self) -> ErtcPauseW<CtrlSpec> {
        ErtcPauseW::new(self, 14)
    }
    #[doc = "Bit 15 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_smbus_timeout(&mut self) -> I2c1SmbusTimeoutW<CtrlSpec> {
        I2c1SmbusTimeoutW::new(self, 15)
    }
    #[doc = "Bit 16 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_smbus_timeout(&mut self) -> I2c2SmbusTimeoutW<CtrlSpec> {
        I2c2SmbusTimeoutW::new(self, 16)
    }
    #[doc = "Bit 19 - TMR6_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6_pause(&mut self) -> Tmr6PauseW<CtrlSpec> {
        Tmr6PauseW::new(self, 19)
    }
    #[doc = "Bit 21 - ERTC_512_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn ertc_512_pause(&mut self) -> Ertc512PauseW<CtrlSpec> {
        Ertc512PauseW::new(self, 21)
    }
    #[doc = "Bit 22 - TMR15_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr15_pause(&mut self) -> Tmr15PauseW<CtrlSpec> {
        Tmr15PauseW::new(self, 22)
    }
    #[doc = "Bit 23 - TMR16_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr16_pause(&mut self) -> Tmr16PauseW<CtrlSpec> {
        Tmr16PauseW::new(self, 23)
    }
    #[doc = "Bit 24 - TMR17_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr17_pause(&mut self) -> Tmr17PauseW<CtrlSpec> {
        Tmr17PauseW::new(self, 24)
    }
    #[doc = "Bit 27 - TMR14_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14_pause(&mut self) -> Tmr14PauseW<CtrlSpec> {
        Tmr14PauseW::new(self, 27)
    }
}
#[doc = "DEBUG_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
