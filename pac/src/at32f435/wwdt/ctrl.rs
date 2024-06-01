#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CNT` reader - Decrement counter"]
pub type CntR = crate::FieldReader;
#[doc = "Field `CNT` writer - Decrement counter"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WWDTEN` reader - Window watchdog enable"]
pub type WwdtenR = crate::BitReader;
#[doc = "Field `WWDTEN` writer - Window watchdog enable"]
pub type WwdtenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Decrement counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Window watchdog enable"]
    #[inline(always)]
    pub fn wwdten(&self) -> WwdtenR {
        WwdtenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("cnt", &self.cnt())
            .field("wwdten", &self.wwdten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Decrement counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<CtrlSpec> {
        CntW::new(self, 0)
    }
    #[doc = "Bit 7 - Window watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdten(&mut self) -> WwdtenW<CtrlSpec> {
        WwdtenW::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x7f"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x7f;
}
