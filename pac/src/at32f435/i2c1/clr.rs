#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `ADDRC` writer - Clear 0~7 bit address match flag"]
pub type AddrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAILC` writer - Clear acknowledge failure flag"]
pub type AckfailcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPC` writer - Clear stop condition generation complete flag"]
pub type StopcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERRC` writer - Clear bus error flag"]
pub type BuserrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOSTC` writer - Clear arbitration lost flag"]
pub type ArlostcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUFC` writer - Clear overload / underload flag"]
pub type OufcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECERRC` writer - Clear PEC receive error flag"]
pub type PecerrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOUTC` writer - Clear SMBus timeout flag"]
pub type TmoutcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTC` writer - Clear SMBus alert flag"]
pub type AlertcW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 3 - Clear 0~7 bit address match flag"]
    #[inline(always)]
    #[must_use]
    pub fn addrc(&mut self) -> AddrcW<ClrSpec> {
        AddrcW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear acknowledge failure flag"]
    #[inline(always)]
    #[must_use]
    pub fn ackfailc(&mut self) -> AckfailcW<ClrSpec> {
        AckfailcW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear stop condition generation complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn stopc(&mut self) -> StopcW<ClrSpec> {
        StopcW::new(self, 5)
    }
    #[doc = "Bit 8 - Clear bus error flag"]
    #[inline(always)]
    #[must_use]
    pub fn buserrc(&mut self) -> BuserrcW<ClrSpec> {
        BuserrcW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn arlostc(&mut self) -> ArlostcW<ClrSpec> {
        ArlostcW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear overload / underload flag"]
    #[inline(always)]
    #[must_use]
    pub fn oufc(&mut self) -> OufcW<ClrSpec> {
        OufcW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear PEC receive error flag"]
    #[inline(always)]
    #[must_use]
    pub fn pecerrc(&mut self) -> PecerrcW<ClrSpec> {
        PecerrcW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear SMBus timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn tmoutc(&mut self) -> TmoutcW<ClrSpec> {
        TmoutcW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear SMBus alert flag"]
    #[inline(always)]
    #[must_use]
    pub fn alertc(&mut self) -> AlertcW<ClrSpec> {
        AlertcW::new(self, 13)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for ClrSpec {
    const RESET_VALUE: u32 = 0;
}
