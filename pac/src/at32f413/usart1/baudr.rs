#[doc = "Register `BAUDR` reader"]
pub type R = crate::R<BaudrSpec>;
#[doc = "Register `BAUDR` writer"]
pub type W = crate::W<BaudrSpec>;
#[doc = "Field `DIV` reader - Division"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Division"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Division"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BAUDR").field("div", &self.div()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Division"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<BaudrSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baudr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudrSpec;
impl crate::RegisterSpec for BaudrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudr::R`](R) reader structure"]
impl crate::Readable for BaudrSpec {}
#[doc = "`write(|w| ..)` method takes [`baudr::W`](W) writer structure"]
impl crate::Writable for BaudrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BAUDR to value 0"]
impl crate::Resettable for BaudrSpec {
    const RESET_VALUE: u32 = 0;
}
