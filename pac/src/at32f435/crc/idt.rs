#[doc = "Register `IDT` reader"]
pub type R = crate::R<IdtSpec>;
#[doc = "Register `IDT` writer"]
pub type W = crate::W<IdtSpec>;
#[doc = "Field `IDT` reader - Initial Data"]
pub type IdtR = crate::FieldReader<u32>;
#[doc = "Field `IDT` writer - Initial Data"]
pub type IdtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initial Data"]
    #[inline(always)]
    pub fn idt(&self) -> IdtR {
        IdtR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDT").field("idt", &self.idt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Initial Data"]
    #[inline(always)]
    #[must_use]
    pub fn idt(&mut self) -> IdtW<IdtSpec> {
        IdtW::new(self, 0)
    }
}
#[doc = "Initial data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdtSpec;
impl crate::RegisterSpec for IdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idt::R`](R) reader structure"]
impl crate::Readable for IdtSpec {}
#[doc = "`write(|w| ..)` method takes [`idt::W`](W) writer structure"]
impl crate::Writable for IdtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDT to value 0xffff_ffff"]
impl crate::Resettable for IdtSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
