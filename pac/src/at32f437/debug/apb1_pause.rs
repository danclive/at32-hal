#[doc = "Register `APB1_PAUSE` reader"]
pub type R = crate::R<Apb1PauseSpec>;
#[doc = "Register `APB1_PAUSE` writer"]
pub type W = crate::W<Apb1PauseSpec>;
#[doc = "Field `TMR2_PAUSE` reader - TMR2_PAUSE"]
pub type Tmr2PauseR = crate::BitReader;
#[doc = "Field `TMR2_PAUSE` writer - TMR2_PAUSE"]
pub type Tmr2PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3_PAUSE` reader - TMR3_PAUSE"]
pub type Tmr3PauseR = crate::BitReader;
#[doc = "Field `TMR3_PAUSE` writer - TMR3_PAUSE"]
pub type Tmr3PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4_PAUSE` reader - TMR4_PAUSE"]
pub type Tmr4PauseR = crate::BitReader;
#[doc = "Field `TMR4_PAUSE` writer - TMR4_PAUSE"]
pub type Tmr4PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5_PAUSE` reader - TMR5_PAUSE"]
pub type Tmr5PauseR = crate::BitReader;
#[doc = "Field `TMR5_PAUSE` writer - TMR5_PAUSE"]
pub type Tmr5PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6_PAUSE` reader - TMR6_PAUSE"]
pub type Tmr6PauseR = crate::BitReader;
#[doc = "Field `TMR6_PAUSE` writer - TMR6_PAUSE"]
pub type Tmr6PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR7_PAUSE` reader - TMR7_PAUSE"]
pub type Tmr7PauseR = crate::BitReader;
#[doc = "Field `TMR7_PAUSE` writer - TMR7_PAUSE"]
pub type Tmr7PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR12_PAUSE` reader - TMR12_PAUSE"]
pub type Tmr12PauseR = crate::BitReader;
#[doc = "Field `TMR12_PAUSE` writer - TMR12_PAUSE"]
pub type Tmr12PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR13_PAUSE` reader - TMR13_PAUSE"]
pub type Tmr13PauseR = crate::BitReader;
#[doc = "Field `TMR13_PAUSE` writer - TMR13_PAUSE"]
pub type Tmr13PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14_PAUSE` reader - TMR14_PAUSE"]
pub type Tmr14PauseR = crate::BitReader;
#[doc = "Field `TMR14_PAUSE` writer - TMR14_PAUSE"]
pub type Tmr14PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERTC_PAUSE` reader - ERTC_PAUSE"]
pub type ErtcPauseR = crate::BitReader;
#[doc = "Field `ERTC_PAUSE` writer - ERTC_PAUSE"]
pub type ErtcPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT_PAUSE` reader - WWDT_PAUSE"]
pub type WwdtPauseR = crate::BitReader;
#[doc = "Field `WWDT_PAUSE` writer - WWDT_PAUSE"]
pub type WwdtPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_PAUSE` reader - WDT_PAUSE"]
pub type WdtPauseR = crate::BitReader;
#[doc = "Field `WDT_PAUSE` writer - WDT_PAUSE"]
pub type WdtPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERTC512_PAUSE` reader - ERTC512_PAUSE"]
pub type Ertc512PauseR = crate::BitReader;
#[doc = "Field `ERTC512_PAUSE` writer - ERTC512_PAUSE"]
pub type Ertc512PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` reader - I2C1_SMBUS_TIMEOUT"]
pub type I2c1SmbusTimeoutR = crate::BitReader;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` writer - I2C1_SMBUS_TIMEOUT"]
pub type I2c1SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1_PAUSE` reader - CAN1_PAUSE"]
pub type Can1PauseR = crate::BitReader;
#[doc = "Field `CAN1_PAUSE` writer - CAN1_PAUSE"]
pub type Can1PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2_PAUSE` reader - CAN2_PAUSE"]
pub type Can2PauseR = crate::BitReader;
#[doc = "Field `CAN2_PAUSE` writer - CAN2_PAUSE"]
pub type Can2PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` reader - I2C2_SMBUS_TIMEOUT"]
pub type I2c2SmbusTimeoutR = crate::BitReader;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` writer - I2C2_SMBUS_TIMEOUT"]
pub type I2c2SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_SMBUS_TIMEOUT` reader - I2C3_SMBUS_TIMEOUT"]
pub type I2c3SmbusTimeoutR = crate::BitReader;
#[doc = "Field `I2C3_SMBUS_TIMEOUT` writer - I2C3_SMBUS_TIMEOUT"]
pub type I2c3SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TMR2_PAUSE"]
    #[inline(always)]
    pub fn tmr2_pause(&self) -> Tmr2PauseR {
        Tmr2PauseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMR3_PAUSE"]
    #[inline(always)]
    pub fn tmr3_pause(&self) -> Tmr3PauseR {
        Tmr3PauseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TMR4_PAUSE"]
    #[inline(always)]
    pub fn tmr4_pause(&self) -> Tmr4PauseR {
        Tmr4PauseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TMR5_PAUSE"]
    #[inline(always)]
    pub fn tmr5_pause(&self) -> Tmr5PauseR {
        Tmr5PauseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TMR6_PAUSE"]
    #[inline(always)]
    pub fn tmr6_pause(&self) -> Tmr6PauseR {
        Tmr6PauseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TMR7_PAUSE"]
    #[inline(always)]
    pub fn tmr7_pause(&self) -> Tmr7PauseR {
        Tmr7PauseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TMR12_PAUSE"]
    #[inline(always)]
    pub fn tmr12_pause(&self) -> Tmr12PauseR {
        Tmr12PauseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TMR13_PAUSE"]
    #[inline(always)]
    pub fn tmr13_pause(&self) -> Tmr13PauseR {
        Tmr13PauseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TMR14_PAUSE"]
    #[inline(always)]
    pub fn tmr14_pause(&self) -> Tmr14PauseR {
        Tmr14PauseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - ERTC_PAUSE"]
    #[inline(always)]
    pub fn ertc_pause(&self) -> ErtcPauseR {
        ErtcPauseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDT_PAUSE"]
    #[inline(always)]
    pub fn wwdt_pause(&self) -> WwdtPauseR {
        WwdtPauseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WDT_PAUSE"]
    #[inline(always)]
    pub fn wdt_pause(&self) -> WdtPauseR {
        WdtPauseR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - ERTC512_PAUSE"]
    #[inline(always)]
    pub fn ertc512_pause(&self) -> Ertc512PauseR {
        Ertc512PauseR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&self) -> I2c1SmbusTimeoutR {
        I2c1SmbusTimeoutR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1_PAUSE"]
    #[inline(always)]
    pub fn can1_pause(&self) -> Can1PauseR {
        Can1PauseR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2_PAUSE"]
    #[inline(always)]
    pub fn can2_pause(&self) -> Can2PauseR {
        Can2PauseR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&self) -> I2c2SmbusTimeoutR {
        I2c2SmbusTimeoutR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - I2C3_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c3_smbus_timeout(&self) -> I2c3SmbusTimeoutR {
        I2c3SmbusTimeoutR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1_PAUSE")
            .field("tmr2_pause", &self.tmr2_pause())
            .field("tmr3_pause", &self.tmr3_pause())
            .field("tmr4_pause", &self.tmr4_pause())
            .field("tmr5_pause", &self.tmr5_pause())
            .field("tmr6_pause", &self.tmr6_pause())
            .field("tmr7_pause", &self.tmr7_pause())
            .field("tmr12_pause", &self.tmr12_pause())
            .field("tmr13_pause", &self.tmr13_pause())
            .field("tmr14_pause", &self.tmr14_pause())
            .field("ertc_pause", &self.ertc_pause())
            .field("wwdt_pause", &self.wwdt_pause())
            .field("wdt_pause", &self.wdt_pause())
            .field("ertc512_pause", &self.ertc512_pause())
            .field("i2c1_smbus_timeout", &self.i2c1_smbus_timeout())
            .field("can1_pause", &self.can1_pause())
            .field("can2_pause", &self.can2_pause())
            .field("i2c2_smbus_timeout", &self.i2c2_smbus_timeout())
            .field("i2c3_smbus_timeout", &self.i2c3_smbus_timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TMR2_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_pause(&mut self) -> Tmr2PauseW<Apb1PauseSpec> {
        Tmr2PauseW::new(self, 0)
    }
    #[doc = "Bit 1 - TMR3_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_pause(&mut self) -> Tmr3PauseW<Apb1PauseSpec> {
        Tmr3PauseW::new(self, 1)
    }
    #[doc = "Bit 2 - TMR4_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4_pause(&mut self) -> Tmr4PauseW<Apb1PauseSpec> {
        Tmr4PauseW::new(self, 2)
    }
    #[doc = "Bit 3 - TMR5_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5_pause(&mut self) -> Tmr5PauseW<Apb1PauseSpec> {
        Tmr5PauseW::new(self, 3)
    }
    #[doc = "Bit 4 - TMR6_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6_pause(&mut self) -> Tmr6PauseW<Apb1PauseSpec> {
        Tmr6PauseW::new(self, 4)
    }
    #[doc = "Bit 5 - TMR7_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr7_pause(&mut self) -> Tmr7PauseW<Apb1PauseSpec> {
        Tmr7PauseW::new(self, 5)
    }
    #[doc = "Bit 6 - TMR12_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr12_pause(&mut self) -> Tmr12PauseW<Apb1PauseSpec> {
        Tmr12PauseW::new(self, 6)
    }
    #[doc = "Bit 7 - TMR13_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13_pause(&mut self) -> Tmr13PauseW<Apb1PauseSpec> {
        Tmr13PauseW::new(self, 7)
    }
    #[doc = "Bit 8 - TMR14_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14_pause(&mut self) -> Tmr14PauseW<Apb1PauseSpec> {
        Tmr14PauseW::new(self, 8)
    }
    #[doc = "Bit 10 - ERTC_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn ertc_pause(&mut self) -> ErtcPauseW<Apb1PauseSpec> {
        ErtcPauseW::new(self, 10)
    }
    #[doc = "Bit 11 - WWDT_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt_pause(&mut self) -> WwdtPauseW<Apb1PauseSpec> {
        WwdtPauseW::new(self, 11)
    }
    #[doc = "Bit 12 - WDT_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_pause(&mut self) -> WdtPauseW<Apb1PauseSpec> {
        WdtPauseW::new(self, 12)
    }
    #[doc = "Bit 15 - ERTC512_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn ertc512_pause(&mut self) -> Ertc512PauseW<Apb1PauseSpec> {
        Ertc512PauseW::new(self, 15)
    }
    #[doc = "Bit 24 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_smbus_timeout(&mut self) -> I2c1SmbusTimeoutW<Apb1PauseSpec> {
        I2c1SmbusTimeoutW::new(self, 24)
    }
    #[doc = "Bit 25 - CAN1_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn can1_pause(&mut self) -> Can1PauseW<Apb1PauseSpec> {
        Can1PauseW::new(self, 25)
    }
    #[doc = "Bit 26 - CAN2_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn can2_pause(&mut self) -> Can2PauseW<Apb1PauseSpec> {
        Can2PauseW::new(self, 26)
    }
    #[doc = "Bit 27 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_smbus_timeout(&mut self) -> I2c2SmbusTimeoutW<Apb1PauseSpec> {
        I2c2SmbusTimeoutW::new(self, 27)
    }
    #[doc = "Bit 28 - I2C3_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_smbus_timeout(&mut self) -> I2c3SmbusTimeoutW<Apb1PauseSpec> {
        I2c3SmbusTimeoutW::new(self, 28)
    }
}
#[doc = "DEBUG APB1 PAUSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1_pause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1_pause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1PauseSpec;
impl crate::RegisterSpec for Apb1PauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1_pause::R`](R) reader structure"]
impl crate::Readable for Apb1PauseSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1_pause::W`](W) writer structure"]
impl crate::Writable for Apb1PauseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1_PAUSE to value 0"]
impl crate::Resettable for Apb1PauseSpec {
    const RESET_VALUE: u32 = 0;
}
