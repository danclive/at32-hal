#[doc = "Register `EXINTC2` reader"]
pub type R = crate::R<Exintc2Spec>;
#[doc = "Register `EXINTC2` writer"]
pub type W = crate::W<Exintc2Spec>;
#[doc = "Field `EXINT4` reader - Configure EXINT4 source"]
pub type Exint4R = crate::FieldReader;
#[doc = "Field `EXINT4` writer - Configure EXINT4 source"]
pub type Exint4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT5` reader - Configure EXINT5 source"]
pub type Exint5R = crate::FieldReader;
#[doc = "Field `EXINT5` writer - Configure EXINT5 source"]
pub type Exint5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT6` reader - Configure EXINT6 source"]
pub type Exint6R = crate::FieldReader;
#[doc = "Field `EXINT6` writer - Configure EXINT6 source"]
pub type Exint6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT7` reader - Configure EXINT7 source"]
pub type Exint7R = crate::FieldReader;
#[doc = "Field `EXINT7` writer - Configure EXINT7 source"]
pub type Exint7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure EXINT4 source"]
    #[inline(always)]
    pub fn exint4(&self) -> Exint4R {
        Exint4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure EXINT5 source"]
    #[inline(always)]
    pub fn exint5(&self) -> Exint5R {
        Exint5R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure EXINT6 source"]
    #[inline(always)]
    pub fn exint6(&self) -> Exint6R {
        Exint6R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configure EXINT7 source"]
    #[inline(always)]
    pub fn exint7(&self) -> Exint7R {
        Exint7R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC2")
            .field("exint4", &self.exint4())
            .field("exint5", &self.exint5())
            .field("exint6", &self.exint6())
            .field("exint7", &self.exint7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure EXINT4 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint4(&mut self) -> Exint4W<Exintc2Spec> {
        Exint4W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configure EXINT5 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint5(&mut self) -> Exint5W<Exintc2Spec> {
        Exint5W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Configure EXINT6 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint6(&mut self) -> Exint6W<Exintc2Spec> {
        Exint6W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Configure EXINT7 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint7(&mut self) -> Exint7W<Exintc2Spec> {
        Exint7W::new(self, 12)
    }
}
#[doc = "External interrupt configuration register 2 (IOMUX_EXINTC2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Exintc2Spec;
impl crate::RegisterSpec for Exintc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc2::R`](R) reader structure"]
impl crate::Readable for Exintc2Spec {}
#[doc = "`write(|w| ..)` method takes [`exintc2::W`](W) writer structure"]
impl crate::Writable for Exintc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXINTC2 to value 0"]
impl crate::Resettable for Exintc2Spec {
    const RESET_VALUE: u32 = 0;
}
