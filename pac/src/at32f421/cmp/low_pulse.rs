#[doc = "Register `LOW_PULSE` reader"]
pub type R = crate::R<LowPulseSpec>;
#[doc = "Register `LOW_PULSE` writer"]
pub type W = crate::W<LowPulseSpec>;
#[doc = "Field `L_PULSE_CNT` reader - Low pulse Count"]
pub type LPulseCntR = crate::FieldReader;
#[doc = "Field `L_PULSE_CNT` writer - Low pulse Count"]
pub type LPulseCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Low pulse Count"]
    #[inline(always)]
    pub fn l_pulse_cnt(&self) -> LPulseCntR {
        LPulseCntR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOW_PULSE")
            .field("l_pulse_cnt", &self.l_pulse_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Low pulse Count"]
    #[inline(always)]
    #[must_use]
    pub fn l_pulse_cnt(&mut self) -> LPulseCntW<LowPulseSpec> {
        LPulseCntW::new(self, 0)
    }
}
#[doc = "LOW_PULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_pulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low_pulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowPulseSpec;
impl crate::RegisterSpec for LowPulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`low_pulse::R`](R) reader structure"]
impl crate::Readable for LowPulseSpec {}
#[doc = "`write(|w| ..)` method takes [`low_pulse::W`](W) writer structure"]
impl crate::Writable for LowPulseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOW_PULSE to value 0"]
impl crate::Resettable for LowPulseSpec {
    const RESET_VALUE: u32 = 0;
}
