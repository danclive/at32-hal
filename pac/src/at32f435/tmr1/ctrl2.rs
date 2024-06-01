#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `CBCTRL` reader - Channel buffer control"]
pub type CbctrlR = crate::BitReader;
#[doc = "Field `CBCTRL` writer - Channel buffer control"]
pub type CbctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFS` reader - Channel control bit flash select"]
pub type CcfsR = crate::BitReader;
#[doc = "Field `CCFS` writer - Channel control bit flash select"]
pub type CcfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRS` reader - DMA request source"]
pub type DrsR = crate::BitReader;
#[doc = "Field `DRS` writer - DMA request source"]
pub type DrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTOS` reader - Primary TMR output selection"]
pub type PtosR = crate::FieldReader;
#[doc = "Field `PTOS` writer - Primary TMR output selection"]
pub type PtosW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C1INSEL` reader - C1IN selection"]
pub type C1inselR = crate::BitReader;
#[doc = "Field `C1INSEL` writer - C1IN selection"]
pub type C1inselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1IOS` reader - Channel 1 idle output state"]
pub type C1iosR = crate::BitReader;
#[doc = "Field `C1IOS` writer - Channel 1 idle output state"]
pub type C1iosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1CIOS` reader - Channel 1 complementary idle output state"]
pub type C1ciosR = crate::BitReader;
#[doc = "Field `C1CIOS` writer - Channel 1 complementary idle output state"]
pub type C1ciosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2IOS` reader - Channel 2 idle output state"]
pub type C2iosR = crate::BitReader;
#[doc = "Field `C2IOS` writer - Channel 2 idle output state"]
pub type C2iosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2CIOS` reader - Channel 2 complementary idle output state"]
pub type C2ciosR = crate::BitReader;
#[doc = "Field `C2CIOS` writer - Channel 2 complementary idle output state"]
pub type C2ciosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3IOS` reader - Channel 3 idle output state"]
pub type C3iosR = crate::BitReader;
#[doc = "Field `C3IOS` writer - Channel 3 idle output state"]
pub type C3iosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3CIOS` reader - Channel 3 complementary idle output state"]
pub type C3ciosR = crate::BitReader;
#[doc = "Field `C3CIOS` writer - Channel 3 complementary idle output state"]
pub type C3ciosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4IOS` reader - Channel 4 idle output state"]
pub type C4iosR = crate::BitReader;
#[doc = "Field `C4IOS` writer - Channel 4 idle output state"]
pub type C4iosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOUT2EN` reader - TRGOUT2 enable"]
pub type Trgout2enR = crate::BitReader;
#[doc = "Field `TRGOUT2EN` writer - TRGOUT2 enable"]
pub type Trgout2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    pub fn cbctrl(&self) -> CbctrlR {
        CbctrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Channel control bit flash select"]
    #[inline(always)]
    pub fn ccfs(&self) -> CcfsR {
        CcfsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    pub fn drs(&self) -> DrsR {
        DrsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    pub fn ptos(&self) -> PtosR {
        PtosR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - C1IN selection"]
    #[inline(always)]
    pub fn c1insel(&self) -> C1inselR {
        C1inselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    pub fn c1ios(&self) -> C1iosR {
        C1iosR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    pub fn c1cios(&self) -> C1ciosR {
        C1ciosR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 idle output state"]
    #[inline(always)]
    pub fn c2ios(&self) -> C2iosR {
        C2iosR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 complementary idle output state"]
    #[inline(always)]
    pub fn c2cios(&self) -> C2ciosR {
        C2ciosR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 idle output state"]
    #[inline(always)]
    pub fn c3ios(&self) -> C3iosR {
        C3iosR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 3 complementary idle output state"]
    #[inline(always)]
    pub fn c3cios(&self) -> C3ciosR {
        C3ciosR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 4 idle output state"]
    #[inline(always)]
    pub fn c4ios(&self) -> C4iosR {
        C4iosR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 31 - TRGOUT2 enable"]
    #[inline(always)]
    pub fn trgout2en(&self) -> Trgout2enR {
        Trgout2enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("trgout2en", &self.trgout2en())
            .field("c4ios", &self.c4ios())
            .field("c3cios", &self.c3cios())
            .field("c3ios", &self.c3ios())
            .field("c2cios", &self.c2cios())
            .field("c2ios", &self.c2ios())
            .field("c1cios", &self.c1cios())
            .field("c1ios", &self.c1ios())
            .field("c1insel", &self.c1insel())
            .field("ptos", &self.ptos())
            .field("drs", &self.drs())
            .field("ccfs", &self.ccfs())
            .field("cbctrl", &self.cbctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    #[must_use]
    pub fn cbctrl(&mut self) -> CbctrlW<Ctrl2Spec> {
        CbctrlW::new(self, 0)
    }
    #[doc = "Bit 2 - Channel control bit flash select"]
    #[inline(always)]
    #[must_use]
    pub fn ccfs(&mut self) -> CcfsW<Ctrl2Spec> {
        CcfsW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    #[must_use]
    pub fn drs(&mut self) -> DrsW<Ctrl2Spec> {
        DrsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptos(&mut self) -> PtosW<Ctrl2Spec> {
        PtosW::new(self, 4)
    }
    #[doc = "Bit 7 - C1IN selection"]
    #[inline(always)]
    #[must_use]
    pub fn c1insel(&mut self) -> C1inselW<Ctrl2Spec> {
        C1inselW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c1ios(&mut self) -> C1iosW<Ctrl2Spec> {
        C1iosW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c1cios(&mut self) -> C1ciosW<Ctrl2Spec> {
        C1ciosW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c2ios(&mut self) -> C2iosW<Ctrl2Spec> {
        C2iosW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 2 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c2cios(&mut self) -> C2ciosW<Ctrl2Spec> {
        C2ciosW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 3 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c3ios(&mut self) -> C3iosW<Ctrl2Spec> {
        C3iosW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 3 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c3cios(&mut self) -> C3ciosW<Ctrl2Spec> {
        C3ciosW::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 4 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c4ios(&mut self) -> C4iosW<Ctrl2Spec> {
        C4iosW::new(self, 14)
    }
    #[doc = "Bit 31 - TRGOUT2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgout2en(&mut self) -> Trgout2enW<Ctrl2Spec> {
        Trgout2enW::new(self, 31)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0;
}
