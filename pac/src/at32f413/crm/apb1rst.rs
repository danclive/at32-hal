#[doc = "Register `APB1RST` reader"]
pub type R = crate::R<Apb1rstSpec>;
#[doc = "Register `APB1RST` writer"]
pub type W = crate::W<Apb1rstSpec>;
#[doc = "Field `TMR2RST` reader - Timer 2 reset"]
pub type Tmr2rstR = crate::BitReader;
#[doc = "Field `TMR2RST` writer - Timer 2 reset"]
pub type Tmr2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3RST` reader - Timer 3 reset"]
pub type Tmr3rstR = crate::BitReader;
#[doc = "Field `TMR3RST` writer - Timer 3 reset"]
pub type Tmr3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4RST` reader - Timer 4 reset"]
pub type Tmr4rstR = crate::BitReader;
#[doc = "Field `TMR4RST` writer - Timer 4 reset"]
pub type Tmr4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5RST` reader - Timer 5 reset"]
pub type Tmr5rstR = crate::BitReader;
#[doc = "Field `TMR5RST` writer - Timer 5 reset"]
pub type Tmr5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTRST` reader - Window watchdog timer reset"]
pub type WwdtrstR = crate::BitReader;
#[doc = "Field `WWDTRST` writer - Window watchdog timer reset"]
pub type WwdtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type Spi2rstR = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type Spi2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2RST` reader - USART 2 reset"]
pub type Usart2rstR = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART 2 reset"]
pub type Usart2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3RST` reader - USART 3 reset"]
pub type Usart3rstR = crate::BitReader;
#[doc = "Field `USART3RST` writer - USART 3 reset"]
pub type Usart3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4RST` reader - UART 4 reset"]
pub type Uart4rstR = crate::BitReader;
#[doc = "Field `UART4RST` writer - UART 4 reset"]
pub type Uart4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5RST` reader - UART 5 reset"]
pub type Uart5rstR = crate::BitReader;
#[doc = "Field `UART5RST` writer - UART 5 reset"]
pub type Uart5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2c1rstR = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2c2rstR = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2c2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB reset"]
pub type UsbrstR = crate::BitReader;
#[doc = "Field `USBRST` writer - USB reset"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1RST` reader - CAN1 reset"]
pub type Can1rstR = crate::BitReader;
#[doc = "Field `CAN1RST` writer - CAN1 reset"]
pub type Can1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPRRST` reader - Battery powered domain register reset"]
pub type BprrstR = crate::BitReader;
#[doc = "Field `BPRRST` writer - Battery powered domain register reset"]
pub type BprrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWCRST` reader - Power controller reset"]
pub type PwcrstR = crate::BitReader;
#[doc = "Field `PWCRST` writer - Power controller reset"]
pub type PwcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2RST` reader - CAN2 reset"]
pub type Can2rstR = crate::BitReader;
#[doc = "Field `CAN2RST` writer - CAN2 reset"]
pub type Can2rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    pub fn tmr2rst(&self) -> Tmr2rstR {
        Tmr2rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    pub fn tmr3rst(&self) -> Tmr3rstR {
        Tmr3rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    pub fn tmr4rst(&self) -> Tmr4rstR {
        Tmr4rstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    pub fn tmr5rst(&self) -> Tmr5rstR {
        Tmr5rstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    pub fn wwdtrst(&self) -> WwdtrstR {
        WwdtrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> Spi2rstR {
        Spi2rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> Usart2rstR {
        Usart2rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> Usart3rstR {
        Usart3rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART 4 reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> Uart4rstR {
        Uart4rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART 5 reset"]
    #[inline(always)]
    pub fn uart5rst(&self) -> Uart5rstR {
        Uart5rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2c2rstR {
        I2c2rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&self) -> Can1rstR {
        Can1rstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Battery powered domain register reset"]
    #[inline(always)]
    pub fn bprrst(&self) -> BprrstR {
        BprrstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power controller reset"]
    #[inline(always)]
    pub fn pwcrst(&self) -> PwcrstR {
        PwcrstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - CAN2 reset"]
    #[inline(always)]
    pub fn can2rst(&self) -> Can2rstR {
        Can2rstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RST")
            .field("tmr2rst", &self.tmr2rst())
            .field("tmr3rst", &self.tmr3rst())
            .field("tmr4rst", &self.tmr4rst())
            .field("tmr5rst", &self.tmr5rst())
            .field("wwdtrst", &self.wwdtrst())
            .field("spi2rst", &self.spi2rst())
            .field("usart2rst", &self.usart2rst())
            .field("usart3rst", &self.usart3rst())
            .field("uart4rst", &self.uart4rst())
            .field("uart5rst", &self.uart5rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("usbrst", &self.usbrst())
            .field("can1rst", &self.can1rst())
            .field("bprrst", &self.bprrst())
            .field("pwcrst", &self.pwcrst())
            .field("can2rst", &self.can2rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2rst(&mut self) -> Tmr2rstW<Apb1rstSpec> {
        Tmr2rstW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3rst(&mut self) -> Tmr3rstW<Apb1rstSpec> {
        Tmr3rstW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4rst(&mut self) -> Tmr4rstW<Apb1rstSpec> {
        Tmr4rstW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5rst(&mut self) -> Tmr5rstW<Apb1rstSpec> {
        Tmr5rstW::new(self, 3)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdtrst(&mut self) -> WwdtrstW<Apb1rstSpec> {
        WwdtrstW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> Spi2rstW<Apb1rstSpec> {
        Spi2rstW::new(self, 14)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> Usart2rstW<Apb1rstSpec> {
        Usart2rstW::new(self, 17)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> Usart3rstW<Apb1rstSpec> {
        Usart3rstW::new(self, 18)
    }
    #[doc = "Bit 19 - UART 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> Uart4rstW<Apb1rstSpec> {
        Uart4rstW::new(self, 19)
    }
    #[doc = "Bit 20 - UART 5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> Uart5rstW<Apb1rstSpec> {
        Uart5rstW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2c1rstW<Apb1rstSpec> {
        I2c1rstW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2c2rstW<Apb1rstSpec> {
        I2c2rstW::new(self, 22)
    }
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> UsbrstW<Apb1rstSpec> {
        UsbrstW::new(self, 23)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can1rst(&mut self) -> Can1rstW<Apb1rstSpec> {
        Can1rstW::new(self, 25)
    }
    #[doc = "Bit 27 - Battery powered domain register reset"]
    #[inline(always)]
    #[must_use]
    pub fn bprrst(&mut self) -> BprrstW<Apb1rstSpec> {
        BprrstW::new(self, 27)
    }
    #[doc = "Bit 28 - Power controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwcrst(&mut self) -> PwcrstW<Apb1rstSpec> {
        PwcrstW::new(self, 28)
    }
    #[doc = "Bit 31 - CAN2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can2rst(&mut self) -> Can2rstW<Apb1rstSpec> {
        Can2rstW::new(self, 31)
    }
}
#[doc = "APB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1rstSpec;
impl crate::RegisterSpec for Apb1rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rst::R`](R) reader structure"]
impl crate::Readable for Apb1rstSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1rst::W`](W) writer structure"]
impl crate::Writable for Apb1rstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RST to value 0"]
impl crate::Resettable for Apb1rstSpec {
    const RESET_VALUE: u32 = 0;
}
