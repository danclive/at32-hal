#[doc = "Register `CM2_OUTPUT` reader"]
pub type R = crate::R<Cm2OutputSpec>;
#[doc = "Register `CM2_OUTPUT` writer"]
pub type W = crate::W<Cm2OutputSpec>;
#[doc = "Field `C3C` reader - Channel 3 configure"]
pub type C3cR = crate::FieldReader;
#[doc = "Field `C3C` writer - Channel 3 configure"]
pub type C3cW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C3OIEN` reader - Channel 3 output immediately enable"]
pub type C3oienR = crate::BitReader;
#[doc = "Field `C3OIEN` writer - Channel 3 output immediately enable"]
pub type C3oienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3OBEN` reader - Channel 3 output buffer enable"]
pub type C3obenR = crate::BitReader;
#[doc = "Field `C3OBEN` writer - Channel 3 output buffer enable"]
pub type C3obenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3OCTRL` reader - Channel 3 output control"]
pub type C3octrlR = crate::FieldReader;
#[doc = "Field `C3OCTRL` writer - Channel 3 output control"]
pub type C3octrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C3OSEN` reader - Channel 3 output switch enable"]
pub type C3osenR = crate::BitReader;
#[doc = "Field `C3OSEN` writer - Channel 3 output switch enable"]
pub type C3osenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4C` reader - Channel 4 configure"]
pub type C4cR = crate::FieldReader;
#[doc = "Field `C4C` writer - Channel 4 configure"]
pub type C4cW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C4OIEN` reader - Channel 4 output immediately enable"]
pub type C4oienR = crate::BitReader;
#[doc = "Field `C4OIEN` writer - Channel 4 output immediately enable"]
pub type C4oienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4OBEN` reader - Channel 4 output buffer enable"]
pub type C4obenR = crate::BitReader;
#[doc = "Field `C4OBEN` writer - Channel 4 output buffer enable"]
pub type C4obenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4OCTRL` reader - Channel 4 output control"]
pub type C4octrlR = crate::FieldReader;
#[doc = "Field `C4OCTRL` writer - Channel 4 output control"]
pub type C4octrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C4OSEN` reader - Channel 4 output switch enable"]
pub type C4osenR = crate::BitReader;
#[doc = "Field `C4OSEN` writer - Channel 4 output switch enable"]
pub type C4osenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Channel 3 configure"]
    #[inline(always)]
    pub fn c3c(&self) -> C3cR {
        C3cR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 3 output immediately enable"]
    #[inline(always)]
    pub fn c3oien(&self) -> C3oienR {
        C3oienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 output buffer enable"]
    #[inline(always)]
    pub fn c3oben(&self) -> C3obenR {
        C3obenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 3 output control"]
    #[inline(always)]
    pub fn c3octrl(&self) -> C3octrlR {
        C3octrlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 3 output switch enable"]
    #[inline(always)]
    pub fn c3osen(&self) -> C3osenR {
        C3osenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    pub fn c4c(&self) -> C4cR {
        C4cR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel 4 output immediately enable"]
    #[inline(always)]
    pub fn c4oien(&self) -> C4oienR {
        C4oienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 4 output buffer enable"]
    #[inline(always)]
    pub fn c4oben(&self) -> C4obenR {
        C4obenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 4 output control"]
    #[inline(always)]
    pub fn c4octrl(&self) -> C4octrlR {
        C4octrlR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Channel 4 output switch enable"]
    #[inline(always)]
    pub fn c4osen(&self) -> C4osenR {
        C4osenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM2_OUTPUT")
            .field("c4osen", &self.c4osen())
            .field("c4octrl", &self.c4octrl())
            .field("c4oben", &self.c4oben())
            .field("c4oien", &self.c4oien())
            .field("c4c", &self.c4c())
            .field("c3osen", &self.c3osen())
            .field("c3octrl", &self.c3octrl())
            .field("c3oben", &self.c3oben())
            .field("c3oien", &self.c3oien())
            .field("c3c", &self.c3c())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 3 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c3c(&mut self) -> C3cW<Cm2OutputSpec> {
        C3cW::new(self, 0)
    }
    #[doc = "Bit 2 - Channel 3 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3oien(&mut self) -> C3oienW<Cm2OutputSpec> {
        C3oienW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3oben(&mut self) -> C3obenW<Cm2OutputSpec> {
        C3obenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Channel 3 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c3octrl(&mut self) -> C3octrlW<Cm2OutputSpec> {
        C3octrlW::new(self, 4)
    }
    #[doc = "Bit 7 - Channel 3 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3osen(&mut self) -> C3osenW<Cm2OutputSpec> {
        C3osenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c4c(&mut self) -> C4cW<Cm2OutputSpec> {
        C4cW::new(self, 8)
    }
    #[doc = "Bit 10 - Channel 4 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4oien(&mut self) -> C4oienW<Cm2OutputSpec> {
        C4oienW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 4 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4oben(&mut self) -> C4obenW<Cm2OutputSpec> {
        C4obenW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Channel 4 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c4octrl(&mut self) -> C4octrlW<Cm2OutputSpec> {
        C4octrlW::new(self, 12)
    }
    #[doc = "Bit 15 - Channel 4 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4osen(&mut self) -> C4osenW<Cm2OutputSpec> {
        C4osenW::new(self, 15)
    }
}
#[doc = "Channel output mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm2_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm2_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm2OutputSpec;
impl crate::RegisterSpec for Cm2OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm2_output::R`](R) reader structure"]
impl crate::Readable for Cm2OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`cm2_output::W`](W) writer structure"]
impl crate::Writable for Cm2OutputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM2_OUTPUT to value 0"]
impl crate::Resettable for Cm2OutputSpec {
    const RESET_VALUE: u32 = 0;
}
