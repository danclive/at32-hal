#[doc = "Register `XIP_CMD_W3` reader"]
pub type R = crate::R<XipCmdW3Spec>;
#[doc = "Register `XIP_CMD_W3` writer"]
pub type W = crate::W<XipCmdW3Spec>;
#[doc = "Field `BYPASSC` reader - Bypass cache function"]
pub type BypasscR = crate::BitReader;
#[doc = "Field `BYPASSC` writer - Bypass cache function"]
pub type BypasscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTS` reader - Cache status"]
pub type CstsR = crate::BitReader;
#[doc = "Field `CSTS` writer - Cache status"]
pub type CstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bypass cache function"]
    #[inline(always)]
    pub fn bypassc(&self) -> BypasscR {
        BypasscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Cache status"]
    #[inline(always)]
    pub fn csts(&self) -> CstsR {
        CstsR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIP_CMD_W3")
            .field("bypassc", &self.bypassc())
            .field("csts", &self.csts())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Bypass cache function"]
    #[inline(always)]
    #[must_use]
    pub fn bypassc(&mut self) -> BypasscW<XipCmdW3Spec> {
        BypasscW::new(self, 0)
    }
    #[doc = "Bit 3 - Cache status"]
    #[inline(always)]
    #[must_use]
    pub fn csts(&mut self) -> CstsW<XipCmdW3Spec> {
        CstsW::new(self, 3)
    }
}
#[doc = "XIP command word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipCmdW3Spec;
impl crate::RegisterSpec for XipCmdW3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_cmd_w3::R`](R) reader structure"]
impl crate::Readable for XipCmdW3Spec {}
#[doc = "`write(|w| ..)` method takes [`xip_cmd_w3::W`](W) writer structure"]
impl crate::Writable for XipCmdW3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIP_CMD_W3 to value 0"]
impl crate::Resettable for XipCmdW3Spec {
    const RESET_VALUE: u32 = 0;
}
