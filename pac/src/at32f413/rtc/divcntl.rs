#[doc = "Register `DIVCNTL` reader"]
pub type R = crate::R<DivcntlSpec>;
#[doc = "Register `DIVCNTL` writer"]
pub type W = crate::W<DivcntlSpec>;
#[doc = "Field `DIVCNT` reader - RTC divider register low"]
pub type DivcntR = crate::FieldReader<u16>;
#[doc = "Field `DIVCNT` writer - RTC divider register low"]
pub type DivcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC divider register low"]
    #[inline(always)]
    pub fn divcnt(&self) -> DivcntR {
        DivcntR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVCNTL")
            .field("divcnt", &self.divcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC divider register low"]
    #[inline(always)]
    #[must_use]
    pub fn divcnt(&mut self) -> DivcntW<DivcntlSpec> {
        DivcntW::new(self, 0)
    }
}
#[doc = "RTC Divider Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divcntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divcntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivcntlSpec;
impl crate::RegisterSpec for DivcntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divcntl::R`](R) reader structure"]
impl crate::Readable for DivcntlSpec {}
#[doc = "`write(|w| ..)` method takes [`divcntl::W`](W) writer structure"]
impl crate::Writable for DivcntlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVCNTL to value 0x8000"]
impl crate::Resettable for DivcntlSpec {
    const RESET_VALUE: u32 = 0x8000;
}
