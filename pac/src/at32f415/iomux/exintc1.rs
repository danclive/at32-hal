#[doc = "Register `EXINTC1` reader"]
pub type R = crate::R<Exintc1Spec>;
#[doc = "Register `EXINTC1` writer"]
pub type W = crate::W<Exintc1Spec>;
#[doc = "Field `EXINT0` reader - Configure EXINT0 source"]
pub type Exint0R = crate::FieldReader;
#[doc = "Field `EXINT0` writer - Configure EXINT0 source"]
pub type Exint0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT1` reader - Configure EXINT1 source"]
pub type Exint1R = crate::FieldReader;
#[doc = "Field `EXINT1` writer - Configure EXINT1 source"]
pub type Exint1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT2` reader - Configure EXINT2 source"]
pub type Exint2R = crate::FieldReader;
#[doc = "Field `EXINT2` writer - Configure EXINT2 source"]
pub type Exint2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT3` reader - Configure EXINT3 source"]
pub type Exint3R = crate::FieldReader;
#[doc = "Field `EXINT3` writer - Configure EXINT3 source"]
pub type Exint3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure EXINT0 source"]
    #[inline(always)]
    pub fn exint0(&self) -> Exint0R {
        Exint0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure EXINT1 source"]
    #[inline(always)]
    pub fn exint1(&self) -> Exint1R {
        Exint1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure EXINT2 source"]
    #[inline(always)]
    pub fn exint2(&self) -> Exint2R {
        Exint2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configure EXINT3 source"]
    #[inline(always)]
    pub fn exint3(&self) -> Exint3R {
        Exint3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC1")
            .field("exint0", &self.exint0())
            .field("exint1", &self.exint1())
            .field("exint2", &self.exint2())
            .field("exint3", &self.exint3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure EXINT0 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint0(&mut self) -> Exint0W<Exintc1Spec> {
        Exint0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configure EXINT1 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint1(&mut self) -> Exint1W<Exintc1Spec> {
        Exint1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Configure EXINT2 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint2(&mut self) -> Exint2W<Exintc1Spec> {
        Exint2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Configure EXINT3 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint3(&mut self) -> Exint3W<Exintc1Spec> {
        Exint3W::new(self, 12)
    }
}
#[doc = "External interrupt configuration register 1 (IOMUX_EXINTC1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Exintc1Spec;
impl crate::RegisterSpec for Exintc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc1::R`](R) reader structure"]
impl crate::Readable for Exintc1Spec {}
#[doc = "`write(|w| ..)` method takes [`exintc1::W`](W) writer structure"]
impl crate::Writable for Exintc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXINTC1 to value 0"]
impl crate::Resettable for Exintc1Spec {
    const RESET_VALUE: u32 = 0;
}
