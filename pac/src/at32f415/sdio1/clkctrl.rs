#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Field `CLKPSC` reader - Clock divide factor"]
pub type ClkpscR = crate::FieldReader;
#[doc = "Field `CLKPSC` writer - Clock divide factor"]
pub type ClkpscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKEN` reader - Clock enable bit"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable bit"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSVG` reader - Power saving configuration bit"]
pub type PwrsvgR = crate::BitReader;
#[doc = "Field `PWRSVG` writer - Power saving configuration bit"]
pub type PwrsvgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPS` reader - Clock divider bypass enable bit"]
pub type BypsR = crate::BitReader;
#[doc = "Field `BYPS` writer - Clock divider bypass enable bit"]
pub type BypsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSWIDTH` reader - Wide bus mode enable bit"]
pub type BuswidthR = crate::FieldReader;
#[doc = "Field `BUSWIDTH` writer - Wide bus mode enable bit"]
pub type BuswidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKEDG` reader - SDIO_CK dephasing selection bit"]
pub type ClkedgR = crate::BitReader;
#[doc = "Field `CLKEDG` writer - SDIO_CK dephasing selection bit"]
pub type ClkedgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLWCTRLEN` reader - HW Flow Control enable"]
pub type FlwctrlenR = crate::BitReader;
#[doc = "Field `FLWCTRLEN` writer - HW Flow Control enable"]
pub type FlwctrlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPSC98` reader - Clock divide factor bit9 and bit8"]
pub type Clkpsc98R = crate::FieldReader;
#[doc = "Field `CLKPSC98` writer - Clock divide factor bit9 and bit8"]
pub type Clkpsc98W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    pub fn clkpsc(&self) -> ClkpscR {
        ClkpscR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    pub fn pwrsvg(&self) -> PwrsvgR {
        PwrsvgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn byps(&self) -> BypsR {
        BypsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    pub fn buswidth(&self) -> BuswidthR {
        BuswidthR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    pub fn clkedg(&self) -> ClkedgR {
        ClkedgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    pub fn flwctrlen(&self) -> FlwctrlenR {
        FlwctrlenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Clock divide factor bit9 and bit8"]
    #[inline(always)]
    pub fn clkpsc98(&self) -> Clkpsc98R {
        Clkpsc98R::new(((self.bits >> 15) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTRL")
            .field("clkpsc", &self.clkpsc())
            .field("clken", &self.clken())
            .field("pwrsvg", &self.pwrsvg())
            .field("byps", &self.byps())
            .field("buswidth", &self.buswidth())
            .field("clkedg", &self.clkedg())
            .field("flwctrlen", &self.flwctrlen())
            .field("clkpsc98", &self.clkpsc98())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clkpsc(&mut self) -> ClkpscW<ClkctrlSpec> {
        ClkpscW::new(self, 0)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<ClkctrlSpec> {
        ClkenW::new(self, 8)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsvg(&mut self) -> PwrsvgW<ClkctrlSpec> {
        PwrsvgW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn byps(&mut self) -> BypsW<ClkctrlSpec> {
        BypsW::new(self, 10)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn buswidth(&mut self) -> BuswidthW<ClkctrlSpec> {
        BuswidthW::new(self, 11)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn clkedg(&mut self) -> ClkedgW<ClkctrlSpec> {
        ClkedgW::new(self, 13)
    }
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    #[must_use]
    pub fn flwctrlen(&mut self) -> FlwctrlenW<ClkctrlSpec> {
        FlwctrlenW::new(self, 14)
    }
    #[doc = "Bits 15:16 - Clock divide factor bit9 and bit8"]
    #[inline(always)]
    #[must_use]
    pub fn clkpsc98(&mut self) -> Clkpsc98W<ClkctrlSpec> {
        Clkpsc98W::new(self, 15)
    }
}
#[doc = "SDI clock control register (SDIO_CLKCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
