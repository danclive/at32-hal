#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CtrlstsSpec>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CtrlstsSpec>;
#[doc = "Field `CMPEN` reader - Comparator enable bit"]
pub type CmpenR = crate::BitReader;
#[doc = "Field `CMPEN` writer - Comparator enable bit"]
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIS` reader - Comparator input shift"]
pub type CmpisR = crate::BitReader;
#[doc = "Field `CMPIS` writer - Comparator input shift"]
pub type CmpisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPSSEL` reader - Comparator speed selection"]
pub type CmpsselR = crate::FieldReader;
#[doc = "Field `CMPSSEL` writer - Comparator speed selection"]
pub type CmpsselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPINVSEL` reader - Comparator inverting selection"]
pub type CmpinvselR = crate::FieldReader;
#[doc = "Field `CMPINVSEL` writer - Comparator inverting selection"]
pub type CmpinvselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMPNINVSEL` reader - Comparator non-inverting input selection"]
pub type CmpninvselR = crate::FieldReader;
#[doc = "Field `CMPNINVSEL` writer - Comparator non-inverting input selection"]
pub type CmpninvselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPTAG` reader - Comparator output target"]
pub type CmptagR = crate::FieldReader;
#[doc = "Field `CMPTAG` writer - Comparator output target"]
pub type CmptagW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMPP` reader - Comparator polarity"]
pub type CmppR = crate::BitReader;
#[doc = "Field `CMPP` writer - Comparator polarity"]
pub type CmppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPHYST` reader - Comparator hysteresis"]
pub type CmphystR = crate::FieldReader;
#[doc = "Field `CMPHYST` writer - Comparator hysteresis"]
pub type CmphystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPBLANKING` reader - Comparator blanking"]
pub type CmpblankingR = crate::FieldReader;
#[doc = "Field `CMPBLANKING` writer - Comparator blanking"]
pub type CmpblankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BRGEN` reader - Comparator brgen"]
pub type BrgenR = crate::BitReader;
#[doc = "Field `BRGEN` writer - Comparator brgen"]
pub type BrgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALEN` reader - Comparator scalen"]
pub type ScalenR = crate::BitReader;
#[doc = "Field `SCALEN` writer - Comparator scalen"]
pub type ScalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPVALUE` reader - Comparator output value"]
pub type CmpvalueR = crate::BitReader;
#[doc = "Field `CMPWP` reader - Comparator write protect"]
pub type CmpwpR = crate::BitReader;
#[doc = "Field `CMPWP` writer - Comparator write protect"]
pub type CmpwpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator enable bit"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator input shift"]
    #[inline(always)]
    pub fn cmpis(&self) -> CmpisR {
        CmpisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator speed selection"]
    #[inline(always)]
    pub fn cmpssel(&self) -> CmpsselR {
        CmpsselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator inverting selection"]
    #[inline(always)]
    pub fn cmpinvsel(&self) -> CmpinvselR {
        CmpinvselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Comparator non-inverting input selection"]
    #[inline(always)]
    pub fn cmpninvsel(&self) -> CmpninvselR {
        CmpninvselR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Comparator output target"]
    #[inline(always)]
    pub fn cmptag(&self) -> CmptagR {
        CmptagR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 15 - Comparator polarity"]
    #[inline(always)]
    pub fn cmpp(&self) -> CmppR {
        CmppR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator hysteresis"]
    #[inline(always)]
    pub fn cmphyst(&self) -> CmphystR {
        CmphystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator blanking"]
    #[inline(always)]
    pub fn cmpblanking(&self) -> CmpblankingR {
        CmpblankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Comparator brgen"]
    #[inline(always)]
    pub fn brgen(&self) -> BrgenR {
        BrgenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Comparator scalen"]
    #[inline(always)]
    pub fn scalen(&self) -> ScalenR {
        ScalenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator output value"]
    #[inline(always)]
    pub fn cmpvalue(&self) -> CmpvalueR {
        CmpvalueR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator write protect"]
    #[inline(always)]
    pub fn cmpwp(&self) -> CmpwpR {
        CmpwpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS")
            .field("cmpen", &self.cmpen())
            .field("cmpis", &self.cmpis())
            .field("cmpssel", &self.cmpssel())
            .field("cmpinvsel", &self.cmpinvsel())
            .field("cmpninvsel", &self.cmpninvsel())
            .field("cmptag", &self.cmptag())
            .field("cmpp", &self.cmpp())
            .field("cmphyst", &self.cmphyst())
            .field("cmpblanking", &self.cmpblanking())
            .field("brgen", &self.brgen())
            .field("scalen", &self.scalen())
            .field("cmpvalue", &self.cmpvalue())
            .field("cmpwp", &self.cmpwp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Comparator enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CmpenW<CtrlstsSpec> {
        CmpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator input shift"]
    #[inline(always)]
    #[must_use]
    pub fn cmpis(&mut self) -> CmpisW<CtrlstsSpec> {
        CmpisW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Comparator speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpssel(&mut self) -> CmpsselW<CtrlstsSpec> {
        CmpsselW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator inverting selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpinvsel(&mut self) -> CmpinvselW<CtrlstsSpec> {
        CmpinvselW::new(self, 4)
    }
    #[doc = "Bits 7:8 - Comparator non-inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpninvsel(&mut self) -> CmpninvselW<CtrlstsSpec> {
        CmpninvselW::new(self, 7)
    }
    #[doc = "Bits 10:12 - Comparator output target"]
    #[inline(always)]
    #[must_use]
    pub fn cmptag(&mut self) -> CmptagW<CtrlstsSpec> {
        CmptagW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cmpp(&mut self) -> CmppW<CtrlstsSpec> {
        CmppW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmphyst(&mut self) -> CmphystW<CtrlstsSpec> {
        CmphystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator blanking"]
    #[inline(always)]
    #[must_use]
    pub fn cmpblanking(&mut self) -> CmpblankingW<CtrlstsSpec> {
        CmpblankingW::new(self, 18)
    }
    #[doc = "Bit 22 - Comparator brgen"]
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BrgenW<CtrlstsSpec> {
        BrgenW::new(self, 22)
    }
    #[doc = "Bit 23 - Comparator scalen"]
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> ScalenW<CtrlstsSpec> {
        ScalenW::new(self, 23)
    }
    #[doc = "Bit 31 - Comparator write protect"]
    #[inline(always)]
    #[must_use]
    pub fn cmpwp(&mut self) -> CmpwpW<CtrlstsSpec> {
        CmpwpW::new(self, 31)
    }
}
#[doc = "CMP control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlstsSpec;
impl crate::RegisterSpec for CtrlstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CtrlstsSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CtrlstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLSTS to value 0"]
impl crate::Resettable for CtrlstsSpec {
    const RESET_VALUE: u32 = 0;
}
