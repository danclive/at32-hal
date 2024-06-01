#[doc = "Register `EXINTC4` reader"]
pub type R = crate::R<Exintc4Spec>;
#[doc = "Register `EXINTC4` writer"]
pub type W = crate::W<Exintc4Spec>;
#[doc = "Field `EXINT12` reader - Configure EXINT12 source"]
pub type Exint12R = crate::FieldReader;
#[doc = "Field `EXINT12` writer - Configure EXINT12 source"]
pub type Exint12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT13` reader - Configure EXINT13 source"]
pub type Exint13R = crate::FieldReader;
#[doc = "Field `EXINT13` writer - Configure EXINT13 source"]
pub type Exint13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT14` reader - Configure EXINT14 source"]
pub type Exint14R = crate::FieldReader;
#[doc = "Field `EXINT14` writer - Configure EXINT14 source"]
pub type Exint14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT15` reader - Configure EXINT15 source"]
pub type Exint15R = crate::FieldReader;
#[doc = "Field `EXINT15` writer - Configure EXINT15 source"]
pub type Exint15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure EXINT12 source"]
    #[inline(always)]
    pub fn exint12(&self) -> Exint12R {
        Exint12R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure EXINT13 source"]
    #[inline(always)]
    pub fn exint13(&self) -> Exint13R {
        Exint13R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure EXINT14 source"]
    #[inline(always)]
    pub fn exint14(&self) -> Exint14R {
        Exint14R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configure EXINT15 source"]
    #[inline(always)]
    pub fn exint15(&self) -> Exint15R {
        Exint15R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC4")
            .field("exint12", &self.exint12())
            .field("exint13", &self.exint13())
            .field("exint14", &self.exint14())
            .field("exint15", &self.exint15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure EXINT12 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint12(&mut self) -> Exint12W<Exintc4Spec> {
        Exint12W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configure EXINT13 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint13(&mut self) -> Exint13W<Exintc4Spec> {
        Exint13W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Configure EXINT14 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint14(&mut self) -> Exint14W<Exintc4Spec> {
        Exint14W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Configure EXINT15 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint15(&mut self) -> Exint15W<Exintc4Spec> {
        Exint15W::new(self, 12)
    }
}
#[doc = "External interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Exintc4Spec;
impl crate::RegisterSpec for Exintc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc4::R`](R) reader structure"]
impl crate::Readable for Exintc4Spec {}
#[doc = "`write(|w| ..)` method takes [`exintc4::W`](W) writer structure"]
impl crate::Writable for Exintc4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXINTC4 to value 0"]
impl crate::Resettable for Exintc4Spec {
    const RESET_VALUE: u32 = 0;
}
