#[doc = "Register `C5DT` reader"]
pub type R = crate::R<C5dtSpec>;
#[doc = "Register `C5DT` writer"]
pub type W = crate::W<C5dtSpec>;
#[doc = "Field `C5DT` reader - Channel 5 data register"]
pub type C5dtR = crate::FieldReader<u16>;
#[doc = "Field `C5DT` writer - Channel 5 data register"]
pub type C5dtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel 5 data register"]
    #[inline(always)]
    pub fn c5dt(&self) -> C5dtR {
        C5dtR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C5DT").field("c5dt", &self.c5dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel 5 data register"]
    #[inline(always)]
    #[must_use]
    pub fn c5dt(&mut self) -> C5dtW<C5dtSpec> {
        C5dtW::new(self, 0)
    }
}
#[doc = "Channel 5 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C5dtSpec;
impl crate::RegisterSpec for C5dtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c5dt::R`](R) reader structure"]
impl crate::Readable for C5dtSpec {}
#[doc = "`write(|w| ..)` method takes [`c5dt::W`](W) writer structure"]
impl crate::Writable for C5dtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C5DT to value 0"]
impl crate::Resettable for C5dtSpec {
    const RESET_VALUE: u32 = 0;
}
