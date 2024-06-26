#[doc = "Register `CTRLSTS2` reader"]
pub type R = crate::R<Ctrlsts2Spec>;
#[doc = "Register `CTRLSTS2` writer"]
pub type W = crate::W<Ctrlsts2Spec>;
#[doc = "Field `COMP1NINVSEL` reader - Comparator1 non-inverting input selection"]
pub type Comp1ninvselR = crate::FieldReader;
#[doc = "Field `COMP1NINVSEL` writer - Comparator1 non-inverting input selection"]
pub type Comp1ninvselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2NINVSEL` reader - Comparator2 non-inverting input selection"]
pub type Comp2ninvselR = crate::FieldReader;
#[doc = "Field `COMP2NINVSEL` writer - Comparator2 non-inverting input selection"]
pub type Comp2ninvselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Comparator1 non-inverting input selection"]
    #[inline(always)]
    pub fn comp1ninvsel(&self) -> Comp1ninvselR {
        Comp1ninvselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - Comparator2 non-inverting input selection"]
    #[inline(always)]
    pub fn comp2ninvsel(&self) -> Comp2ninvselR {
        Comp2ninvselR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS2")
            .field("comp1ninvsel", &self.comp1ninvsel())
            .field("comp2ninvsel", &self.comp2ninvsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator1 non-inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1ninvsel(&mut self) -> Comp1ninvselW<Ctrlsts2Spec> {
        Comp1ninvselW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Comparator2 non-inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2ninvsel(&mut self) -> Comp2ninvselW<Ctrlsts2Spec> {
        Comp2ninvselW::new(self, 16)
    }
}
#[doc = "CMP control/status register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrlsts2Spec;
impl crate::RegisterSpec for Ctrlsts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts2::R`](R) reader structure"]
impl crate::Readable for Ctrlsts2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts2::W`](W) writer structure"]
impl crate::Writable for Ctrlsts2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLSTS2 to value 0"]
impl crate::Resettable for Ctrlsts2Spec {
    const RESET_VALUE: u32 = 0;
}
