#[doc = "Register `LDOOV` reader"]
pub type R = crate::R<LdoovSpec>;
#[doc = "Register `LDOOV` writer"]
pub type W = crate::W<LdoovSpec>;
#[doc = "Field `LDOOVSEL` reader - LDO output voltage select"]
pub type LdoovselR = crate::FieldReader;
#[doc = "Field `LDOOVSEL` writer - LDO output voltage select"]
pub type LdoovselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldoovsel(&self) -> LdoovselR {
        LdoovselR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDOOV")
            .field("ldoovsel", &self.ldoovsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - LDO output voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn ldoovsel(&mut self) -> LdoovselW<LdoovSpec> {
        LdoovselW::new(self, 0)
    }
}
#[doc = "LDO output voltage register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldoov::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldoov::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LdoovSpec;
impl crate::RegisterSpec for LdoovSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldoov::R`](R) reader structure"]
impl crate::Readable for LdoovSpec {}
#[doc = "`write(|w| ..)` method takes [`ldoov::W`](W) writer structure"]
impl crate::Writable for LdoovSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDOOV to value 0"]
impl crate::Resettable for LdoovSpec {
    const RESET_VALUE: u32 = 0;
}
