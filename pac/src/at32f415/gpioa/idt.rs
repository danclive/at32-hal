#[doc = "Register `IDT` reader"]
pub type R = crate::R<IdtSpec>;
#[doc = "Field `IDT0` reader - Port input data"]
pub type Idt0R = crate::BitReader;
#[doc = "Field `IDT1` reader - Port input data"]
pub type Idt1R = crate::BitReader;
#[doc = "Field `IDT2` reader - Port input data"]
pub type Idt2R = crate::BitReader;
#[doc = "Field `IDT3` reader - Port input data"]
pub type Idt3R = crate::BitReader;
#[doc = "Field `IDT4` reader - Port input data"]
pub type Idt4R = crate::BitReader;
#[doc = "Field `IDT5` reader - Port input data"]
pub type Idt5R = crate::BitReader;
#[doc = "Field `IDT6` reader - Port input data"]
pub type Idt6R = crate::BitReader;
#[doc = "Field `IDT7` reader - Port input data"]
pub type Idt7R = crate::BitReader;
#[doc = "Field `IDT8` reader - Port input data"]
pub type Idt8R = crate::BitReader;
#[doc = "Field `IDT9` reader - Port input data"]
pub type Idt9R = crate::BitReader;
#[doc = "Field `IDT10` reader - Port input data"]
pub type Idt10R = crate::BitReader;
#[doc = "Field `IDT11` reader - Port input data"]
pub type Idt11R = crate::BitReader;
#[doc = "Field `IDT12` reader - Port input data"]
pub type Idt12R = crate::BitReader;
#[doc = "Field `IDT13` reader - Port input data"]
pub type Idt13R = crate::BitReader;
#[doc = "Field `IDT14` reader - Port input data"]
pub type Idt14R = crate::BitReader;
#[doc = "Field `IDT15` reader - Port input data"]
pub type Idt15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Port input data"]
    #[inline(always)]
    pub fn idt0(&self) -> Idt0R {
        Idt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input data"]
    #[inline(always)]
    pub fn idt1(&self) -> Idt1R {
        Idt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input data"]
    #[inline(always)]
    pub fn idt2(&self) -> Idt2R {
        Idt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input data"]
    #[inline(always)]
    pub fn idt3(&self) -> Idt3R {
        Idt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input data"]
    #[inline(always)]
    pub fn idt4(&self) -> Idt4R {
        Idt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input data"]
    #[inline(always)]
    pub fn idt5(&self) -> Idt5R {
        Idt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input data"]
    #[inline(always)]
    pub fn idt6(&self) -> Idt6R {
        Idt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input data"]
    #[inline(always)]
    pub fn idt7(&self) -> Idt7R {
        Idt7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input data"]
    #[inline(always)]
    pub fn idt8(&self) -> Idt8R {
        Idt8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input data"]
    #[inline(always)]
    pub fn idt9(&self) -> Idt9R {
        Idt9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input data"]
    #[inline(always)]
    pub fn idt10(&self) -> Idt10R {
        Idt10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input data"]
    #[inline(always)]
    pub fn idt11(&self) -> Idt11R {
        Idt11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input data"]
    #[inline(always)]
    pub fn idt12(&self) -> Idt12R {
        Idt12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input data"]
    #[inline(always)]
    pub fn idt13(&self) -> Idt13R {
        Idt13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input data"]
    #[inline(always)]
    pub fn idt14(&self) -> Idt14R {
        Idt14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input data"]
    #[inline(always)]
    pub fn idt15(&self) -> Idt15R {
        Idt15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDT")
            .field("idt0", &self.idt0())
            .field("idt1", &self.idt1())
            .field("idt2", &self.idt2())
            .field("idt3", &self.idt3())
            .field("idt4", &self.idt4())
            .field("idt5", &self.idt5())
            .field("idt6", &self.idt6())
            .field("idt7", &self.idt7())
            .field("idt8", &self.idt8())
            .field("idt9", &self.idt9())
            .field("idt10", &self.idt10())
            .field("idt11", &self.idt11())
            .field("idt12", &self.idt12())
            .field("idt13", &self.idt13())
            .field("idt14", &self.idt14())
            .field("idt15", &self.idt15())
            .finish()
    }
}
#[doc = "Port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdtSpec;
impl crate::RegisterSpec for IdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idt::R`](R) reader structure"]
impl crate::Readable for IdtSpec {}
#[doc = "`reset()` method sets IDT to value 0"]
impl crate::Resettable for IdtSpec {
    const RESET_VALUE: u32 = 0;
}
