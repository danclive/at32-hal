#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<Apb2enSpec>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<Apb2enSpec>;
#[doc = "Field `IOMUXEN` reader - MUX function I/O clock enable"]
pub type IomuxenR = crate::BitReader;
#[doc = "Field `IOMUXEN` writer - MUX function I/O clock enable"]
pub type IomuxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOAEN` reader - I/O port A clock enable"]
pub type GpioaenR = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - I/O port A clock enable"]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - I/O port B clock enable"]
pub type GpiobenR = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - I/O port B clock enable"]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - I/O port C clock enable"]
pub type GpiocenR = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - I/O port C clock enable"]
pub type GpiocenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - I/O port D clock enable"]
pub type GpiodenR = crate::BitReader;
#[doc = "Field `GPIODEN` writer - I/O port D clock enable"]
pub type GpiodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - I/O port F clock enable"]
pub type GpiofenR = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - I/O port F clock enable"]
pub type GpiofenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub type Adc1enR = crate::BitReader;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub type Adc1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1EN` reader - Timer1 clock enable"]
pub type Tmr1enR = crate::BitReader;
#[doc = "Field `TMR1EN` writer - Timer1 clock enable"]
pub type Tmr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type Usart1enR = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `ACCEN` reader - ACC clock enable"]
pub type AccenR = crate::BitReader;
#[doc = "Field `ACCEN` writer - ACC clock enable"]
pub type AccenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MUX function I/O clock enable"]
    #[inline(always)]
    pub fn iomuxen(&self) -> IomuxenR {
        IomuxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GpiocenR {
        GpiocenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GpiodenR {
        GpiodenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GpiofenR {
        GpiofenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> Adc1enR {
        Adc1enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer1 clock enable"]
    #[inline(always)]
    pub fn tmr1en(&self) -> Tmr1enR {
        Tmr1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer9 clock enable"]
    #[inline(always)]
    pub fn tmr9en(&self) -> Tmr9enR {
        Tmr9enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer10 clock enable"]
    #[inline(always)]
    pub fn tmr10en(&self) -> Tmr10enR {
        Tmr10enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer11 clock enable"]
    #[inline(always)]
    pub fn tmr11en(&self) -> Tmr11enR {
        Tmr11enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ACC clock enable"]
    #[inline(always)]
    pub fn accen(&self) -> AccenR {
        AccenR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2EN")
            .field("iomuxen", &self.iomuxen())
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpiofen", &self.gpiofen())
            .field("adc1en", &self.adc1en())
            .field("tmr1en", &self.tmr1en())
            .field("spi1en", &self.spi1en())
            .field("usart1en", &self.usart1en())
            .field("tmr9en", &self.tmr9en())
            .field("tmr10en", &self.tmr10en())
            .field("tmr11en", &self.tmr11en())
            .field("accen", &self.accen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MUX function I/O clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxen(&mut self) -> IomuxenW<Apb2enSpec> {
        IomuxenW::new(self, 0)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GpioaenW<Apb2enSpec> {
        GpioaenW::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GpiobenW<Apb2enSpec> {
        GpiobenW::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GpiocenW<Apb2enSpec> {
        GpiocenW::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GpiodenW<Apb2enSpec> {
        GpiodenW::new(self, 5)
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GpiofenW<Apb2enSpec> {
        GpiofenW::new(self, 7)
    }
    #[doc = "Bit 9 - ADC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> Adc1enW<Apb2enSpec> {
        Adc1enW::new(self, 9)
    }
    #[doc = "Bit 11 - Timer1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1en(&mut self) -> Tmr1enW<Apb2enSpec> {
        Tmr1enW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> Spi1enW<Apb2enSpec> {
        Spi1enW::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> Usart1enW<Apb2enSpec> {
        Usart1enW::new(self, 14)
    }
    #[doc = "Bit 19 - Timer9 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9en(&mut self) -> Tmr9enW<Apb2enSpec> {
        Tmr9enW::new(self, 19)
    }
    #[doc = "Bit 20 - Timer10 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10en(&mut self) -> Tmr10enW<Apb2enSpec> {
        Tmr10enW::new(self, 20)
    }
    #[doc = "Bit 21 - Timer11 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11en(&mut self) -> Tmr11enW<Apb2enSpec> {
        Tmr11enW::new(self, 21)
    }
    #[doc = "Bit 22 - ACC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn accen(&mut self) -> AccenW<Apb2enSpec> {
        AccenW::new(self, 22)
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
