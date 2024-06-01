#[doc = "Register `MMCRIM` reader"]
pub type R = crate::R<MmcrimSpec>;
#[doc = "Register `MMCRIM` writer"]
pub type W = crate::W<MmcrimSpec>;
#[doc = "Field `RCEFCIM` reader - Received CRC error frame counter interrupt mask"]
pub type RcefcimR = crate::BitReader;
#[doc = "Field `RCEFCIM` writer - Received CRC error frame counter interrupt mask"]
pub type RcefcimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAEFACIM` reader - Received alignment error frame alignment counter interrupt mask"]
pub type RaefacimR = crate::BitReader;
#[doc = "Field `RAEFACIM` writer - Received alignment error frame alignment counter interrupt mask"]
pub type RaefacimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUGFCIM` reader - Received unicast good frame counter interrupt mask"]
pub type RugfcimR = crate::BitReader;
#[doc = "Field `RUGFCIM` writer - Received unicast good frame counter interrupt mask"]
pub type RugfcimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Received CRC error frame counter interrupt mask"]
    #[inline(always)]
    pub fn rcefcim(&self) -> RcefcimR {
        RcefcimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received alignment error frame alignment counter interrupt mask"]
    #[inline(always)]
    pub fn raefacim(&self) -> RaefacimR {
        RaefacimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received unicast good frame counter interrupt mask"]
    #[inline(always)]
    pub fn rugfcim(&self) -> RugfcimR {
        RugfcimR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRIM")
            .field("rcefcim", &self.rcefcim())
            .field("raefacim", &self.raefacim())
            .field("rugfcim", &self.rugfcim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - Received CRC error frame counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rcefcim(&mut self) -> RcefcimW<MmcrimSpec> {
        RcefcimW::new(self, 5)
    }
    #[doc = "Bit 6 - Received alignment error frame alignment counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn raefacim(&mut self) -> RaefacimW<MmcrimSpec> {
        RaefacimW::new(self, 6)
    }
    #[doc = "Bit 17 - Received unicast good frame counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rugfcim(&mut self) -> RugfcimW<MmcrimSpec> {
        RugfcimW::new(self, 17)
    }
}
#[doc = "Ethernet MMC receive interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcrimSpec;
impl crate::RegisterSpec for MmcrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrim::R`](R) reader structure"]
impl crate::Readable for MmcrimSpec {}
#[doc = "`write(|w| ..)` method takes [`mmcrim::W`](W) writer structure"]
impl crate::Writable for MmcrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCRIM to value 0"]
impl crate::Resettable for MmcrimSpec {
    const RESET_VALUE: u32 = 0;
}
