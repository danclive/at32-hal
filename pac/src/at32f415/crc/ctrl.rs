#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `RST` reader - Reset bit"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Reset bit"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLY_SIZE` reader - Polynomial size"]
pub type PolySizeR = crate::FieldReader;
#[doc = "Field `POLY_SIZE` writer - Polynomial size"]
pub type PolySizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REVID` reader - Reverse input data"]
pub type RevidR = crate::FieldReader;
#[doc = "Field `REVID` writer - Reverse input data"]
pub type RevidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REVOD` reader - Reverse output data"]
pub type RevodR = crate::BitReader;
#[doc = "Field `REVOD` writer - Reverse output data"]
pub type RevodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset bit"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    pub fn poly_size(&self) -> PolySizeR {
        PolySizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    pub fn revid(&self) -> RevidR {
        RevidR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn revod(&self) -> RevodR {
        RevodR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("rst", &self.rst())
            .field("poly_size", &self.poly_size())
            .field("revid", &self.revid())
            .field("revod", &self.revod())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<CtrlSpec> {
        RstW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    #[must_use]
    pub fn poly_size(&mut self) -> PolySizeW<CtrlSpec> {
        PolySizeW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    #[must_use]
    pub fn revid(&mut self) -> RevidW<CtrlSpec> {
        RevidW::new(self, 5)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    #[must_use]
    pub fn revod(&mut self) -> RevodW<CtrlSpec> {
        RevodW::new(self, 7)
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
