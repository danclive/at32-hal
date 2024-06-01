#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CtrlstsSpec>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CtrlstsSpec>;
#[doc = "Field `SWEF` reader - Standby wake-up event flag"]
pub type SwefR = crate::BitReader;
#[doc = "Field `SEF` reader - Standby mode entry flag"]
pub type SefR = crate::BitReader;
#[doc = "Field `PVMOF` reader - Power voltage monitoring output flag"]
pub type PvmofR = crate::BitReader;
#[doc = "Field `SWPEN` reader - Standby wake-up pin enable"]
pub type SwpenR = crate::BitReader;
#[doc = "Field `SWPEN` writer - Standby wake-up pin enable"]
pub type SwpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Standby wake-up event flag"]
    #[inline(always)]
    pub fn swef(&self) -> SwefR {
        SwefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby mode entry flag"]
    #[inline(always)]
    pub fn sef(&self) -> SefR {
        SefR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power voltage monitoring output flag"]
    #[inline(always)]
    pub fn pvmof(&self) -> PvmofR {
        PvmofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby wake-up pin enable"]
    #[inline(always)]
    pub fn swpen(&self) -> SwpenR {
        SwpenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS")
            .field("swef", &self.swef())
            .field("sef", &self.sef())
            .field("pvmof", &self.pvmof())
            .field("swpen", &self.swpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Standby wake-up pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpen(&mut self) -> SwpenW<CtrlstsSpec> {
        SwpenW::new(self, 8)
    }
}
#[doc = "Power control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
