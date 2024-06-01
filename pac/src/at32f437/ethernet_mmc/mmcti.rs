#[doc = "Register `MMCTI` reader"]
pub type R = crate::R<MmctiSpec>;
#[doc = "Register `MMCTI` writer"]
pub type W = crate::W<MmctiSpec>;
#[doc = "Field `TSCGFCI` reader - Transmit single collision good frame counter interrupt"]
pub type TscgfciR = crate::BitReader;
#[doc = "Field `TSCGFCI` writer - Transmit single collision good frame counter interrupt"]
pub type TscgfciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFMSC` reader - Transmit good frames more single collision"]
pub type TgfmscR = crate::BitReader;
#[doc = "Field `TGFMSC` writer - Transmit good frames more single collision"]
pub type TgfmscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGF` reader - Transmitted good frames"]
pub type TgfR = crate::BitReader;
#[doc = "Field `TGF` writer - Transmitted good frames"]
pub type TgfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - Transmit single collision good frame counter interrupt"]
    #[inline(always)]
    pub fn tscgfci(&self) -> TscgfciR {
        TscgfciR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit good frames more single collision"]
    #[inline(always)]
    pub fn tgfmsc(&self) -> TgfmscR {
        TgfmscR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames"]
    #[inline(always)]
    pub fn tgf(&self) -> TgfR {
        TgfR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTI")
            .field("tscgfci", &self.tscgfci())
            .field("tgfmsc", &self.tgfmsc())
            .field("tgf", &self.tgf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 14 - Transmit single collision good frame counter interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tscgfci(&mut self) -> TscgfciW<MmctiSpec> {
        TscgfciW::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit good frames more single collision"]
    #[inline(always)]
    #[must_use]
    pub fn tgfmsc(&mut self) -> TgfmscW<MmctiSpec> {
        TgfmscW::new(self, 15)
    }
    #[doc = "Bit 21 - Transmitted good frames"]
    #[inline(always)]
    #[must_use]
    pub fn tgf(&mut self) -> TgfW<MmctiSpec> {
        TgfW::new(self, 21)
    }
}
#[doc = "Ethernet MMC transmit interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcti::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcti::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctiSpec;
impl crate::RegisterSpec for MmctiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcti::R`](R) reader structure"]
impl crate::Readable for MmctiSpec {}
#[doc = "`write(|w| ..)` method takes [`mmcti::W`](W) writer structure"]
impl crate::Writable for MmctiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCTI to value 0"]
impl crate::Resettable for MmctiSpec {
    const RESET_VALUE: u32 = 0;
}
