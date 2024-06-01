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
#[doc = "Field `ADCALINIT` reader - initialize A/D calibration"]
pub type AdcalinitR = crate::BitReader;
#[doc = "Field `ADCALINIT` writer - initialize A/D calibration"]
pub type AdcalinitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTALIGN` reader - Data alignment"]
pub type DtalignR = crate::BitReader;
#[doc = "Field `DTALIGN` writer - Data alignment"]
pub type DtalignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCTESEL_L` reader - Low bit of trigger event select for preempted channels conversion"]
pub type PcteselLR = crate::FieldReader;
#[doc = "Field `PCTESEL_L` writer - Low bit of trigger event select for preempted channels conversion"]
pub type PcteselLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PCTEN` reader - Trigger mode enable for preempted channels conversion"]
pub type PctenR = crate::BitReader;
#[doc = "Field `PCTEN` writer - Trigger mode enable for preempted channels conversion"]
pub type PctenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTESEL_L` reader - Low bit of trigger event select for ordinary channels conversion"]
pub type OcteselLR = crate::FieldReader;
#[doc = "Field `OCTESEL_L` writer - Low bit of trigger event select for ordinary channels conversion"]
pub type OcteselLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OCTEN` reader - Trigger mode enable for ordinary channels conversion"]
pub type OctenR = crate::BitReader;
#[doc = "Field `OCTEN` writer - Trigger mode enable for ordinary channels conversion"]
pub type OctenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSWTRG` reader - Conversion trigger by software of preempted channels"]
pub type PcswtrgR = crate::BitReader;
#[doc = "Field `PCSWTRG` writer - Conversion trigger by software of preempted channels"]
pub type PcswtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCSWTRG` reader - Conversion trigger by software of ordinary channels"]
pub type OcswtrgR = crate::BitReader;
#[doc = "Field `OCSWTRG` writer - Conversion trigger by software of ordinary channels"]
pub type OcswtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCTESEL_H` reader - High bit of trigger event select for preempted channels conversion"]
pub type PcteselHR = crate::BitReader;
#[doc = "Field `PCTESEL_H` writer - High bit of trigger event select for preempted channels conversion"]
pub type PcteselHW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - initialize A/D calibration"]
    #[inline(always)]
    pub fn adcalinit(&self) -> AdcalinitR {
        AdcalinitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dtalign(&self) -> DtalignR {
        DtalignR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Low bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    pub fn pctesel_l(&self) -> PcteselLR {
        PcteselLR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Trigger mode enable for preempted channels conversion"]
    #[inline(always)]
    pub fn pcten(&self) -> PctenR {
        PctenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Low bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    pub fn octesel_l(&self) -> OcteselLR {
        OcteselLR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Trigger mode enable for ordinary channels conversion"]
    #[inline(always)]
    pub fn octen(&self) -> OctenR {
        OctenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Conversion trigger by software of preempted channels"]
    #[inline(always)]
    pub fn pcswtrg(&self) -> PcswtrgR {
        PcswtrgR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Conversion trigger by software of ordinary channels"]
    #[inline(always)]
    pub fn ocswtrg(&self) -> OcswtrgR {
        OcswtrgR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - High bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    pub fn pctesel_h(&self) -> PcteselHR {
        PcteselHR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - High bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    pub fn octesel_h(&self) -> OcteselHR {
        OcteselHR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("octesel_h", &self.octesel_h())
            .field("pctesel_h", &self.pctesel_h())
            .field("ocswtrg", &self.ocswtrg())
            .field("pcswtrg", &self.pcswtrg())
            .field("octen", &self.octen())
            .field("octesel_l", &self.octesel_l())
            .field("pcten", &self.pcten())
            .field("pctesel_l", &self.pctesel_l())
            .field("dtalign", &self.dtalign())
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
    #[doc = "Bit 3 - initialize A/D calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcalinit(&mut self) -> AdcalinitW<Ctrl2Spec> {
        AdcalinitW::new(self, 3)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn dtalign(&mut self) -> DtalignW<Ctrl2Spec> {
        DtalignW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Low bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pctesel_l(&mut self) -> PcteselLW<Ctrl2Spec> {
        PcteselLW::new(self, 12)
    }
    #[doc = "Bit 15 - Trigger mode enable for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pcten(&mut self) -> PctenW<Ctrl2Spec> {
        PctenW::new(self, 15)
    }
    #[doc = "Bits 17:19 - Low bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octesel_l(&mut self) -> OcteselLW<Ctrl2Spec> {
        OcteselLW::new(self, 17)
    }
    #[doc = "Bit 20 - Trigger mode enable for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octen(&mut self) -> OctenW<Ctrl2Spec> {
        OctenW::new(self, 20)
    }
    #[doc = "Bit 21 - Conversion trigger by software of preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcswtrg(&mut self) -> PcswtrgW<Ctrl2Spec> {
        PcswtrgW::new(self, 21)
    }
    #[doc = "Bit 22 - Conversion trigger by software of ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocswtrg(&mut self) -> OcswtrgW<Ctrl2Spec> {
        OcswtrgW::new(self, 22)
    }
    #[doc = "Bit 24 - High bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pctesel_h(&mut self) -> PcteselHW<Ctrl2Spec> {
        PcteselHW::new(self, 24)
    }
    #[doc = "Bit 25 - High bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octesel_h(&mut self) -> OcteselHW<Ctrl2Spec> {
        OcteselHW::new(self, 25)
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
