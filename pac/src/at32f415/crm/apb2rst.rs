#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<Apb2rstSpec>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<Apb2rstSpec>;
#[doc = "Field `IOMUXRST` reader - MUX function I/O reset"]
pub type IomuxrstR = crate::BitReader;
#[doc = "Field `IOMUXRST` writer - MUX function I/O reset"]
pub type IomuxrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXINTRST` reader - External interrupt reset"]
pub type ExintrstR = crate::BitReader;
#[doc = "Field `EXINTRST` writer - External interrupt reset"]
pub type ExintrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOARST` reader - IO port A reset"]
pub type GpioarstR = crate::BitReader;
#[doc = "Field `GPIOARST` writer - IO port A reset"]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub type GpiobrstR = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub type GpiocrstR = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub type GpiocrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub type GpiodrstR = crate::BitReader;
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub type GpiodrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFRST` reader - IO port F reset"]
pub type GpiofrstR = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - IO port F reset"]
pub type GpiofrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1RST` reader - ADC1 reset"]
pub type Adc1rstR = crate::BitReader;
#[doc = "Field `ADC1RST` writer - ADC1 reset"]
pub type Adc1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1RST` reader - Timer1 reset"]
pub type Tmr1rstR = crate::BitReader;
#[doc = "Field `TMR1RST` writer - Timer1 reset"]
pub type Tmr1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9RST` reader - Timer9 reset"]
pub type Tmr9rstR = crate::BitReader;
#[doc = "Field `TMR9RST` writer - Timer9 reset"]
pub type Tmr9rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10RST` reader - Timer10 reset"]
pub type Tmr10rstR = crate::BitReader;
#[doc = "Field `TMR10RST` writer - Timer10 reset"]
pub type Tmr10rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11RST` reader - Timer11 reset"]
pub type Tmr11rstR = crate::BitReader;
#[doc = "Field `TMR11RST` writer - Timer11 reset"]
pub type Tmr11rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCRST` reader - ACC reset"]
pub type AccrstR = crate::BitReader;
#[doc = "Field `ACCRST` writer - ACC reset"]
pub type AccrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MUX function I/O reset"]
    #[inline(always)]
    pub fn iomuxrst(&self) -> IomuxrstR {
        IomuxrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    pub fn exintrst(&self) -> ExintrstR {
        ExintrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GpiocrstR {
        GpiocrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GpiodrstR {
        GpiodrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GpiofrstR {
        GpiofrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1rst(&self) -> Adc1rstR {
        Adc1rstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer1 reset"]
    #[inline(always)]
    pub fn tmr1rst(&self) -> Tmr1rstR {
        Tmr1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer9 reset"]
    #[inline(always)]
    pub fn tmr9rst(&self) -> Tmr9rstR {
        Tmr9rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer10 reset"]
    #[inline(always)]
    pub fn tmr10rst(&self) -> Tmr10rstR {
        Tmr10rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer11 reset"]
    #[inline(always)]
    pub fn tmr11rst(&self) -> Tmr11rstR {
        Tmr11rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ACC reset"]
    #[inline(always)]
    pub fn accrst(&self) -> AccrstR {
        AccrstR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RST")
            .field("iomuxrst", &self.iomuxrst())
            .field("exintrst", &self.exintrst())
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpiofrst", &self.gpiofrst())
            .field("adc1rst", &self.adc1rst())
            .field("tmr1rst", &self.tmr1rst())
            .field("spi1rst", &self.spi1rst())
            .field("usart1rst", &self.usart1rst())
            .field("tmr9rst", &self.tmr9rst())
            .field("tmr10rst", &self.tmr10rst())
            .field("tmr11rst", &self.tmr11rst())
            .field("accrst", &self.accrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MUX function I/O reset"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxrst(&mut self) -> IomuxrstW<Apb2rstSpec> {
        IomuxrstW::new(self, 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    #[must_use]
    pub fn exintrst(&mut self) -> ExintrstW<Apb2rstSpec> {
        ExintrstW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GpioarstW<Apb2rstSpec> {
        GpioarstW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GpiobrstW<Apb2rstSpec> {
        GpiobrstW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GpiocrstW<Apb2rstSpec> {
        GpiocrstW::new(self, 4)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GpiodrstW<Apb2rstSpec> {
        GpiodrstW::new(self, 5)
    }
    #[doc = "Bit 7 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GpiofrstW<Apb2rstSpec> {
        GpiofrstW::new(self, 7)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> Adc1rstW<Apb2rstSpec> {
        Adc1rstW::new(self, 9)
    }
    #[doc = "Bit 11 - Timer1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1rst(&mut self) -> Tmr1rstW<Apb2rstSpec> {
        Tmr1rstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> Spi1rstW<Apb2rstSpec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> Usart1rstW<Apb2rstSpec> {
        Usart1rstW::new(self, 14)
    }
    #[doc = "Bit 19 - Timer9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9rst(&mut self) -> Tmr9rstW<Apb2rstSpec> {
        Tmr9rstW::new(self, 19)
    }
    #[doc = "Bit 20 - Timer10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10rst(&mut self) -> Tmr10rstW<Apb2rstSpec> {
        Tmr10rstW::new(self, 20)
    }
    #[doc = "Bit 21 - Timer11 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11rst(&mut self) -> Tmr11rstW<Apb2rstSpec> {
        Tmr11rstW::new(self, 21)
    }
    #[doc = "Bit 22 - ACC reset"]
    #[inline(always)]
    #[must_use]
    pub fn accrst(&mut self) -> AccrstW<Apb2rstSpec> {
        AccrstW::new(self, 22)
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
