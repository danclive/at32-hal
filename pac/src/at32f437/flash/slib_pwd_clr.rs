#[doc = "Register `SLIB_PWD_CLR` writer"]
pub type W = crate::W<SlibPwdClrSpec>;
#[doc = "Field `SLIB_PCLR_VAL` writer - sLib password clear value"]
pub type SlibPclrValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SlibPwdClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - sLib password clear value"]
    #[inline(always)]
    #[must_use]
    pub fn slib_pclr_val(&mut self) -> SlibPclrValW<SlibPwdClrSpec> {
        SlibPclrValW::new(self, 0)
    }
}
#[doc = "SLIB password clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_pwd_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibPwdClrSpec;
impl crate::RegisterSpec for SlibPwdClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_pwd_clr::W`](W) writer structure"]
impl crate::Writable for SlibPwdClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_PWD_CLR to value 0"]
impl crate::Resettable for SlibPwdClrSpec {
    const RESET_VALUE: u32 = 0;
}
