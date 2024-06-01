#[doc = "Register `STCTRL` reader"]
pub type R = crate::R<StctrlSpec>;
#[doc = "Register `STCTRL` writer"]
pub type W = crate::W<StctrlSpec>;
#[doc = "Field `SMSEL` reader - Subordinate TMR mode selection"]
pub type SmselR = crate::FieldReader;
#[doc = "Field `SMSEL` writer - Subordinate TMR mode selection"]
pub type SmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STIS` reader - Subordinate TMR input selection"]
pub type StisR = crate::FieldReader;
#[doc = "Field `STIS` writer - Subordinate TMR input selection"]
pub type StisW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STS` reader - Subordinate TMR synchronization"]
pub type StsR = crate::BitReader;
#[doc = "Field `STS` writer - Subordinate TMR synchronization"]
pub type StsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Subordinate TMR mode selection"]
    #[inline(always)]
    pub fn smsel(&self) -> SmselR {
        SmselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Subordinate TMR input selection"]
    #[inline(always)]
    pub fn stis(&self) -> StisR {
        StisR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Subordinate TMR synchronization"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCTRL")
            .field("sts", &self.sts())
            .field("stis", &self.stis())
            .field("smsel", &self.smsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Subordinate TMR mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn smsel(&mut self) -> SmselW<StctrlSpec> {
        SmselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Subordinate TMR input selection"]
    #[inline(always)]
    #[must_use]
    pub fn stis(&mut self) -> StisW<StctrlSpec> {
        StisW::new(self, 4)
    }
    #[doc = "Bit 7 - Subordinate TMR synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn sts(&mut self) -> StsW<StctrlSpec> {
        StsW::new(self, 7)
    }
}
#[doc = "Subordinate TMR control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StctrlSpec;
impl crate::RegisterSpec for StctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stctrl::R`](R) reader structure"]
impl crate::Readable for StctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`stctrl::W`](W) writer structure"]
impl crate::Writable for StctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCTRL to value 0"]
impl crate::Resettable for StctrlSpec {
    const RESET_VALUE: u32 = 0;
}
