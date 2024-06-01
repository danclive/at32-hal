#[doc = "Register `AHBLPEN3` reader"]
pub type R = crate::R<Ahblpen3Spec>;
#[doc = "Register `AHBLPEN3` writer"]
pub type W = crate::W<Ahblpen3Spec>;
#[doc = "Field `XMCLPEN` reader - XMC clock enable during sleep mode"]
pub type XmclpenR = crate::BitReader;
#[doc = "Field `XMCLPEN` writer - XMC clock enable during sleep mode"]
pub type XmclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI1LPEN` reader - QSPI1 clock enable during sleep mode"]
pub type Qspi1lpenR = crate::BitReader;
#[doc = "Field `QSPI1LPEN` writer - QSPI1 clock enable during sleep mode"]
pub type Qspi1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI2LPEN` reader - QSPI2 clock enable during sleep mode"]
pub type Qspi2lpenR = crate::BitReader;
#[doc = "Field `QSPI2LPEN` writer - QSPI2 clock enable during sleep mode"]
pub type Qspi2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO2LPEN` reader - SDIO2 clock enable during sleep mode"]
pub type Sdio2lpenR = crate::BitReader;
#[doc = "Field `SDIO2LPEN` writer - SDIO2 clock enable during sleep mode"]
pub type Sdio2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XMC clock enable during sleep mode"]
    #[inline(always)]
    pub fn xmclpen(&self) -> XmclpenR {
        XmclpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn qspi1lpen(&self) -> Qspi1lpenR {
        Qspi1lpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn qspi2lpen(&self) -> Qspi2lpenR {
        Qspi2lpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sdio2lpen(&self) -> Sdio2lpenR {
        Sdio2lpenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPEN3")
            .field("xmclpen", &self.xmclpen())
            .field("qspi1lpen", &self.qspi1lpen())
            .field("qspi2lpen", &self.qspi2lpen())
            .field("sdio2lpen", &self.sdio2lpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - XMC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn xmclpen(&mut self) -> XmclpenW<Ahblpen3Spec> {
        XmclpenW::new(self, 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn qspi1lpen(&mut self) -> Qspi1lpenW<Ahblpen3Spec> {
        Qspi1lpenW::new(self, 1)
    }
    #[doc = "Bit 14 - QSPI2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn qspi2lpen(&mut self) -> Qspi2lpenW<Ahblpen3Spec> {
        Qspi2lpenW::new(self, 14)
    }
    #[doc = "Bit 15 - SDIO2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdio2lpen(&mut self) -> Sdio2lpenW<Ahblpen3Spec> {
        Sdio2lpenW::new(self, 15)
    }
}
#[doc = "AHB peripheral Low-power clock enable register 3 (CRM_AHBLPEN3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahblpen3Spec;
impl crate::RegisterSpec for Ahblpen3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen3::R`](R) reader structure"]
impl crate::Readable for Ahblpen3Spec {}
#[doc = "`write(|w| ..)` method takes [`ahblpen3::W`](W) writer structure"]
impl crate::Writable for Ahblpen3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBLPEN3 to value 0xc003"]
impl crate::Resettable for Ahblpen3Spec {
    const RESET_VALUE: u32 = 0xc003;
}
