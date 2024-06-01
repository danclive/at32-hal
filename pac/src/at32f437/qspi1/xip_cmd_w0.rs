#[doc = "Register `XIP_CMD_W0` reader"]
pub type R = crate::R<XipCmdW0Spec>;
#[doc = "Register `XIP_CMD_W0` writer"]
pub type W = crate::W<XipCmdW0Spec>;
#[doc = "Field `XIPR_DUM2` reader - XIP read second dummy cycle"]
pub type XiprDum2R = crate::FieldReader;
#[doc = "Field `XIPR_DUM2` writer - XIP read second dummy cycle"]
pub type XiprDum2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XIPR_OPMODE` reader - XIP read operate mode"]
pub type XiprOpmodeR = crate::FieldReader;
#[doc = "Field `XIPR_OPMODE` writer - XIP read operate mode"]
pub type XiprOpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XIPR_ADRLEN` reader - XIP read address length"]
pub type XiprAdrlenR = crate::BitReader;
#[doc = "Field `XIPR_ADRLEN` writer - XIP read address length"]
pub type XiprAdrlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPR_INSC` reader - XIP read instruction code"]
pub type XiprInscR = crate::FieldReader;
#[doc = "Field `XIPR_INSC` writer - XIP read instruction code"]
pub type XiprInscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - XIP read second dummy cycle"]
    #[inline(always)]
    pub fn xipr_dum2(&self) -> XiprDum2R {
        XiprDum2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - XIP read operate mode"]
    #[inline(always)]
    pub fn xipr_opmode(&self) -> XiprOpmodeR {
        XiprOpmodeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - XIP read address length"]
    #[inline(always)]
    pub fn xipr_adrlen(&self) -> XiprAdrlenR {
        XiprAdrlenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - XIP read instruction code"]
    #[inline(always)]
    pub fn xipr_insc(&self) -> XiprInscR {
        XiprInscR::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIP_CMD_W0")
            .field("xipr_dum2", &self.xipr_dum2())
            .field("xipr_opmode", &self.xipr_opmode())
            .field("xipr_adrlen", &self.xipr_adrlen())
            .field("xipr_insc", &self.xipr_insc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - XIP read second dummy cycle"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_dum2(&mut self) -> XiprDum2W<XipCmdW0Spec> {
        XiprDum2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - XIP read operate mode"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_opmode(&mut self) -> XiprOpmodeW<XipCmdW0Spec> {
        XiprOpmodeW::new(self, 8)
    }
    #[doc = "Bit 11 - XIP read address length"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_adrlen(&mut self) -> XiprAdrlenW<XipCmdW0Spec> {
        XiprAdrlenW::new(self, 11)
    }
    #[doc = "Bits 12:19 - XIP read instruction code"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_insc(&mut self) -> XiprInscW<XipCmdW0Spec> {
        XiprInscW::new(self, 12)
    }
}
#[doc = "XIP command word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipCmdW0Spec;
impl crate::RegisterSpec for XipCmdW0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_cmd_w0::R`](R) reader structure"]
impl crate::Readable for XipCmdW0Spec {}
#[doc = "`write(|w| ..)` method takes [`xip_cmd_w0::W`](W) writer structure"]
impl crate::Writable for XipCmdW0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIP_CMD_W0 to value 0"]
impl crate::Resettable for XipCmdW0Spec {
    const RESET_VALUE: u32 = 0;
}
