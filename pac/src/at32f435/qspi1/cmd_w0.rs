#[doc = "Register `CMD_W0` reader"]
pub type R = crate::R<CmdW0Spec>;
#[doc = "Register `CMD_W0` writer"]
pub type W = crate::W<CmdW0Spec>;
#[doc = "Field `SPIADR` reader - SPI flash address"]
pub type SpiadrR = crate::FieldReader<u32>;
#[doc = "Field `SPIADR` writer - SPI flash address"]
pub type SpiadrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPI flash address"]
    #[inline(always)]
    pub fn spiadr(&self) -> SpiadrR {
        SpiadrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_W0")
            .field("spiadr", &self.spiadr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI flash address"]
    #[inline(always)]
    #[must_use]
    pub fn spiadr(&mut self) -> SpiadrW<CmdW0Spec> {
        SpiadrW::new(self, 0)
    }
}
#[doc = "Command word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdW0Spec;
impl crate::RegisterSpec for CmdW0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_w0::R`](R) reader structure"]
impl crate::Readable for CmdW0Spec {}
#[doc = "`write(|w| ..)` method takes [`cmd_w0::W`](W) writer structure"]
impl crate::Writable for CmdW0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_W0 to value 0"]
impl crate::Resettable for CmdW0Spec {
    const RESET_VALUE: u32 = 0;
}
