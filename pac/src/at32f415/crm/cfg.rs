#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `SCLKSEL` reader - System clock select"]
pub type SclkselR = crate::FieldReader;
#[doc = "Field `SCLKSEL` writer - System clock select"]
pub type SclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCLKSTS` reader - System Clock select Status"]
pub type SclkstsR = crate::FieldReader;
#[doc = "Field `AHBDIV` reader - AHB division"]
pub type AhbdivR = crate::FieldReader;
#[doc = "Field `AHBDIV` writer - AHB division"]
pub type AhbdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `APB1DIV` reader - APB1 division"]
pub type Apb1divR = crate::FieldReader;
#[doc = "Field `APB1DIV` writer - APB1 division"]
pub type Apb1divW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `APB2DIV` reader - APB2 division"]
pub type Apb2divR = crate::FieldReader;
#[doc = "Field `APB2DIV` writer - APB2 division"]
pub type Apb2divW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADCDIV1_0` reader - ADC division bit1 and bit0"]
pub type Adcdiv1_0R = crate::FieldReader;
#[doc = "Field `ADCDIV1_0` writer - ADC division bit1 and bit0"]
pub type Adcdiv1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLRCS` reader - PLL reference clock select"]
pub type PllrcsR = crate::BitReader;
#[doc = "Field `PLLRCS` writer - PLL reference clock select"]
pub type PllrcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLHEXTDIV` reader - HEXT division selection for PLL entry clock"]
pub type PllhextdivR = crate::BitReader;
#[doc = "Field `PLLHEXTDIV` writer - HEXT division selection for PLL entry clock"]
pub type PllhextdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLMULT3_0` reader - PLL Multiplication Factor bit3 to bit0"]
pub type Pllmult3_0R = crate::FieldReader;
#[doc = "Field `PLLMULT3_0` writer - PLL Multiplication Factor bit3 to bit0"]
pub type Pllmult3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USBDIV1_0` reader - USB division bit1 and bit0"]
pub type Usbdiv1_0R = crate::FieldReader;
#[doc = "Field `USBDIV1_0` writer - USB division bit1 and bit0"]
pub type Usbdiv1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKOUT_SEL` reader - Clock output selection bit2 to bit0"]
pub type ClkoutSelR = crate::FieldReader;
#[doc = "Field `CLKOUT_SEL` writer - Clock output selection bit2 to bit0"]
pub type ClkoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USBDIV2` reader - USB division bit2"]
pub type Usbdiv2R = crate::BitReader;
#[doc = "Field `USBDIV2` writer - USB division bit2"]
pub type Usbdiv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDIV2` reader - ADC division bit2"]
pub type Adcdiv2R = crate::BitReader;
#[doc = "Field `ADCDIV2` writer - ADC division bit2"]
pub type Adcdiv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLMULT5_4` reader - PLL Multiplication Factor bit5 and bit4"]
pub type Pllmult5_4R = crate::FieldReader;
#[doc = "Field `PLLMULT5_4` writer - PLL Multiplication Factor bit5 and bit4"]
pub type Pllmult5_4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    pub fn sclksel(&self) -> SclkselR {
        SclkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock select Status"]
    #[inline(always)]
    pub fn sclksts(&self) -> SclkstsR {
        SclkstsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    pub fn ahbdiv(&self) -> AhbdivR {
        AhbdivR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB1 division"]
    #[inline(always)]
    pub fn apb1div(&self) -> Apb1divR {
        Apb1divR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB2 division"]
    #[inline(always)]
    pub fn apb2div(&self) -> Apb2divR {
        Apb2divR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - ADC division bit1 and bit0"]
    #[inline(always)]
    pub fn adcdiv1_0(&self) -> Adcdiv1_0R {
        Adcdiv1_0R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL reference clock select"]
    #[inline(always)]
    pub fn pllrcs(&self) -> PllrcsR {
        PllrcsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HEXT division selection for PLL entry clock"]
    #[inline(always)]
    pub fn pllhextdiv(&self) -> PllhextdivR {
        PllhextdivR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor bit3 to bit0"]
    #[inline(always)]
    pub fn pllmult3_0(&self) -> Pllmult3_0R {
        Pllmult3_0R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - USB division bit1 and bit0"]
    #[inline(always)]
    pub fn usbdiv1_0(&self) -> Usbdiv1_0R {
        Usbdiv1_0R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Clock output selection bit2 to bit0"]
    #[inline(always)]
    pub fn clkout_sel(&self) -> ClkoutSelR {
        ClkoutSelR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - USB division bit2"]
    #[inline(always)]
    pub fn usbdiv2(&self) -> Usbdiv2R {
        Usbdiv2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC division bit2"]
    #[inline(always)]
    pub fn adcdiv2(&self) -> Adcdiv2R {
        Adcdiv2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - PLL Multiplication Factor bit5 and bit4"]
    #[inline(always)]
    pub fn pllmult5_4(&self) -> Pllmult5_4R {
        Pllmult5_4R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("sclksel", &self.sclksel())
            .field("sclksts", &self.sclksts())
            .field("ahbdiv", &self.ahbdiv())
            .field("apb1div", &self.apb1div())
            .field("apb2div", &self.apb2div())
            .field("adcdiv1_0", &self.adcdiv1_0())
            .field("pllrcs", &self.pllrcs())
            .field("pllhextdiv", &self.pllhextdiv())
            .field("pllmult3_0", &self.pllmult3_0())
            .field("usbdiv1_0", &self.usbdiv1_0())
            .field("clkout_sel", &self.clkout_sel())
            .field("usbdiv2", &self.usbdiv2())
            .field("adcdiv2", &self.adcdiv2())
            .field("pllmult5_4", &self.pllmult5_4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    #[must_use]
    pub fn sclksel(&mut self) -> SclkselW<CfgSpec> {
        SclkselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdiv(&mut self) -> AhbdivW<CfgSpec> {
        AhbdivW::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB1 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb1div(&mut self) -> Apb1divW<CfgSpec> {
        Apb1divW::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB2 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb2div(&mut self) -> Apb2divW<CfgSpec> {
        Apb2divW::new(self, 11)
    }
    #[doc = "Bits 14:15 - ADC division bit1 and bit0"]
    #[inline(always)]
    #[must_use]
    pub fn adcdiv1_0(&mut self) -> Adcdiv1_0W<CfgSpec> {
        Adcdiv1_0W::new(self, 14)
    }
    #[doc = "Bit 16 - PLL reference clock select"]
    #[inline(always)]
    #[must_use]
    pub fn pllrcs(&mut self) -> PllrcsW<CfgSpec> {
        PllrcsW::new(self, 16)
    }
    #[doc = "Bit 17 - HEXT division selection for PLL entry clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllhextdiv(&mut self) -> PllhextdivW<CfgSpec> {
        PllhextdivW::new(self, 17)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor bit3 to bit0"]
    #[inline(always)]
    #[must_use]
    pub fn pllmult3_0(&mut self) -> Pllmult3_0W<CfgSpec> {
        Pllmult3_0W::new(self, 18)
    }
    #[doc = "Bits 22:23 - USB division bit1 and bit0"]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv1_0(&mut self) -> Usbdiv1_0W<CfgSpec> {
        Usbdiv1_0W::new(self, 22)
    }
    #[doc = "Bits 24:26 - Clock output selection bit2 to bit0"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_sel(&mut self) -> ClkoutSelW<CfgSpec> {
        ClkoutSelW::new(self, 24)
    }
    #[doc = "Bit 27 - USB division bit2"]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv2(&mut self) -> Usbdiv2W<CfgSpec> {
        Usbdiv2W::new(self, 27)
    }
    #[doc = "Bit 28 - ADC division bit2"]
    #[inline(always)]
    #[must_use]
    pub fn adcdiv2(&mut self) -> Adcdiv2W<CfgSpec> {
        Adcdiv2W::new(self, 28)
    }
    #[doc = "Bits 29:30 - PLL Multiplication Factor bit5 and bit4"]
    #[inline(always)]
    #[must_use]
    pub fn pllmult5_4(&mut self) -> Pllmult5_4W<CfgSpec> {
        Pllmult5_4W::new(self, 29)
    }
}
#[doc = "Clock configuration register (CRM_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
