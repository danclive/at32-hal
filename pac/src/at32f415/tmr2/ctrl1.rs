#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `TMREN` reader - TMR enable"]
pub type TmrenR = crate::BitReader;
#[doc = "Field `TMREN` writer - TMR enable"]
pub type TmrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFEN` reader - Overflow event enable"]
pub type OvfenR = crate::BitReader;
#[doc = "Field `OVFEN` writer - Overflow event enable"]
pub type OvfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFS` reader - Overflow event source"]
pub type OvfsR = crate::BitReader;
#[doc = "Field `OVFS` writer - Overflow event source"]
pub type OvfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCMEN` reader - One cycle mode enable"]
pub type OcmenR = crate::BitReader;
#[doc = "Field `OCMEN` writer - One cycle mode enable"]
pub type OcmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OWCDIR` reader - One-way count direction"]
pub type OwcdirR = crate::BitReader;
#[doc = "Field `OWCDIR` writer - One-way count direction"]
pub type OwcdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWCMSEL` reader - Two-way count mode selection"]
pub type TwcmselR = crate::FieldReader;
#[doc = "Field `TWCMSEL` writer - Two-way count mode selection"]
pub type TwcmselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRBEN` reader - Period buffer enable"]
pub type PrbenR = crate::BitReader;
#[doc = "Field `PRBEN` writer - Period buffer enable"]
pub type PrbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV` reader - Clock divider"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divider"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PMEN` reader - Plus Mode Enable"]
pub type PmenR = crate::BitReader;
#[doc = "Field `PMEN` writer - Plus Mode Enable"]
pub type PmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TMR enable"]
    #[inline(always)]
    pub fn tmren(&self) -> TmrenR {
        TmrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow event enable"]
    #[inline(always)]
    pub fn ovfen(&self) -> OvfenR {
        OvfenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow event source"]
    #[inline(always)]
    pub fn ovfs(&self) -> OvfsR {
        OvfsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One cycle mode enable"]
    #[inline(always)]
    pub fn ocmen(&self) -> OcmenR {
        OcmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - One-way count direction"]
    #[inline(always)]
    pub fn owcdir(&self) -> OwcdirR {
        OwcdirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Two-way count mode selection"]
    #[inline(always)]
    pub fn twcmsel(&self) -> TwcmselR {
        TwcmselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Period buffer enable"]
    #[inline(always)]
    pub fn prben(&self) -> PrbenR {
        PrbenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Plus Mode Enable"]
    #[inline(always)]
    pub fn pmen(&self) -> PmenR {
        PmenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("pmen", &self.pmen())
            .field("clkdiv", &self.clkdiv())
            .field("prben", &self.prben())
            .field("twcmsel", &self.twcmsel())
            .field("owcdir", &self.owcdir())
            .field("ocmen", &self.ocmen())
            .field("ovfs", &self.ovfs())
            .field("ovfen", &self.ovfen())
            .field("tmren", &self.tmren())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TMR enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmren(&mut self) -> TmrenW<Ctrl1Spec> {
        TmrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow event enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfen(&mut self) -> OvfenW<Ctrl1Spec> {
        OvfenW::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow event source"]
    #[inline(always)]
    #[must_use]
    pub fn ovfs(&mut self) -> OvfsW<Ctrl1Spec> {
        OvfsW::new(self, 2)
    }
    #[doc = "Bit 3 - One cycle mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocmen(&mut self) -> OcmenW<Ctrl1Spec> {
        OcmenW::new(self, 3)
    }
    #[doc = "Bit 4 - One-way count direction"]
    #[inline(always)]
    #[must_use]
    pub fn owcdir(&mut self) -> OwcdirW<Ctrl1Spec> {
        OwcdirW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Two-way count mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn twcmsel(&mut self) -> TwcmselW<Ctrl1Spec> {
        TwcmselW::new(self, 5)
    }
    #[doc = "Bit 7 - Period buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn prben(&mut self) -> PrbenW<Ctrl1Spec> {
        PrbenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<Ctrl1Spec> {
        ClkdivW::new(self, 8)
    }
    #[doc = "Bit 10 - Plus Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmen(&mut self) -> PmenW<Ctrl1Spec> {
        PmenW::new(self, 10)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
