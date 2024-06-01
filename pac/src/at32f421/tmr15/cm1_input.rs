#[doc = "Register `CM1_INPUT` reader"]
pub type R = crate::R<Cm1InputSpec>;
#[doc = "Register `CM1_INPUT` writer"]
pub type W = crate::W<Cm1InputSpec>;
#[doc = "Field `C1C` reader - Channel 1 configure"]
pub type C1cR = crate::FieldReader;
#[doc = "Field `C1C` writer - Channel 1 configure"]
pub type C1cW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C1IDIV` reader - Channel 1 input divider"]
pub type C1idivR = crate::FieldReader;
#[doc = "Field `C1IDIV` writer - Channel 1 input divider"]
pub type C1idivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C1DF` reader - Channel 1 digital filter"]
pub type C1dfR = crate::FieldReader;
#[doc = "Field `C1DF` writer - Channel 1 digital filter"]
pub type C1dfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C2C` reader - Channel 2 configure"]
pub type C2cR = crate::FieldReader;
#[doc = "Field `C2C` writer - Channel 2 configure"]
pub type C2cW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C2IDIV` reader - Channel 2 input divider"]
pub type C2idivR = crate::FieldReader;
#[doc = "Field `C2IDIV` writer - Channel 2 input divider"]
pub type C2idivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C2DF` reader - Channel 2 digital filter"]
pub type C2dfR = crate::FieldReader;
#[doc = "Field `C2DF` writer - Channel 2 digital filter"]
pub type C2dfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&self) -> C1cR {
        C1cR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    pub fn c1idiv(&self) -> C1idivR {
        C1idivR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    pub fn c1df(&self) -> C1dfR {
        C1dfR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    pub fn c2c(&self) -> C2cR {
        C2cR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 2 input divider"]
    #[inline(always)]
    pub fn c2idiv(&self) -> C2idivR {
        C2idivR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 2 digital filter"]
    #[inline(always)]
    pub fn c2df(&self) -> C2dfR {
        C2dfR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM1_INPUT")
            .field("c2df", &self.c2df())
            .field("c2idiv", &self.c2idiv())
            .field("c2c", &self.c2c())
            .field("c1df", &self.c1df())
            .field("c1idiv", &self.c1idiv())
            .field("c1c", &self.c1c())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c1c(&mut self) -> C1cW<Cm1InputSpec> {
        C1cW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    #[must_use]
    pub fn c1idiv(&mut self) -> C1idivW<Cm1InputSpec> {
        C1idivW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn c1df(&mut self) -> C1dfW<Cm1InputSpec> {
        C1dfW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c2c(&mut self) -> C2cW<Cm1InputSpec> {
        C2cW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel 2 input divider"]
    #[inline(always)]
    #[must_use]
    pub fn c2idiv(&mut self) -> C2idivW<Cm1InputSpec> {
        C2idivW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Channel 2 digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn c2df(&mut self) -> C2dfW<Cm1InputSpec> {
        C2dfW::new(self, 12)
    }
}
#[doc = "Channel input mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm1InputSpec;
impl crate::RegisterSpec for Cm1InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm1_input::R`](R) reader structure"]
impl crate::Readable for Cm1InputSpec {}
#[doc = "`write(|w| ..)` method takes [`cm1_input::W`](W) writer structure"]
impl crate::Writable for Cm1InputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM1_INPUT to value 0"]
impl crate::Resettable for Cm1InputSpec {
    const RESET_VALUE: u32 = 0;
}
