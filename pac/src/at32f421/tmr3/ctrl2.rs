#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
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
impl R {
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("c1insel", &self.c1insel())
            .field("ptos", &self.ptos())
            .field("drs", &self.drs())
            .finish()
    }
}
impl W {
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
