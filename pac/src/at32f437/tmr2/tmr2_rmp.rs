#[doc = "Register `TMR2_RMP` reader"]
pub type R = crate::R<Tmr2RmpSpec>;
#[doc = "Register `TMR2_RMP` writer"]
pub type W = crate::W<Tmr2RmpSpec>;
#[doc = "Field `TMR2_CH1_IRMP` reader - TMR2 channel 1 input remap"]
pub type Tmr2Ch1IrmpR = crate::FieldReader;
#[doc = "Field `TMR2_CH1_IRMP` writer - TMR2 channel 1 input remap"]
pub type Tmr2Ch1IrmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 10:11 - TMR2 channel 1 input remap"]
    #[inline(always)]
    pub fn tmr2_ch1_irmp(&self) -> Tmr2Ch1IrmpR {
        Tmr2Ch1IrmpR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR2_RMP")
            .field("tmr2_ch1_irmp", &self.tmr2_ch1_irmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:11 - TMR2 channel 1 input remap"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_ch1_irmp(&mut self) -> Tmr2Ch1IrmpW<Tmr2RmpSpec> {
        Tmr2Ch1IrmpW::new(self, 10)
    }
}
#[doc = "TMR2 channel input remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr2_rmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr2_rmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr2RmpSpec;
impl crate::RegisterSpec for Tmr2RmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr2_rmp::R`](R) reader structure"]
impl crate::Readable for Tmr2RmpSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr2_rmp::W`](W) writer structure"]
impl crate::Writable for Tmr2RmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR2_RMP to value 0"]
impl crate::Resettable for Tmr2RmpSpec {
    const RESET_VALUE: u32 = 0;
}
