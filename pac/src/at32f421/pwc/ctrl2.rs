#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `VREXLPEN` reader - Voltage regulator extra low power mode enable"]
pub type VrexlpenR = crate::BitReader;
#[doc = "Field `VREXLPEN` writer - Voltage regulator extra low power mode enable"]
pub type VrexlpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Voltage regulator extra low power mode enable"]
    #[inline(always)]
    pub fn vrexlpen(&self) -> VrexlpenR {
        VrexlpenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("vrexlpen", &self.vrexlpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - Voltage regulator extra low power mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrexlpen(&mut self) -> VrexlpenW<Ctrl2Spec> {
        VrexlpenW::new(self, 5)
    }
}
#[doc = "Power control and status register2 (PWC_CTRL2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
