#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<Apb2enSpec>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<Apb2enSpec>;
#[doc = "Field `TMR1EN` reader - Timer1 clock enable"]
pub type Tmr1enR = crate::BitReader;
#[doc = "Field `TMR1EN` writer - Timer1 clock enable"]
pub type Tmr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8EN` reader - Timer8 clock enable"]
pub type Tmr8enR = crate::BitReader;
#[doc = "Field `TMR8EN` writer - Timer8 clock enable"]
pub type Tmr8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type Usart1enR = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6EN` reader - USART6 clock enable"]
pub type Usart6enR = crate::BitReader;
#[doc = "Field `USART6EN` writer - USART6 clock enable"]
pub type Usart6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub type Adc1enR = crate::BitReader;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub type Adc1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2EN` reader - ADC2 clock enable"]
pub type Adc2enR = crate::BitReader;
#[doc = "Field `ADC2EN` writer - ADC2 clock enable"]
pub type Adc2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3EN` reader - ADC3 clock enable"]
pub type Adc3enR = crate::BitReader;
#[doc = "Field `ADC3EN` writer - ADC3 clock enable"]
pub type Adc3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4EN` reader - SPI4 clock enable"]
pub type Spi4enR = crate::BitReader;
#[doc = "Field `SPI4EN` writer - SPI4 clock enable"]
pub type Spi4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCFGEN` reader - SCFG clock enable"]
pub type ScfgenR = crate::BitReader;
#[doc = "Field `SCFGEN` writer - SCFG clock enable"]
pub type ScfgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9EN` reader - Timer9 clock enable"]
pub type Tmr9enR = crate::BitReader;
#[doc = "Field `TMR9EN` writer - Timer9 clock enable"]
pub type Tmr9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10EN` reader - Timer10 clock enable"]
pub type Tmr10enR = crate::BitReader;
#[doc = "Field `TMR10EN` writer - Timer10 clock enable"]
pub type Tmr10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11EN` reader - Timer11 clock enable"]
pub type Tmr11enR = crate::BitReader;
#[doc = "Field `TMR11EN` writer - Timer11 clock enable"]
pub type Tmr11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR20EN` reader - Timer20 clock enable"]
pub type Tmr20enR = crate::BitReader;
#[doc = "Field `TMR20EN` writer - Timer20 clock enable"]
pub type Tmr20enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCEN` reader - ACC clock enable"]
pub type AccenR = crate::BitReader;
#[doc = "Field `ACCEN` writer - ACC clock enable"]
pub type AccenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer1 clock enable"]
    #[inline(always)]
    pub fn tmr1en(&self) -> Tmr1enR {
        Tmr1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer8 clock enable"]
    #[inline(always)]
    pub fn tmr8en(&self) -> Tmr8enR {
        Tmr8enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> Usart6enR {
        Usart6enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> Adc1enR {
        Adc1enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> Adc2enR {
        Adc2enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> Adc3enR {
        Adc3enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4en(&self) -> Spi4enR {
        Spi4enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCFG clock enable"]
    #[inline(always)]
    pub fn scfgen(&self) -> ScfgenR {
        ScfgenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer9 clock enable"]
    #[inline(always)]
    pub fn tmr9en(&self) -> Tmr9enR {
        Tmr9enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer10 clock enable"]
    #[inline(always)]
    pub fn tmr10en(&self) -> Tmr10enR {
        Tmr10enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer11 clock enable"]
    #[inline(always)]
    pub fn tmr11en(&self) -> Tmr11enR {
        Tmr11enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer20 clock enable"]
    #[inline(always)]
    pub fn tmr20en(&self) -> Tmr20enR {
        Tmr20enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - ACC clock enable"]
    #[inline(always)]
    pub fn accen(&self) -> AccenR {
        AccenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2EN")
            .field("tmr1en", &self.tmr1en())
            .field("tmr8en", &self.tmr8en())
            .field("usart1en", &self.usart1en())
            .field("usart6en", &self.usart6en())
            .field("adc1en", &self.adc1en())
            .field("adc2en", &self.adc2en())
            .field("adc3en", &self.adc3en())
            .field("spi1en", &self.spi1en())
            .field("spi4en", &self.spi4en())
            .field("scfgen", &self.scfgen())
            .field("tmr9en", &self.tmr9en())
            .field("tmr10en", &self.tmr10en())
            .field("tmr11en", &self.tmr11en())
            .field("tmr20en", &self.tmr20en())
            .field("accen", &self.accen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1en(&mut self) -> Tmr1enW<Apb2enSpec> {
        Tmr1enW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer8 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8en(&mut self) -> Tmr8enW<Apb2enSpec> {
        Tmr8enW::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> Usart1enW<Apb2enSpec> {
        Usart1enW::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> Usart6enW<Apb2enSpec> {
        Usart6enW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> Adc1enW<Apb2enSpec> {
        Adc1enW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc2en(&mut self) -> Adc2enW<Apb2enSpec> {
        Adc2enW::new(self, 9)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3en(&mut self) -> Adc3enW<Apb2enSpec> {
        Adc3enW::new(self, 10)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> Spi1enW<Apb2enSpec> {
        Spi1enW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> Spi4enW<Apb2enSpec> {
        Spi4enW::new(self, 13)
    }
    #[doc = "Bit 14 - SCFG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfgen(&mut self) -> ScfgenW<Apb2enSpec> {
        ScfgenW::new(self, 14)
    }
    #[doc = "Bit 16 - Timer9 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9en(&mut self) -> Tmr9enW<Apb2enSpec> {
        Tmr9enW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer10 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10en(&mut self) -> Tmr10enW<Apb2enSpec> {
        Tmr10enW::new(self, 17)
    }
    #[doc = "Bit 18 - Timer11 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11en(&mut self) -> Tmr11enW<Apb2enSpec> {
        Tmr11enW::new(self, 18)
    }
    #[doc = "Bit 20 - Timer20 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr20en(&mut self) -> Tmr20enW<Apb2enSpec> {
        Tmr20enW::new(self, 20)
    }
    #[doc = "Bit 29 - ACC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn accen(&mut self) -> AccenW<Apb2enSpec> {
        AccenW::new(self, 29)
    }
}
#[doc = "APB2 peripheral clock enable register (CRM_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2enSpec;
impl crate::RegisterSpec for Apb2enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2en::R`](R) reader structure"]
impl crate::Readable for Apb2enSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2en::W`](W) writer structure"]
impl crate::Writable for Apb2enSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2EN to value 0"]
impl crate::Resettable for Apb2enSpec {
    const RESET_VALUE: u32 = 0;
}
