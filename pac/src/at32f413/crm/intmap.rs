#[doc = "Register `INTMAP` reader"]
pub type R = crate::R<IntmapSpec>;
#[doc = "Register `INTMAP` writer"]
pub type W = crate::W<IntmapSpec>;
#[doc = "Field `USB_INT_MAP` reader - USBDEV interrupt remap"]
pub type UsbIntMapR = crate::BitReader;
#[doc = "Field `USB_INT_MAP` writer - USBDEV interrupt remap"]
pub type UsbIntMapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USBDEV interrupt remap"]
    #[inline(always)]
    pub fn usb_int_map(&self) -> UsbIntMapR {
        UsbIntMapR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTMAP")
            .field("usb_int_map", &self.usb_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USBDEV interrupt remap"]
    #[inline(always)]
    #[must_use]
    pub fn usb_int_map(&mut self) -> UsbIntMapW<IntmapSpec> {
        UsbIntMapW::new(self, 0)
    }
}
#[doc = "Interrupt remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntmapSpec;
impl crate::RegisterSpec for IntmapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmap::R`](R) reader structure"]
impl crate::Readable for IntmapSpec {}
#[doc = "`write(|w| ..)` method takes [`intmap::W`](W) writer structure"]
impl crate::Writable for IntmapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTMAP to value 0"]
impl crate::Resettable for IntmapSpec {
    const RESET_VALUE: u32 = 0;
}
