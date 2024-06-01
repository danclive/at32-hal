#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `CALON` reader - Calibration on"]
pub type CalonR = crate::BitReader;
#[doc = "Field `CALON` writer - Calibration on"]
pub type CalonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTRIM` reader - Enable trim"]
pub type EntrimR = crate::BitReader;
#[doc = "Field `ENTRIM` writer - Enable trim"]
pub type EntrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIEN` reader - RSLOST error interrupt enable"]
pub type EienR = crate::BitReader;
#[doc = "Field `EIEN` writer - RSLOST error interrupt enable"]
pub type EienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDYIEN` reader - CALRDY interrupt enable"]
pub type CalrdyienR = crate::BitReader;
#[doc = "Field `CALRDYIEN` writer - CALRDY interrupt enable"]
pub type CalrdyienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP` reader - STEP"]
pub type StepR = crate::FieldReader;
#[doc = "Field `STEP` writer - STEP"]
pub type StepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Calibration on"]
    #[inline(always)]
    pub fn calon(&self) -> CalonR {
        CalonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable trim"]
    #[inline(always)]
    pub fn entrim(&self) -> EntrimR {
        EntrimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RSLOST error interrupt enable"]
    #[inline(always)]
    pub fn eien(&self) -> EienR {
        EienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CALRDY interrupt enable"]
    #[inline(always)]
    pub fn calrdyien(&self) -> CalrdyienR {
        CalrdyienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - STEP"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("step", &self.step())
            .field("calrdyien", &self.calrdyien())
            .field("eien", &self.eien())
            .field("entrim", &self.entrim())
            .field("calon", &self.calon())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Calibration on"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CalonW<Ctrl1Spec> {
        CalonW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable trim"]
    #[inline(always)]
    #[must_use]
    pub fn entrim(&mut self) -> EntrimW<Ctrl1Spec> {
        EntrimW::new(self, 1)
    }
    #[doc = "Bit 4 - RSLOST error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eien(&mut self) -> EienW<Ctrl1Spec> {
        EienW::new(self, 4)
    }
    #[doc = "Bit 5 - CALRDY interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn calrdyien(&mut self) -> CalrdyienW<Ctrl1Spec> {
        CalrdyienW::new(self, 5)
    }
    #[doc = "Bits 8:11 - STEP"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> StepW<Ctrl1Spec> {
        StepW::new(self, 8)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0x0100"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0x0100;
}
