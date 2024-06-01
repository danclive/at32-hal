#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CtrlstsSpec>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CtrlstsSpec>;
#[doc = "Field `TPEFCLR` writer - Tamper event flag clear"]
pub type TpefclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIFCLR` writer - Tamper interrupt flag clear"]
pub type TpifclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIEN` reader - Tamper pin interrupt enable"]
pub type TpienR = crate::BitReader;
#[doc = "Field `TPIEN` writer - Tamper pin interrupt enable"]
pub type TpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPEF` reader - Tamper event flag"]
pub type TpefR = crate::BitReader;
#[doc = "Field `TPEF` writer - Tamper event flag"]
pub type TpefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIF` reader - Tamper interrupt flag"]
pub type TpifR = crate::BitReader;
#[doc = "Field `TPIF` writer - Tamper interrupt flag"]
pub type TpifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Tamper pin interrupt enable"]
    #[inline(always)]
    pub fn tpien(&self) -> TpienR {
        TpienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    pub fn tpef(&self) -> TpefR {
        TpefR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tpif(&self) -> TpifR {
        TpifR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS")
            .field("tpien", &self.tpien())
            .field("tpef", &self.tpef())
            .field("tpif", &self.tpif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Tamper event flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tpefclr(&mut self) -> TpefclrW<CtrlstsSpec> {
        TpefclrW::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tpifclr(&mut self) -> TpifclrW<CtrlstsSpec> {
        TpifclrW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper pin interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpien(&mut self) -> TpienW<CtrlstsSpec> {
        TpienW::new(self, 2)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    #[must_use]
    pub fn tpef(&mut self) -> TpefW<CtrlstsSpec> {
        TpefW::new(self, 8)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tpif(&mut self) -> TpifW<CtrlstsSpec> {
        TpifW::new(self, 9)
    }
}
#[doc = "BPR control/status register (BPR_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlstsSpec;
impl crate::RegisterSpec for CtrlstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CtrlstsSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CtrlstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLSTS to value 0"]
impl crate::Resettable for CtrlstsSpec {
    const RESET_VALUE: u32 = 0;
}
