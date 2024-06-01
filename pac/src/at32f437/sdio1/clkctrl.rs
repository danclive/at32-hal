#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Field `CLKDIV` reader - Clock division"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock division"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKOEN` reader - Clock output enable"]
pub type ClkoenR = crate::BitReader;
#[doc = "Field `CLKOEN` writer - Clock output enable"]
pub type ClkoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSVEN` reader - Power saving mode enable"]
pub type PwrsvenR = crate::BitReader;
#[doc = "Field `PWRSVEN` writer - Power saving mode enable"]
pub type PwrsvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSEN` reader - Clock divider bypass enable bit"]
pub type BypsenR = crate::BitReader;
#[doc = "Field `BYPSEN` writer - Clock divider bypass enable bit"]
pub type BypsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSWS` reader - Bus width selection"]
pub type BuswsR = crate::FieldReader;
#[doc = "Field `BUSWS` writer - Bus width selection"]
pub type BuswsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKEDS` reader - SDIO_CK edge selection bit"]
pub type ClkedsR = crate::BitReader;
#[doc = "Field `CLKEDS` writer - SDIO_CK edge selection bit"]
pub type ClkedsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFCEN` reader - Hardware flow control enable"]
pub type HfcenR = crate::BitReader;
#[doc = "Field `HFCEN` writer - Hardware flow control enable"]
pub type HfcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV98` reader - Clock divide factor bit9 and bit8"]
pub type Clkdiv98R = crate::FieldReader;
#[doc = "Field `CLKDIV98` writer - Clock divide factor bit9 and bit8"]
pub type Clkdiv98W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Clock output enable"]
    #[inline(always)]
    pub fn clkoen(&self) -> ClkoenR {
        ClkoenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power saving mode enable"]
    #[inline(always)]
    pub fn pwrsven(&self) -> PwrsvenR {
        PwrsvenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypsen(&self) -> BypsenR {
        BypsenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Bus width selection"]
    #[inline(always)]
    pub fn busws(&self) -> BuswsR {
        BuswsR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CK edge selection bit"]
    #[inline(always)]
    pub fn clkeds(&self) -> ClkedsR {
        ClkedsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware flow control enable"]
    #[inline(always)]
    pub fn hfcen(&self) -> HfcenR {
        HfcenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Clock divide factor bit9 and bit8"]
    #[inline(always)]
    pub fn clkdiv98(&self) -> Clkdiv98R {
        Clkdiv98R::new(((self.bits >> 15) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTRL")
            .field("clkdiv", &self.clkdiv())
            .field("clkoen", &self.clkoen())
            .field("pwrsven", &self.pwrsven())
            .field("bypsen", &self.bypsen())
            .field("busws", &self.busws())
            .field("clkeds", &self.clkeds())
            .field("hfcen", &self.hfcen())
            .field("clkdiv98", &self.clkdiv98())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<ClkctrlSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bit 8 - Clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkoen(&mut self) -> ClkoenW<ClkctrlSpec> {
        ClkoenW::new(self, 8)
    }
    #[doc = "Bit 9 - Power saving mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsven(&mut self) -> PwrsvenW<ClkctrlSpec> {
        PwrsvenW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn bypsen(&mut self) -> BypsenW<ClkctrlSpec> {
        BypsenW::new(self, 10)
    }
    #[doc = "Bits 11:12 - Bus width selection"]
    #[inline(always)]
    #[must_use]
    pub fn busws(&mut self) -> BuswsW<ClkctrlSpec> {
        BuswsW::new(self, 11)
    }
    #[doc = "Bit 13 - SDIO_CK edge selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn clkeds(&mut self) -> ClkedsW<ClkctrlSpec> {
        ClkedsW::new(self, 13)
    }
    #[doc = "Bit 14 - Hardware flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfcen(&mut self) -> HfcenW<ClkctrlSpec> {
        HfcenW::new(self, 14)
    }
    #[doc = "Bits 15:16 - Clock divide factor bit9 and bit8"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv98(&mut self) -> Clkdiv98W<ClkctrlSpec> {
        Clkdiv98W::new(self, 15)
    }
}
#[doc = "SD clock control register (SDIO_CLKCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for ClkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
