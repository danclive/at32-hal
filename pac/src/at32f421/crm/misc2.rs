#[doc = "Register `MISC2` reader"]
pub type R = crate::R<Misc2Spec>;
#[doc = "Register `MISC2` writer"]
pub type W = crate::W<Misc2Spec>;
#[doc = "Field `AUTO_STEP_EN` reader - AUTO_STEP_EN"]
pub type AutoStepEnR = crate::FieldReader;
#[doc = "Field `AUTO_STEP_EN` writer - AUTO_STEP_EN"]
pub type AutoStepEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HICK_TO_SCLK` reader - HICK to system clock"]
pub type HickToSclkR = crate::BitReader;
#[doc = "Field `HICK_TO_SCLK` writer - HICK to system clock"]
pub type HickToSclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5 - AUTO_STEP_EN"]
    #[inline(always)]
    pub fn auto_step_en(&self) -> AutoStepEnR {
        AutoStepEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 9 - HICK to system clock"]
    #[inline(always)]
    pub fn hick_to_sclk(&self) -> HickToSclkR {
        HickToSclkR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC2")
            .field("auto_step_en", &self.auto_step_en())
            .field("hick_to_sclk", &self.hick_to_sclk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - AUTO_STEP_EN"]
    #[inline(always)]
    #[must_use]
    pub fn auto_step_en(&mut self) -> AutoStepEnW<Misc2Spec> {
        AutoStepEnW::new(self, 4)
    }
    #[doc = "Bit 9 - HICK to system clock"]
    #[inline(always)]
    #[must_use]
    pub fn hick_to_sclk(&mut self) -> HickToSclkW<Misc2Spec> {
        HickToSclkW::new(self, 9)
    }
}
#[doc = "Miscellaneous register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Misc2Spec;
impl crate::RegisterSpec for Misc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc2::R`](R) reader structure"]
impl crate::Readable for Misc2Spec {}
#[doc = "`write(|w| ..)` method takes [`misc2::W`](W) writer structure"]
impl crate::Writable for Misc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC2 to value 0x0d"]
impl crate::Resettable for Misc2Spec {
    const RESET_VALUE: u32 = 0x0d;
}
