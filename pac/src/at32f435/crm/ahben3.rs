#[doc = "Register `AHBEN3` reader"]
pub type R = crate::R<Ahben3Spec>;
#[doc = "Register `AHBEN3` writer"]
pub type W = crate::W<Ahben3Spec>;
#[doc = "Field `XMCEN` reader - XMC clock enable"]
pub type XmcenR = crate::BitReader;
#[doc = "Field `XMCEN` writer - XMC clock enable"]
pub type XmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI1EN` reader - QSPI1 clock enable"]
pub type Qspi1enR = crate::BitReader;
#[doc = "Field `QSPI1EN` writer - QSPI1 clock enable"]
pub type Qspi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI2EN` reader - QSPI2 clock enable"]
pub type Qspi2enR = crate::BitReader;
#[doc = "Field `QSPI2EN` writer - QSPI2 clock enable"]
pub type Qspi2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO2EN` reader - SDIO 2 clock enable"]
pub type Sdio2enR = crate::BitReader;
#[doc = "Field `SDIO2EN` writer - SDIO 2 clock enable"]
pub type Sdio2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XMC clock enable"]
    #[inline(always)]
    pub fn xmcen(&self) -> XmcenR {
        XmcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    pub fn qspi1en(&self) -> Qspi1enR {
        Qspi1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI2 clock enable"]
    #[inline(always)]
    pub fn qspi2en(&self) -> Qspi2enR {
        Qspi2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO 2 clock enable"]
    #[inline(always)]
    pub fn sdio2en(&self) -> Sdio2enR {
        Sdio2enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN3")
            .field("xmcen", &self.xmcen())
            .field("qspi1en", &self.qspi1en())
            .field("qspi2en", &self.qspi2en())
            .field("sdio2en", &self.sdio2en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - XMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn xmcen(&mut self) -> XmcenW<Ahben3Spec> {
        XmcenW::new(self, 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspi1en(&mut self) -> Qspi1enW<Ahben3Spec> {
        Qspi1enW::new(self, 1)
    }
    #[doc = "Bit 14 - QSPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspi2en(&mut self) -> Qspi2enW<Ahben3Spec> {
        Qspi2enW::new(self, 14)
    }
    #[doc = "Bit 15 - SDIO 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio2en(&mut self) -> Sdio2enW<Ahben3Spec> {
        Sdio2enW::new(self, 15)
    }
}
#[doc = "AHB peripheral clock enable register 3 (CRM_AHBEN3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahben3Spec;
impl crate::RegisterSpec for Ahben3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben3::R`](R) reader structure"]
impl crate::Readable for Ahben3Spec {}
#[doc = "`write(|w| ..)` method takes [`ahben3::W`](W) writer structure"]
impl crate::Writable for Ahben3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBEN3 to value 0"]
impl crate::Resettable for Ahben3Spec {
    const RESET_VALUE: u32 = 0;
}
