#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CMD` writer - SDRAM Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BK2` writer - SDRAM Bank 2"]
pub type Bk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK1` writer - SDRAM Bank 1"]
pub type Bk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ART` reader - Auto-refresh times"]
pub type ArtR = crate::FieldReader;
#[doc = "Field `ART` writer - Auto-refresh times"]
pub type ArtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MRD` reader - Mode register data"]
pub type MrdR = crate::FieldReader<u16>;
#[doc = "Field `MRD` writer - Mode register data"]
pub type MrdW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 5:8 - Auto-refresh times"]
    #[inline(always)]
    pub fn art(&self) -> ArtR {
        ArtR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:21 - Mode register data"]
    #[inline(always)]
    pub fn mrd(&self) -> MrdR {
        MrdR::new(((self.bits >> 9) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("art", &self.art())
            .field("mrd", &self.mrd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SDRAM Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CmdSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bit 3 - SDRAM Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn bk2(&mut self) -> Bk2W<CmdSpec> {
        Bk2W::new(self, 3)
    }
    #[doc = "Bit 4 - SDRAM Bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn bk1(&mut self) -> Bk1W<CmdSpec> {
        Bk1W::new(self, 4)
    }
    #[doc = "Bits 5:8 - Auto-refresh times"]
    #[inline(always)]
    #[must_use]
    pub fn art(&mut self) -> ArtW<CmdSpec> {
        ArtW::new(self, 5)
    }
    #[doc = "Bits 9:21 - Mode register data"]
    #[inline(always)]
    #[must_use]
    pub fn mrd(&mut self) -> MrdW<CmdSpec> {
        MrdW::new(self, 9)
    }
}
#[doc = "SDRAM Command Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
