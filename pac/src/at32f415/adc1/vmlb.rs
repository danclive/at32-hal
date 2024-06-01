#[doc = "Register `VMLB` reader"]
pub type R = crate::R<VmlbSpec>;
#[doc = "Register `VMLB` writer"]
pub type W = crate::W<VmlbSpec>;
#[doc = "Field `VMLB` reader - Voltage monitoring low boundary"]
pub type VmlbR = crate::FieldReader<u16>;
#[doc = "Field `VMLB` writer - Voltage monitoring low boundary"]
pub type VmlbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Voltage monitoring low boundary"]
    #[inline(always)]
    pub fn vmlb(&self) -> VmlbR {
        VmlbR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMLB").field("vmlb", &self.vmlb()).finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Voltage monitoring low boundary"]
    #[inline(always)]
    #[must_use]
    pub fn vmlb(&mut self) -> VmlbW<VmlbSpec> {
        VmlbW::new(self, 0)
    }
}
#[doc = "Voltage monitoring low boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VmlbSpec;
impl crate::RegisterSpec for VmlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmlb::R`](R) reader structure"]
impl crate::Readable for VmlbSpec {}
#[doc = "`write(|w| ..)` method takes [`vmlb::W`](W) writer structure"]
impl crate::Writable for VmlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VMLB to value 0"]
impl crate::Resettable for VmlbSpec {
    const RESET_VALUE: u32 = 0;
}
