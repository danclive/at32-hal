#[doc = "Register `VMHB` reader"]
pub type R = crate::R<VmhbSpec>;
#[doc = "Register `VMHB` writer"]
pub type W = crate::W<VmhbSpec>;
#[doc = "Field `VMHB` reader - Voltage monitoring high boundary"]
pub type VmhbR = crate::FieldReader<u16>;
#[doc = "Field `VMHB` writer - Voltage monitoring high boundary"]
pub type VmhbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Voltage monitoring high boundary"]
    #[inline(always)]
    pub fn vmhb(&self) -> VmhbR {
        VmhbR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMHB").field("vmhb", &self.vmhb()).finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Voltage monitoring high boundary"]
    #[inline(always)]
    #[must_use]
    pub fn vmhb(&mut self) -> VmhbW<VmhbSpec> {
        VmhbW::new(self, 0)
    }
}
#[doc = "Voltage monitoring high boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmhb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmhb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VmhbSpec;
impl crate::RegisterSpec for VmhbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmhb::R`](R) reader structure"]
impl crate::Readable for VmhbSpec {}
#[doc = "`write(|w| ..)` method takes [`vmhb::W`](W) writer structure"]
impl crate::Writable for VmhbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VMHB to value 0x0fff"]
impl crate::Resettable for VmhbSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
