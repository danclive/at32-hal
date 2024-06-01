#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CMDIDX` reader - CMDIDX"]
pub type CmdidxR = crate::FieldReader;
#[doc = "Field `CMDIDX` writer - CMDIDX"]
pub type CmdidxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RSPWT` reader - WAITRESP"]
pub type RspwtR = crate::FieldReader;
#[doc = "Field `RSPWT` writer - WAITRESP"]
pub type RspwtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTWT` reader - WAITINT"]
pub type IntwtR = crate::BitReader;
#[doc = "Field `INTWT` writer - WAITINT"]
pub type IntwtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNDWT` reader - WAITPEND"]
pub type PndwtR = crate::BitReader;
#[doc = "Field `PNDWT` writer - WAITPEND"]
pub type PndwtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDMEN` reader - CPSMEN"]
pub type CmdmenR = crate::BitReader;
#[doc = "Field `CMDMEN` writer - CPSMEN"]
pub type CmdmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOSUSP` reader - SDIOSuspend"]
pub type SdiosuspR = crate::BitReader;
#[doc = "Field `SDIOSUSP` writer - SDIOSuspend"]
pub type SdiosuspW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - CMDIDX"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CmdidxR {
        CmdidxR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - WAITRESP"]
    #[inline(always)]
    pub fn rspwt(&self) -> RspwtR {
        RspwtR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - WAITINT"]
    #[inline(always)]
    pub fn intwt(&self) -> IntwtR {
        IntwtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WAITPEND"]
    #[inline(always)]
    pub fn pndwt(&self) -> PndwtR {
        PndwtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPSMEN"]
    #[inline(always)]
    pub fn cmdmen(&self) -> CmdmenR {
        CmdmenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIOSuspend"]
    #[inline(always)]
    pub fn sdiosusp(&self) -> SdiosuspR {
        SdiosuspR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("cmdidx", &self.cmdidx())
            .field("rspwt", &self.rspwt())
            .field("intwt", &self.intwt())
            .field("pndwt", &self.pndwt())
            .field("cmdmen", &self.cmdmen())
            .field("sdiosusp", &self.sdiosusp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDIDX"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CmdidxW<CmdSpec> {
        CmdidxW::new(self, 0)
    }
    #[doc = "Bits 6:7 - WAITRESP"]
    #[inline(always)]
    #[must_use]
    pub fn rspwt(&mut self) -> RspwtW<CmdSpec> {
        RspwtW::new(self, 6)
    }
    #[doc = "Bit 8 - WAITINT"]
    #[inline(always)]
    #[must_use]
    pub fn intwt(&mut self) -> IntwtW<CmdSpec> {
        IntwtW::new(self, 8)
    }
    #[doc = "Bit 9 - WAITPEND"]
    #[inline(always)]
    #[must_use]
    pub fn pndwt(&mut self) -> PndwtW<CmdSpec> {
        PndwtW::new(self, 9)
    }
    #[doc = "Bit 10 - CPSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmdmen(&mut self) -> CmdmenW<CmdSpec> {
        CmdmenW::new(self, 10)
    }
    #[doc = "Bit 11 - SDIOSuspend"]
    #[inline(always)]
    #[must_use]
    pub fn sdiosusp(&mut self) -> SdiosuspW<CmdSpec> {
        SdiosuspW::new(self, 11)
    }
}
#[doc = "SDIO command register (SDIO_CMD)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
