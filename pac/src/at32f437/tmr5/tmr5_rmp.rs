#[doc = "Register `TMR5_RMP` reader"]
pub type R = crate::R<Tmr5RmpSpec>;
#[doc = "Register `TMR5_RMP` writer"]
pub type W = crate::W<Tmr5RmpSpec>;
#[doc = "Field `TMR5_CH4_IRMP` reader - TMR5 channel 4 input remap"]
pub type Tmr5Ch4IrmpR = crate::FieldReader;
#[doc = "Field `TMR5_CH4_IRMP` writer - TMR5 channel 4 input remap"]
pub type Tmr5Ch4IrmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:7 - TMR5 channel 4 input remap"]
    #[inline(always)]
    pub fn tmr5_ch4_irmp(&self) -> Tmr5Ch4IrmpR {
        Tmr5Ch4IrmpR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR5_RMP")
            .field("tmr5_ch4_irmp", &self.tmr5_ch4_irmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 6:7 - TMR5 channel 4 input remap"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5_ch4_irmp(&mut self) -> Tmr5Ch4IrmpW<Tmr5RmpSpec> {
        Tmr5Ch4IrmpW::new(self, 6)
    }
}
#[doc = "TMR5 channel input remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr5_rmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr5_rmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr5RmpSpec;
impl crate::RegisterSpec for Tmr5RmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr5_rmp::R`](R) reader structure"]
impl crate::Readable for Tmr5RmpSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr5_rmp::W`](W) writer structure"]
impl crate::Writable for Tmr5RmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR5_RMP to value 0"]
impl crate::Resettable for Tmr5RmpSpec {
    const RESET_VALUE: u32 = 0;
}
