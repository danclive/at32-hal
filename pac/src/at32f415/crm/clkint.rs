#[doc = "Register `CLKINT` reader"]
pub type R = crate::R<ClkintSpec>;
#[doc = "Register `CLKINT` writer"]
pub type W = crate::W<ClkintSpec>;
#[doc = "Field `LICKSTBLF` reader - LICK ready interrupt flag"]
pub type LickstblfR = crate::BitReader;
#[doc = "Field `LEXTSTBLF` reader - LEXT ready interrupt flag"]
pub type LextstblfR = crate::BitReader;
#[doc = "Field `HICKSTBLF` reader - HICK ready interrupt flag"]
pub type HickstblfR = crate::BitReader;
#[doc = "Field `HEXTSTBLF` reader - HEXT ready interrupt flag"]
pub type HextstblfR = crate::BitReader;
#[doc = "Field `PLLSTBLF` reader - PLL ready interrupt flag"]
pub type PllstblfR = crate::BitReader;
#[doc = "Field `CFDF` reader - Clock failure detection interrupt flag"]
pub type CfdfR = crate::BitReader;
#[doc = "Field `LICKSTBLIEN` reader - LICK ready interrupt enable"]
pub type LickstblienR = crate::BitReader;
#[doc = "Field `LICKSTBLIEN` writer - LICK ready interrupt enable"]
pub type LickstblienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEXTSTBLIEN` reader - LEXT ready interrupt enable"]
pub type LextstblienR = crate::BitReader;
#[doc = "Field `LEXTSTBLIEN` writer - LEXT ready interrupt enable"]
pub type LextstblienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICKSTBLIEN` reader - HICK ready interrupt enable"]
pub type HickstblienR = crate::BitReader;
#[doc = "Field `HICKSTBLIEN` writer - HICK ready interrupt enable"]
pub type HickstblienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEXTSTBLIEN` reader - HEXT ready interrupt enable"]
pub type HextstblienR = crate::BitReader;
#[doc = "Field `HEXTSTBLIEN` writer - HEXT ready interrupt enable"]
pub type HextstblienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTBLIEN` reader - PLL ready interrupt enable"]
pub type PllstblienR = crate::BitReader;
#[doc = "Field `PLLSTBLIEN` writer - PLL ready interrupt enable"]
pub type PllstblienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LICKSTBLFC` writer - LICK ready interrupt clear"]
pub type LickstblfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEXTSTBLFC` writer - LEXT ready interrupt clear"]
pub type LextstblfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICKSTBLFC` writer - HICK ready interrupt clear"]
pub type HickstblfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEXTSTBLFC` writer - HEXT ready interrupt clear"]
pub type HextstblfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTBLFC` writer - PLL ready interrupt clear"]
pub type PllstblfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDFC` writer - Clock failure detection interrupt clear"]
pub type CfdfcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LICK ready interrupt flag"]
    #[inline(always)]
    pub fn lickstblf(&self) -> LickstblfR {
        LickstblfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LEXT ready interrupt flag"]
    #[inline(always)]
    pub fn lextstblf(&self) -> LextstblfR {
        LextstblfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HICK ready interrupt flag"]
    #[inline(always)]
    pub fn hickstblf(&self) -> HickstblfR {
        HickstblfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HEXT ready interrupt flag"]
    #[inline(always)]
    pub fn hextstblf(&self) -> HextstblfR {
        HextstblfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllstblf(&self) -> PllstblfR {
        PllstblfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock failure detection interrupt flag"]
    #[inline(always)]
    pub fn cfdf(&self) -> CfdfR {
        CfdfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LICK ready interrupt enable"]
    #[inline(always)]
    pub fn lickstblien(&self) -> LickstblienR {
        LickstblienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LEXT ready interrupt enable"]
    #[inline(always)]
    pub fn lextstblien(&self) -> LextstblienR {
        LextstblienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HICK ready interrupt enable"]
    #[inline(always)]
    pub fn hickstblien(&self) -> HickstblienR {
        HickstblienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HEXT ready interrupt enable"]
    #[inline(always)]
    pub fn hextstblien(&self) -> HextstblienR {
        HextstblienR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllstblien(&self) -> PllstblienR {
        PllstblienR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKINT")
            .field("lickstblf", &self.lickstblf())
            .field("lextstblf", &self.lextstblf())
            .field("hickstblf", &self.hickstblf())
            .field("hextstblf", &self.hextstblf())
            .field("pllstblf", &self.pllstblf())
            .field("cfdf", &self.cfdf())
            .field("lickstblien", &self.lickstblien())
            .field("lextstblien", &self.lextstblien())
            .field("hickstblien", &self.hickstblien())
            .field("hextstblien", &self.hextstblien())
            .field("pllstblien", &self.pllstblien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - LICK ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lickstblien(&mut self) -> LickstblienW<ClkintSpec> {
        LickstblienW::new(self, 8)
    }
    #[doc = "Bit 9 - LEXT ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lextstblien(&mut self) -> LextstblienW<ClkintSpec> {
        LextstblienW::new(self, 9)
    }
    #[doc = "Bit 10 - HICK ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hickstblien(&mut self) -> HickstblienW<ClkintSpec> {
        HickstblienW::new(self, 10)
    }
    #[doc = "Bit 11 - HEXT ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hextstblien(&mut self) -> HextstblienW<ClkintSpec> {
        HextstblienW::new(self, 11)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllstblien(&mut self) -> PllstblienW<ClkintSpec> {
        PllstblienW::new(self, 12)
    }
    #[doc = "Bit 16 - LICK ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lickstblfc(&mut self) -> LickstblfcW<ClkintSpec> {
        LickstblfcW::new(self, 16)
    }
    #[doc = "Bit 17 - LEXT ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lextstblfc(&mut self) -> LextstblfcW<ClkintSpec> {
        LextstblfcW::new(self, 17)
    }
    #[doc = "Bit 18 - HICK ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hickstblfc(&mut self) -> HickstblfcW<ClkintSpec> {
        HickstblfcW::new(self, 18)
    }
    #[doc = "Bit 19 - HEXT ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hextstblfc(&mut self) -> HextstblfcW<ClkintSpec> {
        HextstblfcW::new(self, 19)
    }
    #[doc = "Bit 20 - PLL ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllstblfc(&mut self) -> PllstblfcW<ClkintSpec> {
        PllstblfcW::new(self, 20)
    }
    #[doc = "Bit 23 - Clock failure detection interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cfdfc(&mut self) -> CfdfcW<ClkintSpec> {
        CfdfcW::new(self, 23)
    }
}
#[doc = "Clock interrupt register (CRM_CLKINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkintSpec;
impl crate::RegisterSpec for ClkintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkint::R`](R) reader structure"]
impl crate::Readable for ClkintSpec {}
#[doc = "`write(|w| ..)` method takes [`clkint::W`](W) writer structure"]
impl crate::Writable for ClkintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKINT to value 0"]
impl crate::Resettable for ClkintSpec {
    const RESET_VALUE: u32 = 0;
}
