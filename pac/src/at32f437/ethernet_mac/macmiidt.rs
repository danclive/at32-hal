#[doc = "Register `MACMIIDT` reader"]
pub type R = crate::R<MacmiidtSpec>;
#[doc = "Register `MACMIIDT` writer"]
pub type W = crate::W<MacmiidtSpec>;
#[doc = "Field `MD` reader - MII data"]
pub type MdR = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MII data"]
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACMIIDT").field("md", &self.md()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MdW<MacmiidtSpec> {
        MdW::new(self, 0)
    }
}
#[doc = "Ethernet MAC MII data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiidt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiidt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacmiidtSpec;
impl crate::RegisterSpec for MacmiidtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiidt::R`](R) reader structure"]
impl crate::Readable for MacmiidtSpec {}
#[doc = "`write(|w| ..)` method takes [`macmiidt::W`](W) writer structure"]
impl crate::Writable for MacmiidtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACMIIDT to value 0"]
impl crate::Resettable for MacmiidtSpec {
    const RESET_VALUE: u32 = 0;
}
