#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CAP` reader - Capture function enable"]
pub type CapR = crate::BitReader;
#[doc = "Field `CAP` writer - Capture function enable"]
pub type CapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFM` reader - Capture fire mode"]
pub type CfmR = crate::BitReader;
#[doc = "Field `CFM` writer - Capture fire mode"]
pub type CfmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRP` reader - Cropping function enable"]
pub type CrpR = crate::BitReader;
#[doc = "Field `CRP` writer - Cropping function enable"]
pub type CrpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG` reader - JPEG format"]
pub type JpegR = crate::BitReader;
#[doc = "Field `JPEG` writer - JPEG format"]
pub type JpegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM` reader - synchronization mode"]
pub type SmR = crate::BitReader;
#[doc = "Field `SM` writer - synchronization mode"]
pub type SmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKP` reader - Pixel clock polarity"]
pub type CkpR = crate::BitReader;
#[doc = "Field `CKP` writer - Pixel clock polarity"]
pub type CkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSP` reader - Horizontal synchronization polarity"]
pub type HspR = crate::BitReader;
#[doc = "Field `HSP` writer - Horizontal synchronization polarity"]
pub type HspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSP` reader - Vertical synchronization polarity"]
pub type VspR = crate::BitReader;
#[doc = "Field `VSP` writer - Vertical synchronization polarity"]
pub type VspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFRC` reader - Basic frame rate control"]
pub type BfrcR = crate::FieldReader;
#[doc = "Field `BFRC` writer - Basic frame rate control"]
pub type BfrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PDL` reader - Pixel data length"]
pub type PdlR = crate::FieldReader;
#[doc = "Field `PDL` writer - Pixel data length"]
pub type PdlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENA` reader - DVP enable"]
pub type EnaR = crate::BitReader;
#[doc = "Field `ENA` writer - DVP enable"]
pub type EnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCDC` reader - Basic pixel capture/drop control"]
pub type PcdcR = crate::FieldReader;
#[doc = "Field `PCDC` writer - Basic pixel capture/drop control"]
pub type PcdcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCDS` reader - Pixel capture/drop selection"]
pub type PcdsR = crate::BitReader;
#[doc = "Field `PCDS` writer - Pixel capture/drop selection"]
pub type PcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDC` reader - Line capture/drop control"]
pub type LcdcR = crate::BitReader;
#[doc = "Field `LCDC` writer - Line capture/drop control"]
pub type LcdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS` reader - Line capture/drop selection"]
pub type LcdsR = crate::BitReader;
#[doc = "Field `LCDS` writer - Line capture/drop selection"]
pub type LcdsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture function enable"]
    #[inline(always)]
    pub fn cap(&self) -> CapR {
        CapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture fire mode"]
    #[inline(always)]
    pub fn cfm(&self) -> CfmR {
        CfmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cropping function enable"]
    #[inline(always)]
    pub fn crp(&self) -> CrpR {
        CrpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    pub fn jpeg(&self) -> JpegR {
        JpegR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - synchronization mode"]
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pixel clock polarity"]
    #[inline(always)]
    pub fn ckp(&self) -> CkpR {
        CkpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity"]
    #[inline(always)]
    pub fn hsp(&self) -> HspR {
        HspR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity"]
    #[inline(always)]
    pub fn vsp(&self) -> VspR {
        VspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Basic frame rate control"]
    #[inline(always)]
    pub fn bfrc(&self) -> BfrcR {
        BfrcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pixel data length"]
    #[inline(always)]
    pub fn pdl(&self) -> PdlR {
        PdlR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - DVP enable"]
    #[inline(always)]
    pub fn ena(&self) -> EnaR {
        EnaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Basic pixel capture/drop control"]
    #[inline(always)]
    pub fn pcdc(&self) -> PcdcR {
        PcdcR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Pixel capture/drop selection"]
    #[inline(always)]
    pub fn pcds(&self) -> PcdsR {
        PcdsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Line capture/drop control"]
    #[inline(always)]
    pub fn lcdc(&self) -> LcdcR {
        LcdcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Line capture/drop selection"]
    #[inline(always)]
    pub fn lcds(&self) -> LcdsR {
        LcdsR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("lcds", &self.lcds())
            .field("lcdc", &self.lcdc())
            .field("pcds", &self.pcds())
            .field("pcdc", &self.pcdc())
            .field("ena", &self.ena())
            .field("pdl", &self.pdl())
            .field("bfrc", &self.bfrc())
            .field("vsp", &self.vsp())
            .field("hsp", &self.hsp())
            .field("ckp", &self.ckp())
            .field("sm", &self.sm())
            .field("jpeg", &self.jpeg())
            .field("crp", &self.crp())
            .field("cfm", &self.cfm())
            .field("cap", &self.cap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Capture function enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap(&mut self) -> CapW<CtrlSpec> {
        CapW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture fire mode"]
    #[inline(always)]
    #[must_use]
    pub fn cfm(&mut self) -> CfmW<CtrlSpec> {
        CfmW::new(self, 1)
    }
    #[doc = "Bit 2 - Cropping function enable"]
    #[inline(always)]
    #[must_use]
    pub fn crp(&mut self) -> CrpW<CtrlSpec> {
        CrpW::new(self, 2)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JpegW<CtrlSpec> {
        JpegW::new(self, 3)
    }
    #[doc = "Bit 4 - synchronization mode"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SmW<CtrlSpec> {
        SmW::new(self, 4)
    }
    #[doc = "Bit 5 - Pixel clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckp(&mut self) -> CkpW<CtrlSpec> {
        CkpW::new(self, 5)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hsp(&mut self) -> HspW<CtrlSpec> {
        HspW::new(self, 6)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vsp(&mut self) -> VspW<CtrlSpec> {
        VspW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Basic frame rate control"]
    #[inline(always)]
    #[must_use]
    pub fn bfrc(&mut self) -> BfrcW<CtrlSpec> {
        BfrcW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Pixel data length"]
    #[inline(always)]
    #[must_use]
    pub fn pdl(&mut self) -> PdlW<CtrlSpec> {
        PdlW::new(self, 10)
    }
    #[doc = "Bit 14 - DVP enable"]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> EnaW<CtrlSpec> {
        EnaW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Basic pixel capture/drop control"]
    #[inline(always)]
    #[must_use]
    pub fn pcdc(&mut self) -> PcdcW<CtrlSpec> {
        PcdcW::new(self, 16)
    }
    #[doc = "Bit 18 - Pixel capture/drop selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcds(&mut self) -> PcdsW<CtrlSpec> {
        PcdsW::new(self, 18)
    }
    #[doc = "Bit 19 - Line capture/drop control"]
    #[inline(always)]
    #[must_use]
    pub fn lcdc(&mut self) -> LcdcW<CtrlSpec> {
        LcdcW::new(self, 19)
    }
    #[doc = "Bit 20 - Line capture/drop selection"]
    #[inline(always)]
    #[must_use]
    pub fn lcds(&mut self) -> LcdsW<CtrlSpec> {
        LcdsW::new(self, 20)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
