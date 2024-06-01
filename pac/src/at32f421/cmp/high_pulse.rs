#[doc = "Register `HIGH_PULSE` reader"]
pub type R = crate::R<HighPulseSpec>;
#[doc = "Register `HIGH_PULSE` writer"]
pub type W = crate::W<HighPulseSpec>;
#[doc = "Field `H_PULSE_CNT` reader - High pulse Count"]
pub type HPulseCntR = crate::FieldReader;
#[doc = "Field `H_PULSE_CNT` writer - High pulse Count"]
pub type HPulseCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - High pulse Count"]
    #[inline(always)]
    pub fn h_pulse_cnt(&self) -> HPulseCntR {
        HPulseCntR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIGH_PULSE")
            .field("h_pulse_cnt", &self.h_pulse_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - High pulse Count"]
    #[inline(always)]
    #[must_use]
    pub fn h_pulse_cnt(&mut self) -> HPulseCntW<HighPulseSpec> {
        HPulseCntW::new(self, 0)
    }
}
#[doc = "HIGH_PULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high_pulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high_pulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HighPulseSpec;
impl crate::RegisterSpec for HighPulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`high_pulse::R`](R) reader structure"]
impl crate::Readable for HighPulseSpec {}
#[doc = "`write(|w| ..)` method takes [`high_pulse::W`](W) writer structure"]
impl crate::Writable for HighPulseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIGH_PULSE to value 0"]
impl crate::Resettable for HighPulseSpec {
    const RESET_VALUE: u32 = 0;
}
