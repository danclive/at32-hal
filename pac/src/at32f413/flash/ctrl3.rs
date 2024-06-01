#[doc = "Register `CTRL3` reader"]
pub type R = crate::R<Ctrl3Spec>;
#[doc = "Register `CTRL3` writer"]
pub type W = crate::W<Ctrl3Spec>;
#[doc = "Field `FPRGM` reader - Flash program"]
pub type FprgmR = crate::BitReader;
#[doc = "Field `FPRGM` writer - Flash program"]
pub type FprgmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECERS` reader - Sector erase"]
pub type SecersR = crate::BitReader;
#[doc = "Field `SECERS` writer - Sector erase"]
pub type SecersW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHPERS` reader - Chip erase"]
pub type ChpersR = crate::BitReader;
#[doc = "Field `CHPERS` writer - Chip erase"]
pub type ChpersW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERSTR` reader - Erasing start"]
pub type ErstrR = crate::BitReader;
#[doc = "Field `ERSTR` writer - Erasing start"]
pub type ErstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPLK` reader - Operation lock"]
pub type OplkR = crate::BitReader;
#[doc = "Field `OPLK` writer - Operation lock"]
pub type OplkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODFIE` reader - Operation done flag interrupt enable"]
pub type OdfieR = crate::BitReader;
#[doc = "Field `ODFIE` writer - Operation done flag interrupt enable"]
pub type OdfieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash program"]
    #[inline(always)]
    pub fn fprgm(&self) -> FprgmR {
        FprgmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector erase"]
    #[inline(always)]
    pub fn secers(&self) -> SecersR {
        SecersR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip erase"]
    #[inline(always)]
    pub fn chpers(&self) -> ChpersR {
        ChpersR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Erasing start"]
    #[inline(always)]
    pub fn erstr(&self) -> ErstrR {
        ErstrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Operation lock"]
    #[inline(always)]
    pub fn oplk(&self) -> OplkR {
        OplkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Operation done flag interrupt enable"]
    #[inline(always)]
    pub fn odfie(&self) -> OdfieR {
        OdfieR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL3")
            .field("fprgm", &self.fprgm())
            .field("secers", &self.secers())
            .field("chpers", &self.chpers())
            .field("erstr", &self.erstr())
            .field("oplk", &self.oplk())
            .field("errie", &self.errie())
            .field("odfie", &self.odfie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flash program"]
    #[inline(always)]
    #[must_use]
    pub fn fprgm(&mut self) -> FprgmW<Ctrl3Spec> {
        FprgmW::new(self, 0)
    }
    #[doc = "Bit 1 - Sector erase"]
    #[inline(always)]
    #[must_use]
    pub fn secers(&mut self) -> SecersW<Ctrl3Spec> {
        SecersW::new(self, 1)
    }
    #[doc = "Bit 2 - Chip erase"]
    #[inline(always)]
    #[must_use]
    pub fn chpers(&mut self) -> ChpersW<Ctrl3Spec> {
        ChpersW::new(self, 2)
    }
    #[doc = "Bit 6 - Erasing start"]
    #[inline(always)]
    #[must_use]
    pub fn erstr(&mut self) -> ErstrW<Ctrl3Spec> {
        ErstrW::new(self, 6)
    }
    #[doc = "Bit 7 - Operation lock"]
    #[inline(always)]
    #[must_use]
    pub fn oplk(&mut self) -> OplkW<Ctrl3Spec> {
        OplkW::new(self, 7)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<Ctrl3Spec> {
        ErrieW::new(self, 10)
    }
    #[doc = "Bit 12 - Operation done flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn odfie(&mut self) -> OdfieW<Ctrl3Spec> {
        OdfieW::new(self, 12)
    }
}
#[doc = "Control 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl3Spec;
impl crate::RegisterSpec for Ctrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl3::R`](R) reader structure"]
impl crate::Readable for Ctrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl3::W`](W) writer structure"]
impl crate::Writable for Ctrl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL3 to value 0x80"]
impl crate::Resettable for Ctrl3Spec {
    const RESET_VALUE: u32 = 0x80;
}
