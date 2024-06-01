#[doc = "Register `VSCF` reader"]
pub type R = crate::R<VscfSpec>;
#[doc = "Register `VSCF` writer"]
pub type W = crate::W<VscfSpec>;
#[doc = "Field `VSRSF` reader - Vertical scaling resize source factor"]
pub type VsrsfR = crate::FieldReader<u16>;
#[doc = "Field `VSRSF` writer - Vertical scaling resize source factor"]
pub type VsrsfW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `VSRTF` reader - Vertical scaling resize target factor"]
pub type VsrtfR = crate::FieldReader<u16>;
#[doc = "Field `VSRTF` writer - Vertical scaling resize target factor"]
pub type VsrtfW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Vertical scaling resize source factor"]
    #[inline(always)]
    pub fn vsrsf(&self) -> VsrsfR {
        VsrsfR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Vertical scaling resize target factor"]
    #[inline(always)]
    pub fn vsrtf(&self) -> VsrtfR {
        VsrtfR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VSCF")
            .field("vsrtf", &self.vsrtf())
            .field("vsrsf", &self.vsrsf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - Vertical scaling resize source factor"]
    #[inline(always)]
    #[must_use]
    pub fn vsrsf(&mut self) -> VsrsfW<VscfSpec> {
        VsrsfW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Vertical scaling resize target factor"]
    #[inline(always)]
    #[must_use]
    pub fn vsrtf(&mut self) -> VsrtfW<VscfSpec> {
        VsrtfW::new(self, 16)
    }
}
#[doc = "Vertical scaling control flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vscf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vscf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VscfSpec;
impl crate::RegisterSpec for VscfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vscf::R`](R) reader structure"]
impl crate::Readable for VscfSpec {}
#[doc = "`write(|w| ..)` method takes [`vscf::W`](W) writer structure"]
impl crate::Writable for VscfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSCF to value 0"]
impl crate::Resettable for VscfSpec {
    const RESET_VALUE: u32 = 0;
}
