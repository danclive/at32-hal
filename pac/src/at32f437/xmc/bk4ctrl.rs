#[doc = "Register `BK4CTRL` reader"]
pub type R = crate::R<Bk4ctrlSpec>;
#[doc = "Register `BK4CTRL` writer"]
pub type W = crate::W<Bk4ctrlSpec>;
#[doc = "Field `NWEN` reader - Wait feature enable"]
pub type NwenR = crate::BitReader;
#[doc = "Field `NWEN` writer - Wait feature enable"]
pub type NwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Memory bank enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Memory bank enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn nwen(&self) -> NwenR {
        NwenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory bank enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK4CTRL")
            .field("en", &self.en())
            .field("nwen", &self.nwen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn nwen(&mut self) -> NwenW<Bk4ctrlSpec> {
        NwenW::new(self, 1)
    }
    #[doc = "Bit 2 - Memory bank enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Bk4ctrlSpec> {
        EnW::new(self, 2)
    }
}
#[doc = "PC Card/NAND Flash control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk4ctrlSpec;
impl crate::RegisterSpec for Bk4ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4ctrl::R`](R) reader structure"]
impl crate::Readable for Bk4ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bk4ctrl::W`](W) writer structure"]
impl crate::Writable for Bk4ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK4CTRL to value 0x18"]
impl crate::Resettable for Bk4ctrlSpec {
    const RESET_VALUE: u32 = 0x18;
}
