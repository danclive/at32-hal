#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Field `MII_RMII_SEL` reader - MII or RMII selection bits"]
pub type MiiRmiiSelR = crate::BitReader;
#[doc = "Field `MII_RMII_SEL` writer - MII or RMII selection bits"]
pub type MiiRmiiSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23 - MII or RMII selection bits"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MiiRmiiSelR {
        MiiRmiiSelR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG2")
            .field("mii_rmii_sel", &self.mii_rmii_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 23 - MII or RMII selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mii_rmii_sel(&mut self) -> MiiRmiiSelW<Cfg2Spec> {
        MiiRmiiSelW::new(self, 23)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0;
}
