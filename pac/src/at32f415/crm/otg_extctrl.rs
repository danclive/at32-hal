#[doc = "Register `OTG_EXTCTRL` reader"]
pub type R = crate::R<OtgExtctrlSpec>;
#[doc = "Register `OTG_EXTCTRL` writer"]
pub type W = crate::W<OtgExtctrlSpec>;
#[doc = "Field `USBDIV_RST` reader - USB divider reset"]
pub type UsbdivRstR = crate::BitReader;
#[doc = "Field `USBDIV_RST` writer - USB divider reset"]
pub type UsbdivRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_RMPEN` reader - OTGFS end-point 3 remap enable"]
pub type Ep3RmpenR = crate::BitReader;
#[doc = "Field `EP3_RMPEN` writer - OTGFS end-point 3 remap enable"]
pub type Ep3RmpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - USB divider reset"]
    #[inline(always)]
    pub fn usbdiv_rst(&self) -> UsbdivRstR {
        UsbdivRstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OTGFS end-point 3 remap enable"]
    #[inline(always)]
    pub fn ep3_rmpen(&self) -> Ep3RmpenR {
        Ep3RmpenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_EXTCTRL")
            .field("usbdiv_rst", &self.usbdiv_rst())
            .field("ep3_rmpen", &self.ep3_rmpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - USB divider reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv_rst(&mut self) -> UsbdivRstW<OtgExtctrlSpec> {
        UsbdivRstW::new(self, 30)
    }
    #[doc = "Bit 31 - OTGFS end-point 3 remap enable"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_rmpen(&mut self) -> Ep3RmpenW<OtgExtctrlSpec> {
        Ep3RmpenW::new(self, 31)
    }
}
#[doc = "OTGFS external ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_extctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_extctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgExtctrlSpec;
impl crate::RegisterSpec for OtgExtctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_extctrl::R`](R) reader structure"]
impl crate::Readable for OtgExtctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_extctrl::W`](W) writer structure"]
impl crate::Writable for OtgExtctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_EXTCTRL to value 0"]
impl crate::Resettable for OtgExtctrlSpec {
    const RESET_VALUE: u32 = 0;
}
