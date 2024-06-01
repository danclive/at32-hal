#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `FPRGM` reader - Flash program"]
pub type FprgmR = crate::BitReader;
#[doc = "Field `FPRGM` writer - Flash program"]
pub type FprgmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECERS` reader - Sector erase"]
pub type SecersR = crate::BitReader;
#[doc = "Field `SECERS` writer - Sector erase"]
pub type SecersW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANKERS` reader - Bank erase"]
pub type BankersR = crate::BitReader;
#[doc = "Field `BANKERS` writer - Bank erase"]
pub type BankersW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USDPRGM` reader - User system data program"]
pub type UsdprgmR = crate::BitReader;
#[doc = "Field `USDPRGM` writer - User system data program"]
pub type UsdprgmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USDERS` reader - User system data erase"]
pub type UsdersR = crate::BitReader;
#[doc = "Field `USDERS` writer - User system data erase"]
pub type UsdersW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERSTR` reader - Erasing start"]
pub type ErstrR = crate::BitReader;
#[doc = "Field `ERSTR` writer - Erasing start"]
pub type ErstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPLK` reader - Operation lock"]
pub type OplkR = crate::BitReader;
#[doc = "Field `OPLK` writer - Operation lock"]
pub type OplkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USDULKS` reader - User system data unlock success"]
pub type UsdulksR = crate::BitReader;
#[doc = "Field `USDULKS` writer - User system data unlock success"]
pub type UsdulksW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Bank erase"]
    #[inline(always)]
    pub fn bankers(&self) -> BankersR {
        BankersR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - User system data program"]
    #[inline(always)]
    pub fn usdprgm(&self) -> UsdprgmR {
        UsdprgmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - User system data erase"]
    #[inline(always)]
    pub fn usders(&self) -> UsdersR {
        UsdersR::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 9 - User system data unlock success"]
    #[inline(always)]
    pub fn usdulks(&self) -> UsdulksR {
        UsdulksR::new(((self.bits >> 9) & 1) != 0)
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
        f.debug_struct("CTRL")
            .field("fprgm", &self.fprgm())
            .field("secers", &self.secers())
            .field("bankers", &self.bankers())
            .field("usdprgm", &self.usdprgm())
            .field("usders", &self.usders())
            .field("erstr", &self.erstr())
            .field("oplk", &self.oplk())
            .field("usdulks", &self.usdulks())
            .field("errie", &self.errie())
            .field("odfie", &self.odfie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flash program"]
    #[inline(always)]
    #[must_use]
    pub fn fprgm(&mut self) -> FprgmW<CtrlSpec> {
        FprgmW::new(self, 0)
    }
    #[doc = "Bit 1 - Sector erase"]
    #[inline(always)]
    #[must_use]
    pub fn secers(&mut self) -> SecersW<CtrlSpec> {
        SecersW::new(self, 1)
    }
    #[doc = "Bit 2 - Bank erase"]
    #[inline(always)]
    #[must_use]
    pub fn bankers(&mut self) -> BankersW<CtrlSpec> {
        BankersW::new(self, 2)
    }
    #[doc = "Bit 4 - User system data program"]
    #[inline(always)]
    #[must_use]
    pub fn usdprgm(&mut self) -> UsdprgmW<CtrlSpec> {
        UsdprgmW::new(self, 4)
    }
    #[doc = "Bit 5 - User system data erase"]
    #[inline(always)]
    #[must_use]
    pub fn usders(&mut self) -> UsdersW<CtrlSpec> {
        UsdersW::new(self, 5)
    }
    #[doc = "Bit 6 - Erasing start"]
    #[inline(always)]
    #[must_use]
    pub fn erstr(&mut self) -> ErstrW<CtrlSpec> {
        ErstrW::new(self, 6)
    }
    #[doc = "Bit 7 - Operation lock"]
    #[inline(always)]
    #[must_use]
    pub fn oplk(&mut self) -> OplkW<CtrlSpec> {
        OplkW::new(self, 7)
    }
    #[doc = "Bit 9 - User system data unlock success"]
    #[inline(always)]
    #[must_use]
    pub fn usdulks(&mut self) -> UsdulksW<CtrlSpec> {
        UsdulksW::new(self, 9)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<CtrlSpec> {
        ErrieW::new(self, 10)
    }
    #[doc = "Bit 12 - Operation done flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn odfie(&mut self) -> OdfieW<CtrlSpec> {
        OdfieW::new(self, 12)
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
#[doc = "`reset()` method sets CTRL to value 0x80"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x80;
}
