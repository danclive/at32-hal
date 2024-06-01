#[doc = "Register `CM3_OUTPUT` reader"]
pub type R = crate::R<Cm3OutputSpec>;
#[doc = "Register `CM3_OUTPUT` writer"]
pub type W = crate::W<Cm3OutputSpec>;
#[doc = "Field `C5OIEN` reader - Channel 5 output immediately enable"]
pub type C5oienR = crate::BitReader;
#[doc = "Field `C5OIEN` writer - Channel 5 output immediately enable"]
pub type C5oienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C5OBEN` reader - Channel 5 output buffer enable"]
pub type C5obenR = crate::BitReader;
#[doc = "Field `C5OBEN` writer - Channel 5 output buffer enable"]
pub type C5obenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C5OCTRL` reader - Channel 5 output control"]
pub type C5octrlR = crate::FieldReader;
#[doc = "Field `C5OCTRL` writer - Channel 5 output control"]
pub type C5octrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C5OSEN` reader - Channel 5 output switch enable"]
pub type C5osenR = crate::BitReader;
#[doc = "Field `C5OSEN` writer - Channel 5 output switch enable"]
pub type C5osenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Channel 5 output immediately enable"]
    #[inline(always)]
    pub fn c5oien(&self) -> C5oienR {
        C5oienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 5 output buffer enable"]
    #[inline(always)]
    pub fn c5oben(&self) -> C5obenR {
        C5obenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 5 output control"]
    #[inline(always)]
    pub fn c5octrl(&self) -> C5octrlR {
        C5octrlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 5 output switch enable"]
    #[inline(always)]
    pub fn c5osen(&self) -> C5osenR {
        C5osenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM3_OUTPUT")
            .field("c5osen", &self.c5osen())
            .field("c5octrl", &self.c5octrl())
            .field("c5oben", &self.c5oben())
            .field("c5oien", &self.c5oien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Channel 5 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c5oien(&mut self) -> C5oienW<Cm3OutputSpec> {
        C5oienW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 5 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c5oben(&mut self) -> C5obenW<Cm3OutputSpec> {
        C5obenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Channel 5 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c5octrl(&mut self) -> C5octrlW<Cm3OutputSpec> {
        C5octrlW::new(self, 4)
    }
    #[doc = "Bit 7 - Channel 5 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c5osen(&mut self) -> C5osenW<Cm3OutputSpec> {
        C5osenW::new(self, 7)
    }
}
#[doc = "Channel output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm3_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm3_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm3OutputSpec;
impl crate::RegisterSpec for Cm3OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm3_output::R`](R) reader structure"]
impl crate::Readable for Cm3OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`cm3_output::W`](W) writer structure"]
impl crate::Writable for Cm3OutputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM3_OUTPUT to value 0"]
impl crate::Resettable for Cm3OutputSpec {
    const RESET_VALUE: u32 = 0;
}
