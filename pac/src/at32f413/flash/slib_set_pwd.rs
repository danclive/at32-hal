#[doc = "Register `SLIB_SET_PWD` writer"]
pub type W = crate::W<SlibSetPwdSpec>;
#[doc = "Field `SLIB_PSET_VAL` writer - sLib password setting val"]
pub type SlibPsetValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SlibSetPwdSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - sLib password setting val"]
    #[inline(always)]
    #[must_use]
    pub fn slib_pset_val(&mut self) -> SlibPsetValW<SlibSetPwdSpec> {
        SlibPsetValW::new(self, 0)
    }
}
#[doc = "sLib password setting register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_pwd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibSetPwdSpec;
impl crate::RegisterSpec for SlibSetPwdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_set_pwd::W`](W) writer structure"]
impl crate::Writable for SlibSetPwdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_SET_PWD to value 0"]
impl crate::Resettable for SlibSetPwdSpec {
    const RESET_VALUE: u32 = 0;
}
