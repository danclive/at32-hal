#[doc = "Register `C3DT` reader"]
pub type R = crate::R<C3dtSpec>;
#[doc = "Register `C3DT` writer"]
pub type W = crate::W<C3dtSpec>;
#[doc = "Field `C3DT` reader - Channel 3 data register"]
pub type C3dtR = crate::FieldReader<u16>;
#[doc = "Field `C3DT` writer - Channel 3 data register"]
pub type C3dtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel 3 data register"]
    #[inline(always)]
    pub fn c3dt(&self) -> C3dtR {
        C3dtR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C3DT").field("c3dt", &self.c3dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel 3 data register"]
    #[inline(always)]
    #[must_use]
    pub fn c3dt(&mut self) -> C3dtW<C3dtSpec> {
        C3dtW::new(self, 0)
    }
}
#[doc = "Channel 3 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3dtSpec;
impl crate::RegisterSpec for C3dtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3dt::R`](R) reader structure"]
impl crate::Readable for C3dtSpec {}
#[doc = "`write(|w| ..)` method takes [`c3dt::W`](W) writer structure"]
impl crate::Writable for C3dtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C3DT to value 0"]
impl crate::Resettable for C3dtSpec {
    const RESET_VALUE: u32 = 0;
}
