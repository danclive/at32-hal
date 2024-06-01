#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SLEEP_DEBUG` reader - SLEEP_DEBUG"]
pub type SleepDebugR = crate::BitReader;
#[doc = "Field `SLEEP_DEBUG` writer - SLEEP_DEBUG"]
pub type SleepDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPSLEEP_DEBUG` reader - DEEPSLEEP_DEBUG"]
pub type DeepsleepDebugR = crate::BitReader;
#[doc = "Field `DEEPSLEEP_DEBUG` writer - DEEPSLEEP_DEBUG"]
pub type DeepsleepDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STANDBY_DEBUG` reader - STANDBY_DEBUG"]
pub type StandbyDebugR = crate::BitReader;
#[doc = "Field `STANDBY_DEBUG` writer - STANDBY_DEBUG"]
pub type StandbyDebugW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SLEEP_DEBUG"]
    #[inline(always)]
    pub fn sleep_debug(&self) -> SleepDebugR {
        SleepDebugR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DEEPSLEEP_DEBUG"]
    #[inline(always)]
    pub fn deepsleep_debug(&self) -> DeepsleepDebugR {
        DeepsleepDebugR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STANDBY_DEBUG"]
    #[inline(always)]
    pub fn standby_debug(&self) -> StandbyDebugR {
        StandbyDebugR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("sleep_debug", &self.sleep_debug())
            .field("deepsleep_debug", &self.deepsleep_debug())
            .field("standby_debug", &self.standby_debug())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SLEEP_DEBUG"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_debug(&mut self) -> SleepDebugW<CtrlSpec> {
        SleepDebugW::new(self, 0)
    }
    #[doc = "Bit 1 - DEEPSLEEP_DEBUG"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep_debug(&mut self) -> DeepsleepDebugW<CtrlSpec> {
        DeepsleepDebugW::new(self, 1)
    }
    #[doc = "Bit 2 - STANDBY_DEBUG"]
    #[inline(always)]
    #[must_use]
    pub fn standby_debug(&mut self) -> StandbyDebugW<CtrlSpec> {
        StandbyDebugW::new(self, 2)
    }
}
#[doc = "DEBUG CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
