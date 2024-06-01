#[doc = "Register `MACISTS` reader"]
pub type R = crate::R<MacistsSpec>;
#[doc = "Register `MACISTS` writer"]
pub type W = crate::W<MacistsSpec>;
#[doc = "Field `PIS` reader - PMT interrupt status"]
pub type PisR = crate::BitReader;
#[doc = "Field `PIS` writer - PMT interrupt status"]
pub type PisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIS` reader - MMC interrupt status"]
pub type MisR = crate::BitReader;
#[doc = "Field `MIS` writer - MMC interrupt status"]
pub type MisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRIS` reader - MMC receive interrupt status"]
pub type MrisR = crate::BitReader;
#[doc = "Field `MRIS` writer - MMC receive interrupt status"]
pub type MrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTIS` reader - MMC transmit interrupt status"]
pub type MtisR = crate::BitReader;
#[doc = "Field `MTIS` writer - MMC transmit interrupt status"]
pub type MtisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIS` reader - Timestamp interrupt status"]
pub type TisR = crate::BitReader;
#[doc = "Field `TIS` writer - Timestamp interrupt status"]
pub type TisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PMT interrupt status"]
    #[inline(always)]
    pub fn pis(&self) -> PisR {
        PisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC interrupt status"]
    #[inline(always)]
    pub fn mis(&self) -> MisR {
        MisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC receive interrupt status"]
    #[inline(always)]
    pub fn mris(&self) -> MrisR {
        MrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC transmit interrupt status"]
    #[inline(always)]
    pub fn mtis(&self) -> MtisR {
        MtisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp interrupt status"]
    #[inline(always)]
    pub fn tis(&self) -> TisR {
        TisR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACISTS")
            .field("pis", &self.pis())
            .field("mis", &self.mis())
            .field("mris", &self.mris())
            .field("mtis", &self.mtis())
            .field("tis", &self.tis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - PMT interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn pis(&mut self) -> PisW<MacistsSpec> {
        PisW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mis(&mut self) -> MisW<MacistsSpec> {
        MisW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC receive interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mris(&mut self) -> MrisW<MacistsSpec> {
        MrisW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC transmit interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mtis(&mut self) -> MtisW<MacistsSpec> {
        MtisW::new(self, 6)
    }
    #[doc = "Bit 9 - Timestamp interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn tis(&mut self) -> TisW<MacistsSpec> {
        TisW::new(self, 9)
    }
}
#[doc = "Ethernet MAC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macists::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macists::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacistsSpec;
impl crate::RegisterSpec for MacistsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macists::R`](R) reader structure"]
impl crate::Readable for MacistsSpec {}
#[doc = "`write(|w| ..)` method takes [`macists::W`](W) writer structure"]
impl crate::Writable for MacistsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACISTS to value 0"]
impl crate::Resettable for MacistsSpec {
    const RESET_VALUE: u32 = 0;
}
