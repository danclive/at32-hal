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
#[doc = "Field `ERTCDIV` reader - HEXT division for ERTC clock"]
pub type ErtcdivR = crate::FieldReader;
#[doc = "Field `ERTCDIV` writer - HEXT division for ERTC clock"]
pub type ErtcdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLKOUT1_SEL` reader - Clock output1 selection"]
pub type Clkout1SelR = crate::FieldReader;
#[doc = "Field `CLKOUT1_SEL` writer - Clock output1 selection"]
pub type Clkout1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKOUT1DIV1` reader - Clock output1 division1"]
pub type Clkout1div1R = crate::FieldReader;
#[doc = "Field `CLKOUT1DIV1` writer - Clock output1 division1"]
pub type Clkout1div1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLKOUT2DIV1` reader - Clock output2 division1"]
pub type Clkout2div1R = crate::FieldReader;
#[doc = "Field `CLKOUT2DIV1` writer - Clock output2 division1"]
pub type Clkout2div1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLKOUT2_SEL1` reader - Clock output2 selection1"]
pub type Clkout2Sel1R = crate::FieldReader;
#[doc = "Field `CLKOUT2_SEL1` writer - Clock output2 selection1"]
pub type Clkout2Sel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 10:12 - APB1 division"]
    #[inline(always)]
    pub fn apb1div(&self) -> Apb1divR {
        Apb1divR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - APB2 division"]
    #[inline(always)]
    pub fn apb2div(&self) -> Apb2divR {
        Apb2divR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - HEXT division for ERTC clock"]
    #[inline(always)]
    pub fn ertcdiv(&self) -> ErtcdivR {
        ErtcdivR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Clock output1 selection"]
    #[inline(always)]
    pub fn clkout1_sel(&self) -> Clkout1SelR {
        Clkout1SelR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Clock output1 division1"]
    #[inline(always)]
    pub fn clkout1div1(&self) -> Clkout1div1R {
        Clkout1div1R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Clock output2 division1"]
    #[inline(always)]
    pub fn clkout2div1(&self) -> Clkout2div1R {
        Clkout2div1R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Clock output2 selection1"]
    #[inline(always)]
    pub fn clkout2_sel1(&self) -> Clkout2Sel1R {
        Clkout2Sel1R::new(((self.bits >> 30) & 3) as u8)
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
            .field("ertcdiv", &self.ertcdiv())
            .field("clkout1_sel", &self.clkout1_sel())
            .field("clkout1div1", &self.clkout1div1())
            .field("clkout2div1", &self.clkout2div1())
            .field("clkout2_sel1", &self.clkout2_sel1())
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
    #[doc = "Bits 10:12 - APB1 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb1div(&mut self) -> Apb1divW<CfgSpec> {
        Apb1divW::new(self, 10)
    }
    #[doc = "Bits 13:15 - APB2 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb2div(&mut self) -> Apb2divW<CfgSpec> {
        Apb2divW::new(self, 13)
    }
    #[doc = "Bits 16:20 - HEXT division for ERTC clock"]
    #[inline(always)]
    #[must_use]
    pub fn ertcdiv(&mut self) -> ErtcdivW<CfgSpec> {
        ErtcdivW::new(self, 16)
    }
    #[doc = "Bits 21:22 - Clock output1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn clkout1_sel(&mut self) -> Clkout1SelW<CfgSpec> {
        Clkout1SelW::new(self, 21)
    }
    #[doc = "Bits 24:26 - Clock output1 division1"]
    #[inline(always)]
    #[must_use]
    pub fn clkout1div1(&mut self) -> Clkout1div1W<CfgSpec> {
        Clkout1div1W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Clock output2 division1"]
    #[inline(always)]
    #[must_use]
    pub fn clkout2div1(&mut self) -> Clkout2div1W<CfgSpec> {
        Clkout2div1W::new(self, 27)
    }
    #[doc = "Bits 30:31 - Clock output2 selection1"]
    #[inline(always)]
    #[must_use]
    pub fn clkout2_sel1(&mut self) -> Clkout2Sel1W<CfgSpec> {
        Clkout2Sel1W::new(self, 30)
    }
}
#[doc = "Clock configuration register(CRM_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
