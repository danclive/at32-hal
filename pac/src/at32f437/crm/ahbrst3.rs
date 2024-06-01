#[doc = "Register `AHBRST3` reader"]
pub type R = crate::R<Ahbrst3Spec>;
#[doc = "Register `AHBRST3` writer"]
pub type W = crate::W<Ahbrst3Spec>;
#[doc = "Field `XMCRST` reader - XMC reset"]
pub type XmcrstR = crate::BitReader;
#[doc = "Field `XMCRST` writer - XMC reset"]
pub type XmcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI1RST` reader - QSPI1 reset"]
pub type Qspi1rstR = crate::BitReader;
#[doc = "Field `QSPI1RST` writer - QSPI1 reset"]
pub type Qspi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI2RST` reader - QSPI2 reset"]
pub type Qspi2rstR = crate::BitReader;
#[doc = "Field `QSPI2RST` writer - QSPI2 reset"]
pub type Qspi2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO2RST` reader - SDIO2 reset"]
pub type Sdio2rstR = crate::BitReader;
#[doc = "Field `SDIO2RST` writer - SDIO2 reset"]
pub type Sdio2rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XMC reset"]
    #[inline(always)]
    pub fn xmcrst(&self) -> XmcrstR {
        XmcrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QSPI1 reset"]
    #[inline(always)]
    pub fn qspi1rst(&self) -> Qspi1rstR {
        Qspi1rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI2 reset"]
    #[inline(always)]
    pub fn qspi2rst(&self) -> Qspi2rstR {
        Qspi2rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO2 reset"]
    #[inline(always)]
    pub fn sdio2rst(&self) -> Sdio2rstR {
        Sdio2rstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST3")
            .field("xmcrst", &self.xmcrst())
            .field("qspi1rst", &self.qspi1rst())
            .field("qspi2rst", &self.qspi2rst())
            .field("sdio2rst", &self.sdio2rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - XMC reset"]
    #[inline(always)]
    #[must_use]
    pub fn xmcrst(&mut self) -> XmcrstW<Ahbrst3Spec> {
        XmcrstW::new(self, 0)
    }
    #[doc = "Bit 1 - QSPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn qspi1rst(&mut self) -> Qspi1rstW<Ahbrst3Spec> {
        Qspi1rstW::new(self, 1)
    }
    #[doc = "Bit 14 - QSPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn qspi2rst(&mut self) -> Qspi2rstW<Ahbrst3Spec> {
        Qspi2rstW::new(self, 14)
    }
    #[doc = "Bit 15 - SDIO2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdio2rst(&mut self) -> Sdio2rstW<Ahbrst3Spec> {
        Sdio2rstW::new(self, 15)
    }
}
#[doc = "AHB peripheral reset register 3 (CRM_AHBRST3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahbrst3Spec;
impl crate::RegisterSpec for Ahbrst3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst3::R`](R) reader structure"]
impl crate::Readable for Ahbrst3Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbrst3::W`](W) writer structure"]
impl crate::Writable for Ahbrst3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST3 to value 0"]
impl crate::Resettable for Ahbrst3Spec {
    const RESET_VALUE: u32 = 0;
}
