#[doc = "Register `REV` reader"]
pub type R = crate::R<RevSpec>;
#[doc = "Register `REV` writer"]
pub type W = crate::W<RevSpec>;
#[doc = "Field `REVISION` reader - Revision number"]
pub type RevisionR = crate::FieldReader<u32>;
#[doc = "Field `REVISION` writer - Revision number"]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - Revision number"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(self.bits & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REV")
            .field("revision", &self.revision())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - Revision number"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<RevSpec> {
        RevisionW::new(self, 0)
    }
}
#[doc = "Revision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevSpec;
impl crate::RegisterSpec for RevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rev::R`](R) reader structure"]
impl crate::Readable for RevSpec {}
#[doc = "`write(|w| ..)` method takes [`rev::W`](W) writer structure"]
impl crate::Writable for RevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REV to value 0x0001_0500"]
impl crate::Resettable for RevSpec {
    const RESET_VALUE: u32 = 0x0001_0500;
}
