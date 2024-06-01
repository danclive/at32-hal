#[doc = "Register `INTSTS` reader"]
pub type R = crate::R<IntstsSpec>;
#[doc = "Register `INTSTS` writer"]
pub type W = crate::W<IntstsSpec>;
#[doc = "Field `LINE0` reader - Line 0 state bit"]
pub type Line0R = crate::BitReader;
#[doc = "Field `LINE0` writer - Line 0 state bit"]
pub type Line0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE1` reader - Line 1 state bit"]
pub type Line1R = crate::BitReader;
#[doc = "Field `LINE1` writer - Line 1 state bit"]
pub type Line1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE2` reader - Line 2 state bit"]
pub type Line2R = crate::BitReader;
#[doc = "Field `LINE2` writer - Line 2 state bit"]
pub type Line2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE3` reader - Line 3 state bit"]
pub type Line3R = crate::BitReader;
#[doc = "Field `LINE3` writer - Line 3 state bit"]
pub type Line3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE4` reader - Line 4 state bit"]
pub type Line4R = crate::BitReader;
#[doc = "Field `LINE4` writer - Line 4 state bit"]
pub type Line4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE5` reader - Line 5 state bit"]
pub type Line5R = crate::BitReader;
#[doc = "Field `LINE5` writer - Line 5 state bit"]
pub type Line5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE6` reader - Line 6 state bit"]
pub type Line6R = crate::BitReader;
#[doc = "Field `LINE6` writer - Line 6 state bit"]
pub type Line6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE7` reader - Line 7 state bit"]
pub type Line7R = crate::BitReader;
#[doc = "Field `LINE7` writer - Line 7 state bit"]
pub type Line7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE8` reader - Line 8 state bit"]
pub type Line8R = crate::BitReader;
#[doc = "Field `LINE8` writer - Line 8 state bit"]
pub type Line8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE9` reader - Line 9 state bit"]
pub type Line9R = crate::BitReader;
#[doc = "Field `LINE9` writer - Line 9 state bit"]
pub type Line9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE10` reader - Line 10 state bit"]
pub type Line10R = crate::BitReader;
#[doc = "Field `LINE10` writer - Line 10 state bit"]
pub type Line10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE11` reader - Line 11 state bit"]
pub type Line11R = crate::BitReader;
#[doc = "Field `LINE11` writer - Line 11 state bit"]
pub type Line11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE12` reader - Line 12 state bit"]
pub type Line12R = crate::BitReader;
#[doc = "Field `LINE12` writer - Line 12 state bit"]
pub type Line12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE13` reader - Line 13 state bit"]
pub type Line13R = crate::BitReader;
#[doc = "Field `LINE13` writer - Line 13 state bit"]
pub type Line13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE14` reader - Line 14 state bit"]
pub type Line14R = crate::BitReader;
#[doc = "Field `LINE14` writer - Line 14 state bit"]
pub type Line14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE15` reader - Line 15 state bit"]
pub type Line15R = crate::BitReader;
#[doc = "Field `LINE15` writer - Line 15 state bit"]
pub type Line15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE16` reader - Line 16 state bit"]
pub type Line16R = crate::BitReader;
#[doc = "Field `LINE16` writer - Line 16 state bit"]
pub type Line16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE17` reader - Line 17 state bit"]
pub type Line17R = crate::BitReader;
#[doc = "Field `LINE17` writer - Line 17 state bit"]
pub type Line17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE18` reader - Line 18 state bit"]
pub type Line18R = crate::BitReader;
#[doc = "Field `LINE18` writer - Line 18 state bit"]
pub type Line18W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Line 0 state bit"]
    #[inline(always)]
    pub fn line0(&self) -> Line0R {
        Line0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Line 1 state bit"]
    #[inline(always)]
    pub fn line1(&self) -> Line1R {
        Line1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Line 2 state bit"]
    #[inline(always)]
    pub fn line2(&self) -> Line2R {
        Line2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line 3 state bit"]
    #[inline(always)]
    pub fn line3(&self) -> Line3R {
        Line3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line 4 state bit"]
    #[inline(always)]
    pub fn line4(&self) -> Line4R {
        Line4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line 5 state bit"]
    #[inline(always)]
    pub fn line5(&self) -> Line5R {
        Line5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Line 6 state bit"]
    #[inline(always)]
    pub fn line6(&self) -> Line6R {
        Line6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Line 7 state bit"]
    #[inline(always)]
    pub fn line7(&self) -> Line7R {
        Line7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Line 8 state bit"]
    #[inline(always)]
    pub fn line8(&self) -> Line8R {
        Line8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Line 9 state bit"]
    #[inline(always)]
    pub fn line9(&self) -> Line9R {
        Line9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Line 10 state bit"]
    #[inline(always)]
    pub fn line10(&self) -> Line10R {
        Line10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Line 11 state bit"]
    #[inline(always)]
    pub fn line11(&self) -> Line11R {
        Line11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Line 12 state bit"]
    #[inline(always)]
    pub fn line12(&self) -> Line12R {
        Line12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Line 13 state bit"]
    #[inline(always)]
    pub fn line13(&self) -> Line13R {
        Line13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Line 14 state bit"]
    #[inline(always)]
    pub fn line14(&self) -> Line14R {
        Line14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Line 15 state bit"]
    #[inline(always)]
    pub fn line15(&self) -> Line15R {
        Line15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Line 16 state bit"]
    #[inline(always)]
    pub fn line16(&self) -> Line16R {
        Line16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Line 17 state bit"]
    #[inline(always)]
    pub fn line17(&self) -> Line17R {
        Line17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Line 18 state bit"]
    #[inline(always)]
    pub fn line18(&self) -> Line18R {
        Line18R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTS")
            .field("line0", &self.line0())
            .field("line1", &self.line1())
            .field("line2", &self.line2())
            .field("line3", &self.line3())
            .field("line4", &self.line4())
            .field("line5", &self.line5())
            .field("line6", &self.line6())
            .field("line7", &self.line7())
            .field("line8", &self.line8())
            .field("line9", &self.line9())
            .field("line10", &self.line10())
            .field("line11", &self.line11())
            .field("line12", &self.line12())
            .field("line13", &self.line13())
            .field("line14", &self.line14())
            .field("line15", &self.line15())
            .field("line16", &self.line16())
            .field("line17", &self.line17())
            .field("line18", &self.line18())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Line 0 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line0(&mut self) -> Line0W<IntstsSpec> {
        Line0W::new(self, 0)
    }
    #[doc = "Bit 1 - Line 1 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line1(&mut self) -> Line1W<IntstsSpec> {
        Line1W::new(self, 1)
    }
    #[doc = "Bit 2 - Line 2 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line2(&mut self) -> Line2W<IntstsSpec> {
        Line2W::new(self, 2)
    }
    #[doc = "Bit 3 - Line 3 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line3(&mut self) -> Line3W<IntstsSpec> {
        Line3W::new(self, 3)
    }
    #[doc = "Bit 4 - Line 4 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line4(&mut self) -> Line4W<IntstsSpec> {
        Line4W::new(self, 4)
    }
    #[doc = "Bit 5 - Line 5 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line5(&mut self) -> Line5W<IntstsSpec> {
        Line5W::new(self, 5)
    }
    #[doc = "Bit 6 - Line 6 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line6(&mut self) -> Line6W<IntstsSpec> {
        Line6W::new(self, 6)
    }
    #[doc = "Bit 7 - Line 7 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line7(&mut self) -> Line7W<IntstsSpec> {
        Line7W::new(self, 7)
    }
    #[doc = "Bit 8 - Line 8 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line8(&mut self) -> Line8W<IntstsSpec> {
        Line8W::new(self, 8)
    }
    #[doc = "Bit 9 - Line 9 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line9(&mut self) -> Line9W<IntstsSpec> {
        Line9W::new(self, 9)
    }
    #[doc = "Bit 10 - Line 10 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line10(&mut self) -> Line10W<IntstsSpec> {
        Line10W::new(self, 10)
    }
    #[doc = "Bit 11 - Line 11 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line11(&mut self) -> Line11W<IntstsSpec> {
        Line11W::new(self, 11)
    }
    #[doc = "Bit 12 - Line 12 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line12(&mut self) -> Line12W<IntstsSpec> {
        Line12W::new(self, 12)
    }
    #[doc = "Bit 13 - Line 13 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line13(&mut self) -> Line13W<IntstsSpec> {
        Line13W::new(self, 13)
    }
    #[doc = "Bit 14 - Line 14 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line14(&mut self) -> Line14W<IntstsSpec> {
        Line14W::new(self, 14)
    }
    #[doc = "Bit 15 - Line 15 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line15(&mut self) -> Line15W<IntstsSpec> {
        Line15W::new(self, 15)
    }
    #[doc = "Bit 16 - Line 16 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line16(&mut self) -> Line16W<IntstsSpec> {
        Line16W::new(self, 16)
    }
    #[doc = "Bit 17 - Line 17 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line17(&mut self) -> Line17W<IntstsSpec> {
        Line17W::new(self, 17)
    }
    #[doc = "Bit 18 - Line 18 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line18(&mut self) -> Line18W<IntstsSpec> {
        Line18W::new(self, 18)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstsSpec;
impl crate::RegisterSpec for IntstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsts::R`](R) reader structure"]
impl crate::Readable for IntstsSpec {}
#[doc = "`write(|w| ..)` method takes [`intsts::W`](W) writer structure"]
impl crate::Writable for IntstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for IntstsSpec {
    const RESET_VALUE: u32 = 0;
}
