#[doc = "Register `APB2LPEN` reader"]
pub type R = crate::R<Apb2lpenSpec>;
#[doc = "Register `APB2LPEN` writer"]
pub type W = crate::W<Apb2lpenSpec>;
#[doc = "Field `TMR1LPEN` reader - Timer1 clock enable during sleep mode"]
pub type Tmr1lpenR = crate::BitReader;
#[doc = "Field `TMR1LPEN` writer - Timer1 clock enable during sleep mode"]
pub type Tmr1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8LPEN` reader - Timer8 clock enable during sleep mode"]
pub type Tmr8lpenR = crate::BitReader;
#[doc = "Field `TMR8LPEN` writer - Timer8 clock enable during sleep mode"]
pub type Tmr8lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during sleep mode"]
pub type Usart1lpenR = crate::BitReader;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during sleep mode"]
pub type Usart1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6LPEN` reader - USART6 clock enable during sleep mode"]
pub type Usart6lpenR = crate::BitReader;
#[doc = "Field `USART6LPEN` writer - USART6 clock enable during sleep mode"]
pub type Usart6lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1CPEN` reader - ADC1 clock enable during sleep mode"]
pub type Adc1cpenR = crate::BitReader;
#[doc = "Field `ADC1CPEN` writer - ADC1 clock enable during sleep mode"]
pub type Adc1cpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2CPEN` reader - ADC2 clock enable during sleep mode"]
pub type Adc2cpenR = crate::BitReader;
#[doc = "Field `ADC2CPEN` writer - ADC2 clock enable during sleep mode"]
pub type Adc2cpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3EN` reader - ADC3 clock enable during sleep mode"]
pub type Adc3enR = crate::BitReader;
#[doc = "Field `ADC3EN` writer - ADC3 clock enable during sleep mode"]
pub type Adc3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1LPEN` reader - SPI1 clock enable during sleep mode"]
pub type Spi1lpenR = crate::BitReader;
#[doc = "Field `SPI1LPEN` writer - SPI1 clock enable during sleep mode"]
pub type Spi1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4LPEN` reader - SPI4 clock enable during sleep mode"]
pub type Spi4lpenR = crate::BitReader;
#[doc = "Field `SPI4LPEN` writer - SPI4 clock enable during sleep mode"]
pub type Spi4lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCFGLPEN` reader - SCFG clock enable during sleep mode"]
pub type ScfglpenR = crate::BitReader;
#[doc = "Field `SCFGLPEN` writer - SCFG clock enable during sleep mode"]
pub type ScfglpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9LPEN` reader - Timer9 clock enable during sleep mode"]
pub type Tmr9lpenR = crate::BitReader;
#[doc = "Field `TMR9LPEN` writer - Timer9 clock enable during sleep mode"]
pub type Tmr9lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10LPEN` reader - Timer10 clock enable during sleep mode"]
pub type Tmr10lpenR = crate::BitReader;
#[doc = "Field `TMR10LPEN` writer - Timer10 clock enable during sleep mode"]
pub type Tmr10lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11LPEN` reader - Timer11 clock enable during sleep mode"]
pub type Tmr11lpenR = crate::BitReader;
#[doc = "Field `TMR11LPEN` writer - Timer11 clock enable during sleep mode"]
pub type Tmr11lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR20LPEN` reader - Timer20 clock enable during sleep mode"]
pub type Tmr20lpenR = crate::BitReader;
#[doc = "Field `TMR20LPEN` writer - Timer20 clock enable during sleep mode"]
pub type Tmr20lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCLPEN` reader - ACC clock enable during sleep mode"]
pub type AcclpenR = crate::BitReader;
#[doc = "Field `ACCLPEN` writer - ACC clock enable during sleep mode"]
pub type AcclpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr1lpen(&self) -> Tmr1lpenR {
        Tmr1lpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer8 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr8lpen(&self) -> Tmr8lpenR {
        Tmr8lpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> Usart1lpenR {
        Usart1lpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> Usart6lpenR {
        Usart6lpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn adc1cpen(&self) -> Adc1cpenR {
        Adc1cpenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn adc2cpen(&self) -> Adc2cpenR {
        Adc2cpenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn adc3en(&self) -> Adc3enR {
        Adc3enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> Spi1lpenR {
        Spi1lpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> Spi4lpenR {
        Spi4lpenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCFG clock enable during sleep mode"]
    #[inline(always)]
    pub fn scfglpen(&self) -> ScfglpenR {
        ScfglpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer9 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr9lpen(&self) -> Tmr9lpenR {
        Tmr9lpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer10 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr10lpen(&self) -> Tmr10lpenR {
        Tmr10lpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer11 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr11lpen(&self) -> Tmr11lpenR {
        Tmr11lpenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer20 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr20lpen(&self) -> Tmr20lpenR {
        Tmr20lpenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - ACC clock enable during sleep mode"]
    #[inline(always)]
    pub fn acclpen(&self) -> AcclpenR {
        AcclpenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2LPEN")
            .field("tmr1lpen", &self.tmr1lpen())
            .field("tmr8lpen", &self.tmr8lpen())
            .field("usart1lpen", &self.usart1lpen())
            .field("usart6lpen", &self.usart6lpen())
            .field("adc1cpen", &self.adc1cpen())
            .field("adc2cpen", &self.adc2cpen())
            .field("adc3en", &self.adc3en())
            .field("spi1lpen", &self.spi1lpen())
            .field("spi4lpen", &self.spi4lpen())
            .field("scfglpen", &self.scfglpen())
            .field("tmr9lpen", &self.tmr9lpen())
            .field("tmr10lpen", &self.tmr10lpen())
            .field("tmr11lpen", &self.tmr11lpen())
            .field("tmr20lpen", &self.tmr20lpen())
            .field("acclpen", &self.acclpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1lpen(&mut self) -> Tmr1lpenW<Apb2lpenSpec> {
        Tmr1lpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer8 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8lpen(&mut self) -> Tmr8lpenW<Apb2lpenSpec> {
        Tmr8lpenW::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> Usart1lpenW<Apb2lpenSpec> {
        Usart1lpenW::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> Usart6lpenW<Apb2lpenSpec> {
        Usart6lpenW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc1cpen(&mut self) -> Adc1cpenW<Apb2lpenSpec> {
        Adc1cpenW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc2cpen(&mut self) -> Adc2cpenW<Apb2lpenSpec> {
        Adc2cpenW::new(self, 9)
    }
    #[doc = "Bit 10 - ADC3 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc3en(&mut self) -> Adc3enW<Apb2lpenSpec> {
        Adc3enW::new(self, 10)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> Spi1lpenW<Apb2lpenSpec> {
        Spi1lpenW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi4lpen(&mut self) -> Spi4lpenW<Apb2lpenSpec> {
        Spi4lpenW::new(self, 13)
    }
    #[doc = "Bit 14 - SCFG clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn scfglpen(&mut self) -> ScfglpenW<Apb2lpenSpec> {
        ScfglpenW::new(self, 14)
    }
    #[doc = "Bit 16 - Timer9 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9lpen(&mut self) -> Tmr9lpenW<Apb2lpenSpec> {
        Tmr9lpenW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer10 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10lpen(&mut self) -> Tmr10lpenW<Apb2lpenSpec> {
        Tmr10lpenW::new(self, 17)
    }
    #[doc = "Bit 18 - Timer11 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11lpen(&mut self) -> Tmr11lpenW<Apb2lpenSpec> {
        Tmr11lpenW::new(self, 18)
    }
    #[doc = "Bit 20 - Timer20 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr20lpen(&mut self) -> Tmr20lpenW<Apb2lpenSpec> {
        Tmr20lpenW::new(self, 20)
    }
    #[doc = "Bit 29 - ACC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn acclpen(&mut self) -> AcclpenW<Apb2lpenSpec> {
        AcclpenW::new(self, 29)
    }
}
#[doc = "APB2 peripheral Low-power clock enable register (CRM_APB2LPEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2lpenSpec;
impl crate::RegisterSpec for Apb2lpenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2lpen::R`](R) reader structure"]
impl crate::Readable for Apb2lpenSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2lpen::W`](W) writer structure"]
impl crate::Writable for Apb2lpenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2LPEN to value 0x2017_7733"]
impl crate::Resettable for Apb2lpenSpec {
    const RESET_VALUE: u32 = 0x2017_7733;
}
