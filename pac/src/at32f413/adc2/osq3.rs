#[doc = "Register `OSQ3` reader"]
pub type R = crate::R<Osq3Spec>;
#[doc = "Register `OSQ3` writer"]
pub type W = crate::W<Osq3Spec>;
#[doc = "Field `OSN1` reader - Number of 1st conversion in ordinary sequence"]
pub type Osn1R = crate::FieldReader;
#[doc = "Field `OSN1` writer - Number of 1st conversion in ordinary sequence"]
pub type Osn1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN2` reader - Number of 2nd conversion in ordinary sequence"]
pub type Osn2R = crate::FieldReader;
#[doc = "Field `OSN2` writer - Number of 2nd conversion in ordinary sequence"]
pub type Osn2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN3` reader - number of 3rd conversion in ordinary sequence"]
pub type Osn3R = crate::FieldReader;
#[doc = "Field `OSN3` writer - number of 3rd conversion in ordinary sequence"]
pub type Osn3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN4` reader - Number of 4th conversion in ordinary sequence"]
pub type Osn4R = crate::FieldReader;
#[doc = "Field `OSN4` writer - Number of 4th conversion in ordinary sequence"]
pub type Osn4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN5` reader - Number of 5th conversion in ordinary sequence"]
pub type Osn5R = crate::FieldReader;
#[doc = "Field `OSN5` writer - Number of 5th conversion in ordinary sequence"]
pub type Osn5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN6` reader - Number of 6th conversion in ordinary sequence"]
pub type Osn6R = crate::FieldReader;
#[doc = "Field `OSN6` writer - Number of 6th conversion in ordinary sequence"]
pub type Osn6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number of 1st conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn1(&self) -> Osn1R {
        Osn1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 2nd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn2(&self) -> Osn2R {
        Osn2R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - number of 3rd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn3(&self) -> Osn3R {
        Osn3R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 4th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn4(&self) -> Osn4R {
        Osn4R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 5th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn5(&self) -> Osn5R {
        Osn5R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 6th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn6(&self) -> Osn6R {
        Osn6R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ3")
            .field("osn6", &self.osn6())
            .field("osn5", &self.osn5())
            .field("osn4", &self.osn4())
            .field("osn3", &self.osn3())
            .field("osn2", &self.osn2())
            .field("osn1", &self.osn1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 1st conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn1(&mut self) -> Osn1W<Osq3Spec> {
        Osn1W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 2nd conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn2(&mut self) -> Osn2W<Osq3Spec> {
        Osn2W::new(self, 5)
    }
    #[doc = "Bits 10:14 - number of 3rd conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn3(&mut self) -> Osn3W<Osq3Spec> {
        Osn3W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 4th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn4(&mut self) -> Osn4W<Osq3Spec> {
        Osn4W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Number of 5th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn5(&mut self) -> Osn5W<Osq3Spec> {
        Osn5W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Number of 6th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn6(&mut self) -> Osn6W<Osq3Spec> {
        Osn6W::new(self, 25)
    }
}
#[doc = "Ordinary sequence register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osq3Spec;
impl crate::RegisterSpec for Osq3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq3::R`](R) reader structure"]
impl crate::Readable for Osq3Spec {}
#[doc = "`write(|w| ..)` method takes [`osq3::W`](W) writer structure"]
impl crate::Writable for Osq3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSQ3 to value 0"]
impl crate::Resettable for Osq3Spec {
    const RESET_VALUE: u32 = 0;
}
