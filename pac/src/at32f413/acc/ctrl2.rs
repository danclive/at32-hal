#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `HICKCAL` reader - Internal high-speed auto clock calibration"]
pub type HickcalR = crate::FieldReader;
#[doc = "Field `HICKTWK` reader - Internal high-speed auto clock trimming"]
pub type HicktwkR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Internal high-speed auto clock calibration"]
    #[inline(always)]
    pub fn hickcal(&self) -> HickcalR {
        HickcalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Internal high-speed auto clock trimming"]
    #[inline(always)]
    pub fn hicktwk(&self) -> HicktwkR {
        HicktwkR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("hicktwk", &self.hicktwk())
            .field("hickcal", &self.hickcal())
            .finish()
    }
}
impl W {}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL2 to value 0x2080"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0x2080;
}
