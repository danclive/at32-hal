#[doc = "Register `MMCTIM` reader"]
pub type R = crate::R<MmctimSpec>;
#[doc = "Register `MMCTIM` writer"]
pub type W = crate::W<MmctimSpec>;
#[doc = "Field `TSCGFCIM` reader - Transmit single collision good frame counter interrupt mask"]
pub type TscgfcimR = crate::BitReader;
#[doc = "Field `TSCGFCIM` writer - Transmit single collision good frame counter interrupt mask"]
pub type TscgfcimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMCGFCIM` reader - Transmit multiple collision good frame counter interrupt mask"]
pub type TmcgfcimR = crate::BitReader;
#[doc = "Field `TMCGFCIM` writer - Transmit multiple collision good frame counter interrupt mask"]
pub type TmcgfcimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFCIM` reader - Transmitted good frame counter interrupt mask"]
pub type TgfcimR = crate::BitReader;
#[doc = "Field `TGFCIM` writer - Transmitted good frame counter interrupt mask"]
pub type TgfcimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - Transmit single collision good frame counter interrupt mask"]
    #[inline(always)]
    pub fn tscgfcim(&self) -> TscgfcimR {
        TscgfcimR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit multiple collision good frame counter interrupt mask"]
    #[inline(always)]
    pub fn tmcgfcim(&self) -> TmcgfcimR {
        TmcgfcimR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frame counter interrupt mask"]
    #[inline(always)]
    pub fn tgfcim(&self) -> TgfcimR {
        TgfcimR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTIM")
            .field("tscgfcim", &self.tscgfcim())
            .field("tmcgfcim", &self.tmcgfcim())
            .field("tgfcim", &self.tgfcim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 14 - Transmit single collision good frame counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tscgfcim(&mut self) -> TscgfcimW<MmctimSpec> {
        TscgfcimW::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit multiple collision good frame counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tmcgfcim(&mut self) -> TmcgfcimW<MmctimSpec> {
        TmcgfcimW::new(self, 15)
    }
    #[doc = "Bit 21 - Transmitted good frame counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfcim(&mut self) -> TgfcimW<MmctimSpec> {
        TgfcimW::new(self, 21)
    }
}
#[doc = "Ethernet MMC transmit interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmctim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctimSpec;
impl crate::RegisterSpec for MmctimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctim::R`](R) reader structure"]
impl crate::Readable for MmctimSpec {}
#[doc = "`write(|w| ..)` method takes [`mmctim::W`](W) writer structure"]
impl crate::Writable for MmctimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCTIM to value 0"]
impl crate::Resettable for MmctimSpec {
    const RESET_VALUE: u32 = 0;
}
