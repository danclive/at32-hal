#[doc = "Register `APB1EN` reader"]
pub type R = crate::R<Apb1enSpec>;
#[doc = "Register `APB1EN` writer"]
pub type W = crate::W<Apb1enSpec>;
#[doc = "Field `TMR3EN` reader - Timer3 clock enable"]
pub type Tmr3enR = crate::BitReader;
#[doc = "Field `TMR3EN` writer - Timer3 clock enable"]
pub type Tmr3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6EN` reader - Timer6 clock enable"]
pub type Tmr6enR = crate::BitReader;
#[doc = "Field `TMR6EN` writer - Timer6 clock enable"]
pub type Tmr6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14EN` reader - Timer14 clock enable"]
pub type Tmr14enR = crate::BitReader;
#[doc = "Field `TMR14EN` writer - Timer14 clock enable"]
pub type Tmr14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTEN` reader - Window watchdog timer clock enable"]
pub type WwdtenR = crate::BitReader;
#[doc = "Field `WWDTEN` writer - Window watchdog timer clock enable"]
pub type WwdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub type Spi2enR = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub type Spi2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub type Usart2enR = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub type Usart2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub type I2c1enR = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
pub type I2c2enR = crate::BitReader;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub type I2c2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWCEN` reader - Power clock enable"]
pub type PwcenR = crate::BitReader;
#[doc = "Field `PWCEN` writer - Power clock enable"]
pub type PwcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Timer3 clock enable"]
    #[inline(always)]
    pub fn tmr3en(&self) -> Tmr3enR {
        Tmr3enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer6 clock enable"]
    #[inline(always)]
    pub fn tmr6en(&self) -> Tmr6enR {
        Tmr6enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer14 clock enable"]
    #[inline(always)]
    pub fn tmr14en(&self) -> Tmr14enR {
        Tmr14enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdten(&self) -> WwdtenR {
        WwdtenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> Spi2enR {
        Spi2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Power clock enable"]
    #[inline(always)]
    pub fn pwcen(&self) -> PwcenR {
        PwcenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1EN")
            .field("tmr3en", &self.tmr3en())
            .field("tmr6en", &self.tmr6en())
            .field("tmr14en", &self.tmr14en())
            .field("wwdten", &self.wwdten())
            .field("spi2en", &self.spi2en())
            .field("usart2en", &self.usart2en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("pwcen", &self.pwcen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Timer3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3en(&mut self) -> Tmr3enW<Apb1enSpec> {
        Tmr3enW::new(self, 1)
    }
    #[doc = "Bit 4 - Timer6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6en(&mut self) -> Tmr6enW<Apb1enSpec> {
        Tmr6enW::new(self, 4)
    }
    #[doc = "Bit 8 - Timer14 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14en(&mut self) -> Tmr14enW<Apb1enSpec> {
        Tmr14enW::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdten(&mut self) -> WwdtenW<Apb1enSpec> {
        WwdtenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> Spi2enW<Apb1enSpec> {
        Spi2enW::new(self, 14)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> Usart2enW<Apb1enSpec> {
        Usart2enW::new(self, 17)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2c1enW<Apb1enSpec> {
        I2c1enW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2c2enW<Apb1enSpec> {
        I2c2enW::new(self, 22)
    }
    #[doc = "Bit 28 - Power clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwcen(&mut self) -> PwcenW<Apb1enSpec> {
        PwcenW::new(self, 28)
    }
}
#[doc = "APB1 peripheral clock enable register (CRM_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enSpec;
impl crate::RegisterSpec for Apb1enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1en::R`](R) reader structure"]
impl crate::Readable for Apb1enSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1en::W`](W) writer structure"]
impl crate::Writable for Apb1enSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1EN to value 0"]
impl crate::Resettable for Apb1enSpec {
    const RESET_VALUE: u32 = 0;
}
