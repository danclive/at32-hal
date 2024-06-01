#[doc = "Register `REMAP` reader"]
pub type R = crate::R<RemapSpec>;
#[doc = "Register `REMAP` writer"]
pub type W = crate::W<RemapSpec>;
#[doc = "Field `SPI1_MUX0` reader - SPI1 muxing bit0"]
pub type Spi1Mux0R = crate::BitReader;
#[doc = "Field `SPI1_MUX0` writer - SPI1 muxing bit0"]
pub type Spi1Mux0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_MUX` reader - I2C1 muxing"]
pub type I2c1MuxR = crate::BitReader;
#[doc = "Field `I2C1_MUX` writer - I2C1 muxing"]
pub type I2c1MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1_MUX` reader - USART1 muxing"]
pub type Usart1MuxR = crate::BitReader;
#[doc = "Field `USART1_MUX` writer - USART1 muxing"]
pub type Usart1MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3_MUX` reader - USART3 muxing"]
pub type Usart3MuxR = crate::FieldReader;
#[doc = "Field `USART3_MUX` writer - USART3 muxing"]
pub type Usart3MuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR1_MUX` reader - TMR1 muxing"]
pub type Tmr1MuxR = crate::FieldReader;
#[doc = "Field `TMR1_MUX` writer - TMR1 muxing"]
pub type Tmr1MuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR2_MUX` reader - TMR2 muxing"]
pub type Tmr2MuxR = crate::FieldReader;
#[doc = "Field `TMR2_MUX` writer - TMR2 muxing"]
pub type Tmr2MuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR3_MUX` reader - TMR3 muxing"]
pub type Tmr3MuxR = crate::FieldReader;
#[doc = "Field `TMR3_MUX` writer - TMR3 muxing"]
pub type Tmr3MuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAN_MUX` reader - CAN1 muxing"]
pub type CanMuxR = crate::FieldReader;
#[doc = "Field `CAN_MUX` writer - CAN1 muxing"]
pub type CanMuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD01_MUX` reader - PD0/PD1 muxing on OSCIN/OSCOUT"]
pub type Pd01MuxR = crate::BitReader;
#[doc = "Field `PD01_MUX` writer - PD0/PD1 muxing on OSCIN/OSCOUT"]
pub type Pd01MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5CH4_MUX` reader - TMR5 channel4 internal muxing"]
pub type Tmr5ch4MuxR = crate::BitReader;
#[doc = "Field `TMR5CH4_MUX` writer - TMR5 channel4 internal muxing"]
pub type Tmr5ch4MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETP_MUX` reader - ADC1 external trigger preempted conversion muxing"]
pub type Adc1EtpMuxR = crate::BitReader;
#[doc = "Field `ADC1_ETP_MUX` writer - ADC1 external trigger preempted conversion muxing"]
pub type Adc1EtpMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETO_MUX` reader - ADC1 external trigger ordinary conversion muxing"]
pub type Adc1EtoMuxR = crate::BitReader;
#[doc = "Field `ADC1_ETO_MUX` writer - ADC1 external trigger ordinary conversion muxing"]
pub type Adc1EtoMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETP_MUX` reader - ADC2 external trigger preempted conversion muxing"]
pub type Adc2EtpMuxR = crate::BitReader;
#[doc = "Field `ADC2_ETP_MUX` writer - ADC2 external trigger preempted conversion muxing"]
pub type Adc2EtpMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETO_MUX` reader - ADC2 external trigger ordinary conversion muxing"]
pub type Adc2EtoMuxR = crate::BitReader;
#[doc = "Field `ADC2_ETO_MUX` writer - ADC2 external trigger ordinary conversion muxing"]
pub type Adc2EtoMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWJTAG_MUX` reader - SWD JTAG muxing"]
pub type SwjtagMuxR = crate::FieldReader;
#[doc = "Field `SWJTAG_MUX` writer - SWD JTAG muxing"]
pub type SwjtagMuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI1_MUX1` reader - SPI1 muxing bit1"]
pub type Spi1Mux1R = crate::BitReader;
#[doc = "Field `SPI1_MUX1` writer - SPI1 muxing bit1"]
pub type Spi1Mux1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI1 muxing bit0"]
    #[inline(always)]
    pub fn spi1_mux0(&self) -> Spi1Mux0R {
        Spi1Mux0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_mux(&self) -> I2c1MuxR {
        I2c1MuxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_mux(&self) -> Usart1MuxR {
        Usart1MuxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_mux(&self) -> Usart3MuxR {
        Usart3MuxR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_mux(&self) -> Tmr1MuxR {
        Tmr1MuxR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_mux(&self) -> Tmr2MuxR {
        Tmr2MuxR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_mux(&self) -> Tmr3MuxR {
        Tmr3MuxR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 13:14 - CAN1 muxing"]
    #[inline(always)]
    pub fn can_mux(&self) -> CanMuxR {
        CanMuxR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - PD0/PD1 muxing on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_mux(&self) -> Pd01MuxR {
        Pd01MuxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    pub fn tmr5ch4_mux(&self) -> Tmr5ch4MuxR {
        Tmr5ch4MuxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc1_etp_mux(&self) -> Adc1EtpMuxR {
        Adc1EtpMuxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc1_eto_mux(&self) -> Adc1EtoMuxR {
        Adc1EtoMuxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc2_etp_mux(&self) -> Adc2EtpMuxR {
        Adc2EtpMuxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc2_eto_mux(&self) -> Adc2EtoMuxR {
        Adc2EtoMuxR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SWD JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_mux(&self) -> SwjtagMuxR {
        SwjtagMuxR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - SPI1 muxing bit1"]
    #[inline(always)]
    pub fn spi1_mux1(&self) -> Spi1Mux1R {
        Spi1Mux1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP")
            .field("spi1_mux0", &self.spi1_mux0())
            .field("i2c1_mux", &self.i2c1_mux())
            .field("usart1_mux", &self.usart1_mux())
            .field("usart3_mux", &self.usart3_mux())
            .field("tmr1_mux", &self.tmr1_mux())
            .field("tmr2_mux", &self.tmr2_mux())
            .field("tmr3_mux", &self.tmr3_mux())
            .field("can_mux", &self.can_mux())
            .field("pd01_mux", &self.pd01_mux())
            .field("tmr5ch4_mux", &self.tmr5ch4_mux())
            .field("adc1_etp_mux", &self.adc1_etp_mux())
            .field("adc1_eto_mux", &self.adc1_eto_mux())
            .field("adc2_etp_mux", &self.adc2_etp_mux())
            .field("adc2_eto_mux", &self.adc2_eto_mux())
            .field("swjtag_mux", &self.swjtag_mux())
            .field("spi1_mux1", &self.spi1_mux1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 muxing bit0"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_mux0(&mut self) -> Spi1Mux0W<RemapSpec> {
        Spi1Mux0W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_mux(&mut self) -> I2c1MuxW<RemapSpec> {
        I2c1MuxW::new(self, 1)
    }
    #[doc = "Bit 2 - USART1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_mux(&mut self) -> Usart1MuxW<RemapSpec> {
        Usart1MuxW::new(self, 2)
    }
    #[doc = "Bits 4:5 - USART3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_mux(&mut self) -> Usart3MuxW<RemapSpec> {
        Usart3MuxW::new(self, 4)
    }
    #[doc = "Bits 6:7 - TMR1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_mux(&mut self) -> Tmr1MuxW<RemapSpec> {
        Tmr1MuxW::new(self, 6)
    }
    #[doc = "Bits 8:9 - TMR2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_mux(&mut self) -> Tmr2MuxW<RemapSpec> {
        Tmr2MuxW::new(self, 8)
    }
    #[doc = "Bits 10:11 - TMR3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_mux(&mut self) -> Tmr3MuxW<RemapSpec> {
        Tmr3MuxW::new(self, 10)
    }
    #[doc = "Bits 13:14 - CAN1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn can_mux(&mut self) -> CanMuxW<RemapSpec> {
        CanMuxW::new(self, 13)
    }
    #[doc = "Bit 15 - PD0/PD1 muxing on OSCIN/OSCOUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_mux(&mut self) -> Pd01MuxW<RemapSpec> {
        Pd01MuxW::new(self, 15)
    }
    #[doc = "Bit 16 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5ch4_mux(&mut self) -> Tmr5ch4MuxW<RemapSpec> {
        Tmr5ch4MuxW::new(self, 16)
    }
    #[doc = "Bit 17 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etp_mux(&mut self) -> Adc1EtpMuxW<RemapSpec> {
        Adc1EtpMuxW::new(self, 17)
    }
    #[doc = "Bit 18 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_eto_mux(&mut self) -> Adc1EtoMuxW<RemapSpec> {
        Adc1EtoMuxW::new(self, 18)
    }
    #[doc = "Bit 19 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_etp_mux(&mut self) -> Adc2EtpMuxW<RemapSpec> {
        Adc2EtpMuxW::new(self, 19)
    }
    #[doc = "Bit 20 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_eto_mux(&mut self) -> Adc2EtoMuxW<RemapSpec> {
        Adc2EtoMuxW::new(self, 20)
    }
    #[doc = "Bits 24:26 - SWD JTAG muxing"]
    #[inline(always)]
    #[must_use]
    pub fn swjtag_mux(&mut self) -> SwjtagMuxW<RemapSpec> {
        SwjtagMuxW::new(self, 24)
    }
    #[doc = "Bit 31 - SPI1 muxing bit1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_mux1(&mut self) -> Spi1Mux1W<RemapSpec> {
        Spi1Mux1W::new(self, 31)
    }
}
#[doc = "IO MUX remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemapSpec;
impl crate::RegisterSpec for RemapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap::R`](R) reader structure"]
impl crate::Readable for RemapSpec {}
#[doc = "`write(|w| ..)` method takes [`remap::W`](W) writer structure"]
impl crate::Writable for RemapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP to value 0"]
impl crate::Resettable for RemapSpec {
    const RESET_VALUE: u32 = 0;
}
