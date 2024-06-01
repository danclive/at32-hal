#[doc = "Register `CM1_OUTPUT` reader"]
pub type R = crate::R<Cm1OutputSpec>;
#[doc = "Register `CM1_OUTPUT` writer"]
pub type W = crate::W<Cm1OutputSpec>;
#[doc = "Field `C1C` reader - Channel 1 configure"]
pub type C1cR = crate::FieldReader;
#[doc = "Field `C1C` writer - Channel 1 configure"]
pub type C1cW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C1OIEN` reader - Channel 1 output immediately enable"]
pub type C1oienR = crate::BitReader;
#[doc = "Field `C1OIEN` writer - Channel 1 output immediately enable"]
pub type C1oienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1OBEN` reader - Channel 1 output buffer enable"]
pub type C1obenR = crate::BitReader;
#[doc = "Field `C1OBEN` writer - Channel 1 output buffer enable"]
pub type C1obenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1OCTRL` reader - Channel 1 output control"]
pub type C1octrlR = crate::FieldReader;
#[doc = "Field `C1OCTRL` writer - Channel 1 output control"]
pub type C1octrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C1OSEN` reader - Channel 1 output switch enable"]
pub type C1osenR = crate::BitReader;
#[doc = "Field `C1OSEN` writer - Channel 1 output switch enable"]
pub type C1osenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&self) -> C1cR {
        C1cR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 1 output immediately enable"]
    #[inline(always)]
    pub fn c1oien(&self) -> C1oienR {
        C1oienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 output buffer enable"]
    #[inline(always)]
    pub fn c1oben(&self) -> C1obenR {
        C1obenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 1 output control"]
    #[inline(always)]
    pub fn c1octrl(&self) -> C1octrlR {
        C1octrlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 1 output switch enable"]
    #[inline(always)]
    pub fn c1osen(&self) -> C1osenR {
        C1osenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM1_OUTPUT")
            .field("c1osen", &self.c1osen())
            .field("c1octrl", &self.c1octrl())
            .field("c1oben", &self.c1oben())
            .field("c1oien", &self.c1oien())
            .field("c1c", &self.c1c())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c1c(&mut self) -> C1cW<Cm1OutputSpec> {
        C1cW::new(self, 0)
    }
    #[doc = "Bit 2 - Channel 1 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1oien(&mut self) -> C1oienW<Cm1OutputSpec> {
        C1oienW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 1 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1oben(&mut self) -> C1obenW<Cm1OutputSpec> {
        C1obenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Channel 1 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c1octrl(&mut self) -> C1octrlW<Cm1OutputSpec> {
        C1octrlW::new(self, 4)
    }
    #[doc = "Bit 7 - Channel 1 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1osen(&mut self) -> C1osenW<Cm1OutputSpec> {
        C1osenW::new(self, 7)
    }
}
#[doc = "Channel output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm1OutputSpec;
impl crate::RegisterSpec for Cm1OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm1_output::R`](R) reader structure"]
impl crate::Readable for Cm1OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`cm1_output::W`](W) writer structure"]
impl crate::Writable for Cm1OutputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM1_OUTPUT to value 0"]
impl crate::Resettable for Cm1OutputSpec {
    const RESET_VALUE: u32 = 0;
}
