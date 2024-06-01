#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `ADCEN` reader - A/D converter enable"]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - A/D converter enable"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPEN` reader - Repeat mode enable"]
pub type RpenR = crate::BitReader;
#[doc = "Field `RPEN` writer - Repeat mode enable"]
pub type RpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCAL` reader - A/D Calibration"]
pub type AdcalR = crate::BitReader;
#[doc = "Field `ADCAL` writer - A/D Calibration"]
pub type AdcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCALINIT` reader - Initialize A/D calibration"]
pub type AdcalinitR = crate::BitReader;
#[doc = "Field `ADCALINIT` writer - Initialize A/D calibration"]
pub type AdcalinitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADABRT` reader - ADC conversion abort"]
pub type AdabrtR = crate::BitReader;
#[doc = "Field `ADABRT` writer - ADC conversion abort"]
pub type AdabrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCDMAEN` reader - Ordinary channel DMA transfer enable for independent mode"]
pub type OcdmaenR = crate::BitReader;
#[doc = "Field `OCDMAEN` writer - Ordinary channel DMA transfer enable for independent mode"]
pub type OcdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCDRCEN` reader - Ordinary channel DMA request continuation enable for independent mode"]
pub type OcdrcenR = crate::BitReader;
#[doc = "Field `OCDRCEN` writer - Ordinary channel DMA request continuation enable for independent mode"]
pub type OcdrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCSFEN` reader - Each ordinary channel conversion set OCCE flag enable"]
pub type EocsfenR = crate::BitReader;
#[doc = "Field `EOCSFEN` writer - Each ordinary channel conversion set OCCE flag enable"]
pub type EocsfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTALIGN` reader - Data alignment"]
pub type DtalignR = crate::BitReader;
#[doc = "Field `DTALIGN` writer - Data alignment"]
pub type DtalignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCTESEL_L` reader - Low bit of trigger event select for preempted channels conversion"]
pub type PcteselLR = crate::FieldReader;
#[doc = "Field `PCTESEL_L` writer - Low bit of trigger event select for preempted channels conversion"]
pub type PcteselLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PCETE` reader - Preempted channel external trigger edge select"]
pub type PceteR = crate::FieldReader;
#[doc = "Field `PCETE` writer - Preempted channel external trigger edge select"]
pub type PceteW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCSWTRG` reader - Preempted channel software conversion trigger"]
pub type PcswtrgR = crate::BitReader;
#[doc = "Field `PCSWTRG` writer - Preempted channel software conversion trigger"]
pub type PcswtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCTESEL_H` reader - High bit of trigger event select for preempted channels conversion"]
pub type PcteselHR = crate::BitReader;
#[doc = "Field `PCTESEL_H` writer - High bit of trigger event select for preempted channels conversion"]
pub type PcteselHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTESEL_L` reader - Low bit of trigger event select for ordinary channels conversion"]
pub type OcteselLR = crate::FieldReader;
#[doc = "Field `OCTESEL_L` writer - Low bit of trigger event select for ordinary channels conversion"]
pub type OcteselLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OCETE` reader - Ordinary channel external trigger edge select"]
pub type OceteR = crate::FieldReader;
#[doc = "Field `OCETE` writer - Ordinary channel external trigger edge select"]
pub type OceteW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OCSWTRG` reader - Ordinary channel software conversion trigger"]
pub type OcswtrgR = crate::BitReader;
#[doc = "Field `OCSWTRG` writer - Ordinary channel software conversion trigger"]
pub type OcswtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTESEL_H` reader - High bit of trigger event select for ordinary channels conversion"]
pub type OcteselHR = crate::BitReader;
#[doc = "Field `OCTESEL_H` writer - High bit of trigger event select for ordinary channels conversion"]
pub type OcteselHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - A/D converter enable"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeat mode enable"]
    #[inline(always)]
    pub fn rpen(&self) -> RpenR {
        RpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D Calibration"]
    #[inline(always)]
    pub fn adcal(&self) -> AdcalR {
        AdcalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Initialize A/D calibration"]
    #[inline(always)]
    pub fn adcalinit(&self) -> AdcalinitR {
        AdcalinitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC conversion abort"]
    #[inline(always)]
    pub fn adabrt(&self) -> AdabrtR {
        AdabrtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Ordinary channel DMA transfer enable for independent mode"]
    #[inline(always)]
    pub fn ocdmaen(&self) -> OcdmaenR {
        OcdmaenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ordinary channel DMA request continuation enable for independent mode"]
    #[inline(always)]
    pub fn ocdrcen(&self) -> OcdrcenR {
        OcdrcenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Each ordinary channel conversion set OCCE flag enable"]
    #[inline(always)]
    pub fn eocsfen(&self) -> EocsfenR {
        EocsfenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dtalign(&self) -> DtalignR {
        DtalignR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Low bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    pub fn pctesel_l(&self) -> PcteselLR {
        PcteselLR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Preempted channel external trigger edge select"]
    #[inline(always)]
    pub fn pcete(&self) -> PceteR {
        PceteR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Preempted channel software conversion trigger"]
    #[inline(always)]
    pub fn pcswtrg(&self) -> PcswtrgR {
        PcswtrgR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - High bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    pub fn pctesel_h(&self) -> PcteselHR {
        PcteselHR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Low bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    pub fn octesel_l(&self) -> OcteselLR {
        OcteselLR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Ordinary channel external trigger edge select"]
    #[inline(always)]
    pub fn ocete(&self) -> OceteR {
        OceteR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Ordinary channel software conversion trigger"]
    #[inline(always)]
    pub fn ocswtrg(&self) -> OcswtrgR {
        OcswtrgR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - High bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    pub fn octesel_h(&self) -> OcteselHR {
        OcteselHR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("octesel_h", &self.octesel_h())
            .field("ocswtrg", &self.ocswtrg())
            .field("ocete", &self.ocete())
            .field("octesel_l", &self.octesel_l())
            .field("pctesel_h", &self.pctesel_h())
            .field("pcswtrg", &self.pcswtrg())
            .field("pcete", &self.pcete())
            .field("pctesel_l", &self.pctesel_l())
            .field("dtalign", &self.dtalign())
            .field("eocsfen", &self.eocsfen())
            .field("ocdrcen", &self.ocdrcen())
            .field("ocdmaen", &self.ocdmaen())
            .field("adabrt", &self.adabrt())
            .field("adcalinit", &self.adcalinit())
            .field("adcal", &self.adcal())
            .field("rpen", &self.rpen())
            .field("adcen", &self.adcen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - A/D converter enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> AdcenW<Ctrl2Spec> {
        AdcenW::new(self, 0)
    }
    #[doc = "Bit 1 - Repeat mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpen(&mut self) -> RpenW<Ctrl2Spec> {
        RpenW::new(self, 1)
    }
    #[doc = "Bit 2 - A/D Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> AdcalW<Ctrl2Spec> {
        AdcalW::new(self, 2)
    }
    #[doc = "Bit 3 - Initialize A/D calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcalinit(&mut self) -> AdcalinitW<Ctrl2Spec> {
        AdcalinitW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC conversion abort"]
    #[inline(always)]
    #[must_use]
    pub fn adabrt(&mut self) -> AdabrtW<Ctrl2Spec> {
        AdabrtW::new(self, 4)
    }
    #[doc = "Bit 8 - Ordinary channel DMA transfer enable for independent mode"]
    #[inline(always)]
    #[must_use]
    pub fn ocdmaen(&mut self) -> OcdmaenW<Ctrl2Spec> {
        OcdmaenW::new(self, 8)
    }
    #[doc = "Bit 9 - Ordinary channel DMA request continuation enable for independent mode"]
    #[inline(always)]
    #[must_use]
    pub fn ocdrcen(&mut self) -> OcdrcenW<Ctrl2Spec> {
        OcdrcenW::new(self, 9)
    }
    #[doc = "Bit 10 - Each ordinary channel conversion set OCCE flag enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocsfen(&mut self) -> EocsfenW<Ctrl2Spec> {
        EocsfenW::new(self, 10)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn dtalign(&mut self) -> DtalignW<Ctrl2Spec> {
        DtalignW::new(self, 11)
    }
    #[doc = "Bits 16:19 - Low bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pctesel_l(&mut self) -> PcteselLW<Ctrl2Spec> {
        PcteselLW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Preempted channel external trigger edge select"]
    #[inline(always)]
    #[must_use]
    pub fn pcete(&mut self) -> PceteW<Ctrl2Spec> {
        PceteW::new(self, 20)
    }
    #[doc = "Bit 22 - Preempted channel software conversion trigger"]
    #[inline(always)]
    #[must_use]
    pub fn pcswtrg(&mut self) -> PcswtrgW<Ctrl2Spec> {
        PcswtrgW::new(self, 22)
    }
    #[doc = "Bit 23 - High bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pctesel_h(&mut self) -> PcteselHW<Ctrl2Spec> {
        PcteselHW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Low bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octesel_l(&mut self) -> OcteselLW<Ctrl2Spec> {
        OcteselLW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Ordinary channel external trigger edge select"]
    #[inline(always)]
    #[must_use]
    pub fn ocete(&mut self) -> OceteW<Ctrl2Spec> {
        OceteW::new(self, 28)
    }
    #[doc = "Bit 30 - Ordinary channel software conversion trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ocswtrg(&mut self) -> OcswtrgW<Ctrl2Spec> {
        OcswtrgW::new(self, 30)
    }
    #[doc = "Bit 31 - High bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octesel_h(&mut self) -> OcteselHW<Ctrl2Spec> {
        OcteselHW::new(self, 31)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0;
}
