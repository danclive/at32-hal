#[doc = "Register `BPDC` reader"]
pub type R = crate::R<BpdcSpec>;
#[doc = "Register `BPDC` writer"]
pub type W = crate::W<BpdcSpec>;
#[doc = "Field `LEXTEN` reader - Low speed external crystal enable"]
pub type LextenR = crate::BitReader;
#[doc = "Field `LEXTEN` writer - Low speed external crystal enable"]
pub type LextenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEXTSTBL` reader - Low speed external crystal ready"]
pub type LextstblR = crate::BitReader;
#[doc = "Field `LEXTBYPS` reader - Low speed external crystal bypass"]
pub type LextbypsR = crate::BitReader;
#[doc = "Field `LEXTBYPS` writer - Low speed external crystal bypass"]
pub type LextbypsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERTCSEL` reader - ERTC clock selection"]
pub type ErtcselR = crate::FieldReader;
#[doc = "Field `ERTCSEL` writer - ERTC clock selection"]
pub type ErtcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ERTCEN` reader - ERTC clock enable"]
pub type ErtcenR = crate::BitReader;
#[doc = "Field `ERTCEN` writer - ERTC clock enable"]
pub type ErtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPDRST` reader - Battery powered domain software reset"]
pub type BpdrstR = crate::BitReader;
#[doc = "Field `BPDRST` writer - Battery powered domain software reset"]
pub type BpdrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low speed external crystal enable"]
    #[inline(always)]
    pub fn lexten(&self) -> LextenR {
        LextenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low speed external crystal ready"]
    #[inline(always)]
    pub fn lextstbl(&self) -> LextstblR {
        LextstblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low speed external crystal bypass"]
    #[inline(always)]
    pub fn lextbyps(&self) -> LextbypsR {
        LextbypsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ERTC clock selection"]
    #[inline(always)]
    pub fn ertcsel(&self) -> ErtcselR {
        ErtcselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - ERTC clock enable"]
    #[inline(always)]
    pub fn ertcen(&self) -> ErtcenR {
        ErtcenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Battery powered domain software reset"]
    #[inline(always)]
    pub fn bpdrst(&self) -> BpdrstR {
        BpdrstR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPDC")
            .field("lexten", &self.lexten())
            .field("lextstbl", &self.lextstbl())
            .field("lextbyps", &self.lextbyps())
            .field("ertcsel", &self.ertcsel())
            .field("ertcen", &self.ertcen())
            .field("bpdrst", &self.bpdrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Low speed external crystal enable"]
    #[inline(always)]
    #[must_use]
    pub fn lexten(&mut self) -> LextenW<BpdcSpec> {
        LextenW::new(self, 0)
    }
    #[doc = "Bit 2 - Low speed external crystal bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lextbyps(&mut self) -> LextbypsW<BpdcSpec> {
        LextbypsW::new(self, 2)
    }
    #[doc = "Bits 8:9 - ERTC clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn ertcsel(&mut self) -> ErtcselW<BpdcSpec> {
        ErtcselW::new(self, 8)
    }
    #[doc = "Bit 15 - ERTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ertcen(&mut self) -> ErtcenW<BpdcSpec> {
        ErtcenW::new(self, 15)
    }
    #[doc = "Bit 16 - Battery powered domain software reset"]
    #[inline(always)]
    #[must_use]
    pub fn bpdrst(&mut self) -> BpdrstW<BpdcSpec> {
        BpdrstW::new(self, 16)
    }
}
#[doc = "Battery powered domain control register (CRM_BPDC)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpdcSpec;
impl crate::RegisterSpec for BpdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpdc::R`](R) reader structure"]
impl crate::Readable for BpdcSpec {}
#[doc = "`write(|w| ..)` method takes [`bpdc::W`](W) writer structure"]
impl crate::Writable for BpdcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPDC to value 0"]
impl crate::Resettable for BpdcSpec {
    const RESET_VALUE: u32 = 0;
}
