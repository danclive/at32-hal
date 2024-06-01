#[doc = "Register `APB1RST` reader"]
pub type R = crate::R<Apb1rstSpec>;
#[doc = "Register `APB1RST` writer"]
pub type W = crate::W<Apb1rstSpec>;
#[doc = "Field `TMR3RST` reader - Timer 3 reset"]
pub type Tmr3rstR = crate::BitReader;
#[doc = "Field `TMR3RST` writer - Timer 3 reset"]
pub type Tmr3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6RST` reader - Timer 6 reset"]
pub type Tmr6rstR = crate::BitReader;
#[doc = "Field `TMR6RST` writer - Timer 6 reset"]
pub type Tmr6rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14RST` reader - Timer 14 reset"]
pub type Tmr14rstR = crate::BitReader;
#[doc = "Field `TMR14RST` writer - Timer 14 reset"]
pub type Tmr14rstW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2c1rstR = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2c2rstR = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2c2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWCRST` reader - Power controller reset"]
pub type PwcrstR = crate::BitReader;
#[doc = "Field `PWCRST` writer - Power controller reset"]
pub type PwcrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    pub fn tmr3rst(&self) -> Tmr3rstR {
        Tmr3rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    pub fn tmr6rst(&self) -> Tmr6rstR {
        Tmr6rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer 14 reset"]
    #[inline(always)]
    pub fn tmr14rst(&self) -> Tmr14rstR {
        Tmr14rstR::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bit 28 - Power controller reset"]
    #[inline(always)]
    pub fn pwcrst(&self) -> PwcrstR {
        PwcrstR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RST")
            .field("tmr3rst", &self.tmr3rst())
            .field("tmr6rst", &self.tmr6rst())
            .field("tmr14rst", &self.tmr14rst())
            .field("wwdtrst", &self.wwdtrst())
            .field("spi2rst", &self.spi2rst())
            .field("usart2rst", &self.usart2rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("pwcrst", &self.pwcrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3rst(&mut self) -> Tmr3rstW<Apb1rstSpec> {
        Tmr3rstW::new(self, 1)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6rst(&mut self) -> Tmr6rstW<Apb1rstSpec> {
        Tmr6rstW::new(self, 4)
    }
    #[doc = "Bit 8 - Timer 14 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14rst(&mut self) -> Tmr14rstW<Apb1rstSpec> {
        Tmr14rstW::new(self, 8)
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
    #[doc = "Bit 28 - Power controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwcrst(&mut self) -> PwcrstW<Apb1rstSpec> {
        PwcrstW::new(self, 28)
    }
}
#[doc = "APB1 peripheral reset register (CRM_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
