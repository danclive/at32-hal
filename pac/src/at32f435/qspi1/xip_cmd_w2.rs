#[doc = "Register `XIP_CMD_W2` reader"]
pub type R = crate::R<XipCmdW2Spec>;
#[doc = "Register `XIP_CMD_W2` writer"]
pub type W = crate::W<XipCmdW2Spec>;
#[doc = "Field `XIPR_DCNT` reader - XIP read data counter"]
pub type XiprDcntR = crate::FieldReader;
#[doc = "Field `XIPR_DCNT` writer - XIP read data counter"]
pub type XiprDcntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `XIPR_TCNT` reader - XIP continue read cycle counter"]
pub type XiprTcntR = crate::FieldReader;
#[doc = "Field `XIPR_TCNT` writer - XIP continue read cycle counter"]
pub type XiprTcntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XIPR_SEL` reader - XIP read continue mode select"]
pub type XiprSelR = crate::BitReader;
#[doc = "Field `XIPR_SEL` writer - XIP read continue mode select"]
pub type XiprSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPW_DCNT` reader - XIP write data counter"]
pub type XipwDcntR = crate::FieldReader;
#[doc = "Field `XIPW_DCNT` writer - XIP write data counter"]
pub type XipwDcntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `XIPW_TCNT` reader - XIP continue write cycle counter"]
pub type XipwTcntR = crate::FieldReader;
#[doc = "Field `XIPW_TCNT` writer - XIP continue write cycle counter"]
pub type XipwTcntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XIPW_SEL` reader - XIP write continue mode select"]
pub type XipwSelR = crate::BitReader;
#[doc = "Field `XIPW_SEL` writer - XIP write continue mode select"]
pub type XipwSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - XIP read data counter"]
    #[inline(always)]
    pub fn xipr_dcnt(&self) -> XiprDcntR {
        XiprDcntR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:14 - XIP continue read cycle counter"]
    #[inline(always)]
    pub fn xipr_tcnt(&self) -> XiprTcntR {
        XiprTcntR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - XIP read continue mode select"]
    #[inline(always)]
    pub fn xipr_sel(&self) -> XiprSelR {
        XiprSelR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - XIP write data counter"]
    #[inline(always)]
    pub fn xipw_dcnt(&self) -> XipwDcntR {
        XipwDcntR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:30 - XIP continue write cycle counter"]
    #[inline(always)]
    pub fn xipw_tcnt(&self) -> XipwTcntR {
        XipwTcntR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - XIP write continue mode select"]
    #[inline(always)]
    pub fn xipw_sel(&self) -> XipwSelR {
        XipwSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIP_CMD_W2")
            .field("xipr_dcnt", &self.xipr_dcnt())
            .field("xipr_tcnt", &self.xipr_tcnt())
            .field("xipr_sel", &self.xipr_sel())
            .field("xipw_dcnt", &self.xipw_dcnt())
            .field("xipw_tcnt", &self.xipw_tcnt())
            .field("xipw_sel", &self.xipw_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - XIP read data counter"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_dcnt(&mut self) -> XiprDcntW<XipCmdW2Spec> {
        XiprDcntW::new(self, 0)
    }
    #[doc = "Bits 8:14 - XIP continue read cycle counter"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_tcnt(&mut self) -> XiprTcntW<XipCmdW2Spec> {
        XiprTcntW::new(self, 8)
    }
    #[doc = "Bit 15 - XIP read continue mode select"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_sel(&mut self) -> XiprSelW<XipCmdW2Spec> {
        XiprSelW::new(self, 15)
    }
    #[doc = "Bits 16:20 - XIP write data counter"]
    #[inline(always)]
    #[must_use]
    pub fn xipw_dcnt(&mut self) -> XipwDcntW<XipCmdW2Spec> {
        XipwDcntW::new(self, 16)
    }
    #[doc = "Bits 24:30 - XIP continue write cycle counter"]
    #[inline(always)]
    #[must_use]
    pub fn xipw_tcnt(&mut self) -> XipwTcntW<XipCmdW2Spec> {
        XipwTcntW::new(self, 24)
    }
    #[doc = "Bit 31 - XIP write continue mode select"]
    #[inline(always)]
    #[must_use]
    pub fn xipw_sel(&mut self) -> XipwSelW<XipCmdW2Spec> {
        XipwSelW::new(self, 31)
    }
}
#[doc = "XIP command word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipCmdW2Spec;
impl crate::RegisterSpec for XipCmdW2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_cmd_w2::R`](R) reader structure"]
impl crate::Readable for XipCmdW2Spec {}
#[doc = "`write(|w| ..)` method takes [`xip_cmd_w2::W`](W) writer structure"]
impl crate::Writable for XipCmdW2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIP_CMD_W2 to value 0"]
impl crate::Resettable for XipCmdW2Spec {
    const RESET_VALUE: u32 = 0;
}
