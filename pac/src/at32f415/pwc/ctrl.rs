#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `VRSEL` reader - Voltage regulator state select when deepsleep mode"]
pub type VrselR = crate::BitReader;
#[doc = "Field `VRSEL` writer - Voltage regulator state select when deepsleep mode"]
pub type VrselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPSEL` reader - Low power mode select when Cortex-M4F sleepdeep"]
pub type LpselR = crate::BitReader;
#[doc = "Field `LPSEL` writer - Low power mode select when Cortex-M4F sleepdeep"]
pub type LpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLSWEF` reader - Clear SWEF flag"]
pub type ClswefR = crate::BitReader;
#[doc = "Field `CLSWEF` writer - Clear SWEF flag"]
pub type ClswefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLSEF` reader - Clear SEF flag"]
pub type ClsefR = crate::BitReader;
#[doc = "Field `CLSEF` writer - Clear SEF flag"]
pub type ClsefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVMEN` reader - Power voltage monitoring enable"]
pub type PvmenR = crate::BitReader;
#[doc = "Field `PVMEN` writer - Power voltage monitoring enable"]
pub type PvmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVMSEL` reader - Power voltage monitoring boundary select"]
pub type PvmselR = crate::FieldReader;
#[doc = "Field `PVMSEL` writer - Power voltage monitoring boundary select"]
pub type PvmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BPWEN` reader - Battery powered domain write enable"]
pub type BpwenR = crate::BitReader;
#[doc = "Field `BPWEN` writer - Battery powered domain write enable"]
pub type BpwenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Voltage regulator state select when deepsleep mode"]
    #[inline(always)]
    pub fn vrsel(&self) -> VrselR {
        VrselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low power mode select when Cortex-M4F sleepdeep"]
    #[inline(always)]
    pub fn lpsel(&self) -> LpselR {
        LpselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear SWEF flag"]
    #[inline(always)]
    pub fn clswef(&self) -> ClswefR {
        ClswefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear SEF flag"]
    #[inline(always)]
    pub fn clsef(&self) -> ClsefR {
        ClsefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power voltage monitoring enable"]
    #[inline(always)]
    pub fn pvmen(&self) -> PvmenR {
        PvmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Power voltage monitoring boundary select"]
    #[inline(always)]
    pub fn pvmsel(&self) -> PvmselR {
        PvmselR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Battery powered domain write enable"]
    #[inline(always)]
    pub fn bpwen(&self) -> BpwenR {
        BpwenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("vrsel", &self.vrsel())
            .field("lpsel", &self.lpsel())
            .field("clswef", &self.clswef())
            .field("clsef", &self.clsef())
            .field("pvmen", &self.pvmen())
            .field("pvmsel", &self.pvmsel())
            .field("bpwen", &self.bpwen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Voltage regulator state select when deepsleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn vrsel(&mut self) -> VrselW<CtrlSpec> {
        VrselW::new(self, 0)
    }
    #[doc = "Bit 1 - Low power mode select when Cortex-M4F sleepdeep"]
    #[inline(always)]
    #[must_use]
    pub fn lpsel(&mut self) -> LpselW<CtrlSpec> {
        LpselW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear SWEF flag"]
    #[inline(always)]
    #[must_use]
    pub fn clswef(&mut self) -> ClswefW<CtrlSpec> {
        ClswefW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear SEF flag"]
    #[inline(always)]
    #[must_use]
    pub fn clsef(&mut self) -> ClsefW<CtrlSpec> {
        ClsefW::new(self, 3)
    }
    #[doc = "Bit 4 - Power voltage monitoring enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvmen(&mut self) -> PvmenW<CtrlSpec> {
        PvmenW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Power voltage monitoring boundary select"]
    #[inline(always)]
    #[must_use]
    pub fn pvmsel(&mut self) -> PvmselW<CtrlSpec> {
        PvmselW::new(self, 5)
    }
    #[doc = "Bit 8 - Battery powered domain write enable"]
    #[inline(always)]
    #[must_use]
    pub fn bpwen(&mut self) -> BpwenW<CtrlSpec> {
        BpwenW::new(self, 8)
    }
}
#[doc = "Power control register (PWC_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
