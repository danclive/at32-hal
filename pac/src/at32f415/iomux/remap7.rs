#[doc = "Register `REMAP7` reader"]
pub type R = crate::R<Remap7Spec>;
#[doc = "Register `REMAP7` writer"]
pub type W = crate::W<Remap7Spec>;
#[doc = "Field `ADC1_ETP_GMUX` reader - ADC1 external trigger preempted conversion muxing"]
pub type Adc1EtpGmuxR = crate::BitReader;
#[doc = "Field `ADC1_ETP_GMUX` writer - ADC1 external trigger preempted conversion muxing"]
pub type Adc1EtpGmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETO_GMUX` reader - ADC1 external trigger ordinary conversion muxing"]
pub type Adc1EtoGmuxR = crate::BitReader;
#[doc = "Field `ADC1_ETO_GMUX` writer - ADC1 external trigger ordinary conversion muxing"]
pub type Adc1EtoGmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWJTAG_GMUX` reader - Serial wire JTAG muxing"]
pub type SwjtagGmuxR = crate::FieldReader;
#[doc = "Field `SWJTAG_GMUX` writer - Serial wire JTAG muxing"]
pub type SwjtagGmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD01_GMUX` reader - PortD0/PortD1 muxing on OSC_IN/OSC_OUT"]
pub type Pd01GmuxR = crate::BitReader;
#[doc = "Field `PD01_GMUX` writer - PortD0/PortD1 muxing on OSC_IN/OSC_OUT"]
pub type Pd01GmuxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc1_etp_gmux(&self) -> Adc1EtpGmuxR {
        Adc1EtpGmuxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc1_eto_gmux(&self) -> Adc1EtoGmuxR {
        Adc1EtoGmuxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Serial wire JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_gmux(&self) -> SwjtagGmuxR {
        SwjtagGmuxR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - PortD0/PortD1 muxing on OSC_IN/OSC_OUT"]
    #[inline(always)]
    pub fn pd01_gmux(&self) -> Pd01GmuxR {
        Pd01GmuxR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP7")
            .field("pd01_gmux", &self.pd01_gmux())
            .field("swjtag_gmux", &self.swjtag_gmux())
            .field("adc1_eto_gmux", &self.adc1_eto_gmux())
            .field("adc1_etp_gmux", &self.adc1_etp_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etp_gmux(&mut self) -> Adc1EtpGmuxW<Remap7Spec> {
        Adc1EtpGmuxW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_eto_gmux(&mut self) -> Adc1EtoGmuxW<Remap7Spec> {
        Adc1EtoGmuxW::new(self, 5)
    }
    #[doc = "Bits 16:18 - Serial wire JTAG muxing"]
    #[inline(always)]
    #[must_use]
    pub fn swjtag_gmux(&mut self) -> SwjtagGmuxW<Remap7Spec> {
        SwjtagGmuxW::new(self, 16)
    }
    #[doc = "Bit 20 - PortD0/PortD1 muxing on OSC_IN/OSC_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_gmux(&mut self) -> Pd01GmuxW<Remap7Spec> {
        Pd01GmuxW::new(self, 20)
    }
}
#[doc = "IO MUX remap register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap7Spec;
impl crate::RegisterSpec for Remap7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap7::R`](R) reader structure"]
impl crate::Readable for Remap7Spec {}
#[doc = "`write(|w| ..)` method takes [`remap7::W`](W) writer structure"]
impl crate::Writable for Remap7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP7 to value 0"]
impl crate::Resettable for Remap7Spec {
    const RESET_VALUE: u32 = 0;
}
