#[doc = "Register `CMD_W3` reader"]
pub type R = crate::R<CmdW3Spec>;
#[doc = "Register `CMD_W3` writer"]
pub type W = crate::W<CmdW3Spec>;
#[doc = "Field `WEN` reader - Write data enable"]
pub type WenR = crate::BitReader;
#[doc = "Field `WEN` writer - Write data enable"]
pub type WenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTSEN` reader - Read spi status enable"]
pub type RstsenR = crate::BitReader;
#[doc = "Field `RSTSEN` writer - Read spi status enable"]
pub type RstsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTSC` reader - Read spi status configure"]
pub type RstscR = crate::BitReader;
#[doc = "Field `RSTSC` writer - Read spi status configure"]
pub type RstscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPMODE` reader - SPI operate mode"]
pub type OpmodeR = crate::FieldReader;
#[doc = "Field `OPMODE` writer - SPI operate mode"]
pub type OpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PEMOPC` reader - Performance enhance mode operate code"]
pub type PemopcR = crate::FieldReader;
#[doc = "Field `PEMOPC` writer - Performance enhance mode operate code"]
pub type PemopcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSC` reader - Instruction code"]
pub type InscR = crate::FieldReader;
#[doc = "Field `INSC` writer - Instruction code"]
pub type InscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - Write data enable"]
    #[inline(always)]
    pub fn wen(&self) -> WenR {
        WenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read spi status enable"]
    #[inline(always)]
    pub fn rstsen(&self) -> RstsenR {
        RstsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read spi status configure"]
    #[inline(always)]
    pub fn rstsc(&self) -> RstscR {
        RstscR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:7 - SPI operate mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OpmodeR {
        OpmodeR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Performance enhance mode operate code"]
    #[inline(always)]
    pub fn pemopc(&self) -> PemopcR {
        PemopcR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Instruction code"]
    #[inline(always)]
    pub fn insc(&self) -> InscR {
        InscR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_W3")
            .field("wen", &self.wen())
            .field("rstsen", &self.rstsen())
            .field("rstsc", &self.rstsc())
            .field("opmode", &self.opmode())
            .field("pemopc", &self.pemopc())
            .field("insc", &self.insc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Write data enable"]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WenW<CmdW3Spec> {
        WenW::new(self, 1)
    }
    #[doc = "Bit 2 - Read spi status enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstsen(&mut self) -> RstsenW<CmdW3Spec> {
        RstsenW::new(self, 2)
    }
    #[doc = "Bit 3 - Read spi status configure"]
    #[inline(always)]
    #[must_use]
    pub fn rstsc(&mut self) -> RstscW<CmdW3Spec> {
        RstscW::new(self, 3)
    }
    #[doc = "Bits 5:7 - SPI operate mode"]
    #[inline(always)]
    #[must_use]
    pub fn opmode(&mut self) -> OpmodeW<CmdW3Spec> {
        OpmodeW::new(self, 5)
    }
    #[doc = "Bits 16:23 - Performance enhance mode operate code"]
    #[inline(always)]
    #[must_use]
    pub fn pemopc(&mut self) -> PemopcW<CmdW3Spec> {
        PemopcW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Instruction code"]
    #[inline(always)]
    #[must_use]
    pub fn insc(&mut self) -> InscW<CmdW3Spec> {
        InscW::new(self, 24)
    }
}
#[doc = "Command word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdW3Spec;
impl crate::RegisterSpec for CmdW3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_w3::R`](R) reader structure"]
impl crate::Readable for CmdW3Spec {}
#[doc = "`write(|w| ..)` method takes [`cmd_w3::W`](W) writer structure"]
impl crate::Writable for CmdW3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_W3 to value 0"]
impl crate::Resettable for CmdW3Spec {
    const RESET_VALUE: u32 = 0;
}
