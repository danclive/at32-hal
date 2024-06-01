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
#[doc = "Field `ESF` reader - External signal filter"]
pub type EsfR = crate::FieldReader;
#[doc = "Field `ESF` writer - External signal filter"]
pub type EsfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ESDIV` reader - External signal divider"]
pub type EsdivR = crate::FieldReader;
#[doc = "Field `ESDIV` writer - External signal divider"]
pub type EsdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECMBEN` reader - External clock mode B enable"]
pub type EcmbenR = crate::BitReader;
#[doc = "Field `ECMBEN` writer - External clock mode B enable"]
pub type EcmbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESP` reader - External signal polarity"]
pub type EspR = crate::BitReader;
#[doc = "Field `ESP` writer - External signal polarity"]
pub type EspW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 8:11 - External signal filter"]
    #[inline(always)]
    pub fn esf(&self) -> EsfR {
        EsfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External signal divider"]
    #[inline(always)]
    pub fn esdiv(&self) -> EsdivR {
        EsdivR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock mode B enable"]
    #[inline(always)]
    pub fn ecmben(&self) -> EcmbenR {
        EcmbenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External signal polarity"]
    #[inline(always)]
    pub fn esp(&self) -> EspR {
        EspR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCTRL")
            .field("esp", &self.esp())
            .field("ecmben", &self.ecmben())
            .field("esdiv", &self.esdiv())
            .field("esf", &self.esf())
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
    #[doc = "Bits 8:11 - External signal filter"]
    #[inline(always)]
    #[must_use]
    pub fn esf(&mut self) -> EsfW<StctrlSpec> {
        EsfW::new(self, 8)
    }
    #[doc = "Bits 12:13 - External signal divider"]
    #[inline(always)]
    #[must_use]
    pub fn esdiv(&mut self) -> EsdivW<StctrlSpec> {
        EsdivW::new(self, 12)
    }
    #[doc = "Bit 14 - External clock mode B enable"]
    #[inline(always)]
    #[must_use]
    pub fn ecmben(&mut self) -> EcmbenW<StctrlSpec> {
        EcmbenW::new(self, 14)
    }
    #[doc = "Bit 15 - External signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn esp(&mut self) -> EspW<StctrlSpec> {
        EspW::new(self, 15)
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
