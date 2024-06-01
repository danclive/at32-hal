#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<Apb2rstSpec>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<Apb2rstSpec>;
#[doc = "Field `TMR1RST` reader - Timer1 reset"]
pub type Tmr1rstR = crate::BitReader;
#[doc = "Field `TMR1RST` writer - Timer1 reset"]
pub type Tmr1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8RST` reader - Timer8 reset"]
pub type Tmr8rstR = crate::BitReader;
#[doc = "Field `TMR8RST` writer - Timer8 reset"]
pub type Tmr8rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6RST` reader - USART6 reset"]
pub type Usart6rstR = crate::BitReader;
#[doc = "Field `USART6RST` writer - USART6 reset"]
pub type Usart6rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type AdcrstR = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4RST` reader - SPI4 reset"]
pub type Spi4rstR = crate::BitReader;
#[doc = "Field `SPI4RST` writer - SPI4 reset"]
pub type Spi4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCFGRST` reader - SCFG reset"]
pub type ScfgrstR = crate::BitReader;
#[doc = "Field `SCFGRST` writer - SCFG reset"]
pub type ScfgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9RST` reader - Timer9 reset"]
pub type Tmr9rstR = crate::BitReader;
#[doc = "Field `TMR9RST` writer - Timer9 reset"]
pub type Tmr9rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10RST` reader - Timer10 reset"]
pub type Tmr10rstR = crate::BitReader;
#[doc = "Field `TMR10RST` writer - Timer10 reset"]
pub type Tmr10rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11RST` reader - Timer 11 reset"]
pub type Tmr11rstR = crate::BitReader;
#[doc = "Field `TMR11RST` writer - Timer 11 reset"]
pub type Tmr11rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR20RST` reader - Timer20 reset"]
pub type Tmr20rstR = crate::BitReader;
#[doc = "Field `TMR20RST` writer - Timer20 reset"]
pub type Tmr20rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCRST` reader - ACC reset"]
pub type AccrstR = crate::BitReader;
#[doc = "Field `ACCRST` writer - ACC reset"]
pub type AccrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer1 reset"]
    #[inline(always)]
    pub fn tmr1rst(&self) -> Tmr1rstR {
        Tmr1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer8 reset"]
    #[inline(always)]
    pub fn tmr8rst(&self) -> Tmr8rstR {
        Tmr8rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    pub fn usart6rst(&self) -> Usart6rstR {
        Usart6rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline(always)]
    pub fn spi4rst(&self) -> Spi4rstR {
        Spi4rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCFG reset"]
    #[inline(always)]
    pub fn scfgrst(&self) -> ScfgrstR {
        ScfgrstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer9 reset"]
    #[inline(always)]
    pub fn tmr9rst(&self) -> Tmr9rstR {
        Tmr9rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer10 reset"]
    #[inline(always)]
    pub fn tmr10rst(&self) -> Tmr10rstR {
        Tmr10rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer 11 reset"]
    #[inline(always)]
    pub fn tmr11rst(&self) -> Tmr11rstR {
        Tmr11rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer20 reset"]
    #[inline(always)]
    pub fn tmr20rst(&self) -> Tmr20rstR {
        Tmr20rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - ACC reset"]
    #[inline(always)]
    pub fn accrst(&self) -> AccrstR {
        AccrstR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RST")
            .field("tmr1rst", &self.tmr1rst())
            .field("tmr8rst", &self.tmr8rst())
            .field("usart1rst", &self.usart1rst())
            .field("usart6rst", &self.usart6rst())
            .field("adcrst", &self.adcrst())
            .field("spi1rst", &self.spi1rst())
            .field("spi4rst", &self.spi4rst())
            .field("scfgrst", &self.scfgrst())
            .field("tmr9rst", &self.tmr9rst())
            .field("tmr10rst", &self.tmr10rst())
            .field("tmr11rst", &self.tmr11rst())
            .field("tmr20rst", &self.tmr20rst())
            .field("accrst", &self.accrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1rst(&mut self) -> Tmr1rstW<Apb2rstSpec> {
        Tmr1rstW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8rst(&mut self) -> Tmr8rstW<Apb2rstSpec> {
        Tmr8rstW::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> Usart1rstW<Apb2rstSpec> {
        Usart1rstW::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> Usart6rstW<Apb2rstSpec> {
        Usart6rstW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> AdcrstW<Apb2rstSpec> {
        AdcrstW::new(self, 8)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> Spi1rstW<Apb2rstSpec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi4rst(&mut self) -> Spi4rstW<Apb2rstSpec> {
        Spi4rstW::new(self, 13)
    }
    #[doc = "Bit 14 - SCFG reset"]
    #[inline(always)]
    #[must_use]
    pub fn scfgrst(&mut self) -> ScfgrstW<Apb2rstSpec> {
        ScfgrstW::new(self, 14)
    }
    #[doc = "Bit 16 - Timer9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9rst(&mut self) -> Tmr9rstW<Apb2rstSpec> {
        Tmr9rstW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10rst(&mut self) -> Tmr10rstW<Apb2rstSpec> {
        Tmr10rstW::new(self, 17)
    }
    #[doc = "Bit 18 - Timer 11 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11rst(&mut self) -> Tmr11rstW<Apb2rstSpec> {
        Tmr11rstW::new(self, 18)
    }
    #[doc = "Bit 20 - Timer20 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr20rst(&mut self) -> Tmr20rstW<Apb2rstSpec> {
        Tmr20rstW::new(self, 20)
    }
    #[doc = "Bit 29 - ACC reset"]
    #[inline(always)]
    #[must_use]
    pub fn accrst(&mut self) -> AccrstW<Apb2rstSpec> {
        AccrstW::new(self, 29)
    }
}
#[doc = "APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstSpec;
impl crate::RegisterSpec for Apb2rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for Apb2rstSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for Apb2rstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for Apb2rstSpec {
    const RESET_VALUE: u32 = 0;
}
