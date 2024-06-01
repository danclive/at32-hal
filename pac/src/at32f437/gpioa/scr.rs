#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `IOSB0` writer - Set bit 0"]
pub type Iosb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB1` writer - Set bit 1"]
pub type Iosb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB2` writer - Set bit 1"]
pub type Iosb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB3` writer - Set bit 3"]
pub type Iosb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB4` writer - Set bit 4"]
pub type Iosb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB5` writer - Set bit 5"]
pub type Iosb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB6` writer - Set bit 6"]
pub type Iosb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB7` writer - Set bit 7"]
pub type Iosb7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB8` writer - Set bit 8"]
pub type Iosb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB9` writer - Set bit 9"]
pub type Iosb9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB10` writer - Set bit 10"]
pub type Iosb10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB11` writer - Set bit 11"]
pub type Iosb11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB12` writer - Set bit 12"]
pub type Iosb12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB13` writer - Set bit 13"]
pub type Iosb13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB14` writer - Set bit 14"]
pub type Iosb14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSB15` writer - Set bit 15"]
pub type Iosb15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB0` writer - Clear bit 0"]
pub type Iocb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB1` writer - Clear bit 1"]
pub type Iocb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB2` writer - Clear bit 2"]
pub type Iocb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB3` writer - Clear bit 3"]
pub type Iocb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB4` writer - Clear bit 4"]
pub type Iocb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB5` writer - Clear bit 5"]
pub type Iocb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB6` writer - Clear bit 6"]
pub type Iocb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB7` writer - Clear bit 7"]
pub type Iocb7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB8` writer - Clear bit 8"]
pub type Iocb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB9` writer - Clear bit 9"]
pub type Iocb9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB10` writer - Clear bit 10"]
pub type Iocb10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB11` writer - Clear bit 11"]
pub type Iocb11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB12` writer - Clear bit 12"]
pub type Iocb12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB13` writer - Clear bit 13"]
pub type Iocb13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB14` writer - Clear bit 14"]
pub type Iocb14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB15` writer - Clear bit 15"]
pub type Iocb15W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ScrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iosb0(&mut self) -> Iosb0W<ScrSpec> {
        Iosb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iosb1(&mut self) -> Iosb1W<ScrSpec> {
        Iosb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iosb2(&mut self) -> Iosb2W<ScrSpec> {
        Iosb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Set bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iosb3(&mut self) -> Iosb3W<ScrSpec> {
        Iosb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Set bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iosb4(&mut self) -> Iosb4W<ScrSpec> {
        Iosb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Set bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iosb5(&mut self) -> Iosb5W<ScrSpec> {
        Iosb5W::new(self, 5)
    }
    #[doc = "Bit 6 - Set bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iosb6(&mut self) -> Iosb6W<ScrSpec> {
        Iosb6W::new(self, 6)
    }
    #[doc = "Bit 7 - Set bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iosb7(&mut self) -> Iosb7W<ScrSpec> {
        Iosb7W::new(self, 7)
    }
    #[doc = "Bit 8 - Set bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iosb8(&mut self) -> Iosb8W<ScrSpec> {
        Iosb8W::new(self, 8)
    }
    #[doc = "Bit 9 - Set bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iosb9(&mut self) -> Iosb9W<ScrSpec> {
        Iosb9W::new(self, 9)
    }
    #[doc = "Bit 10 - Set bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iosb10(&mut self) -> Iosb10W<ScrSpec> {
        Iosb10W::new(self, 10)
    }
    #[doc = "Bit 11 - Set bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iosb11(&mut self) -> Iosb11W<ScrSpec> {
        Iosb11W::new(self, 11)
    }
    #[doc = "Bit 12 - Set bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iosb12(&mut self) -> Iosb12W<ScrSpec> {
        Iosb12W::new(self, 12)
    }
    #[doc = "Bit 13 - Set bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iosb13(&mut self) -> Iosb13W<ScrSpec> {
        Iosb13W::new(self, 13)
    }
    #[doc = "Bit 14 - Set bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iosb14(&mut self) -> Iosb14W<ScrSpec> {
        Iosb14W::new(self, 14)
    }
    #[doc = "Bit 15 - Set bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iosb15(&mut self) -> Iosb15W<ScrSpec> {
        Iosb15W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iocb0(&mut self) -> Iocb0W<ScrSpec> {
        Iocb0W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb1(&mut self) -> Iocb1W<ScrSpec> {
        Iocb1W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iocb2(&mut self) -> Iocb2W<ScrSpec> {
        Iocb2W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iocb3(&mut self) -> Iocb3W<ScrSpec> {
        Iocb3W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iocb4(&mut self) -> Iocb4W<ScrSpec> {
        Iocb4W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iocb5(&mut self) -> Iocb5W<ScrSpec> {
        Iocb5W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iocb6(&mut self) -> Iocb6W<ScrSpec> {
        Iocb6W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iocb7(&mut self) -> Iocb7W<ScrSpec> {
        Iocb7W::new(self, 23)
    }
    #[doc = "Bit 24 - Clear bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iocb8(&mut self) -> Iocb8W<ScrSpec> {
        Iocb8W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iocb9(&mut self) -> Iocb9W<ScrSpec> {
        Iocb9W::new(self, 25)
    }
    #[doc = "Bit 26 - Clear bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iocb10(&mut self) -> Iocb10W<ScrSpec> {
        Iocb10W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iocb11(&mut self) -> Iocb11W<ScrSpec> {
        Iocb11W::new(self, 27)
    }
    #[doc = "Bit 28 - Clear bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iocb12(&mut self) -> Iocb12W<ScrSpec> {
        Iocb12W::new(self, 28)
    }
    #[doc = "Bit 29 - Clear bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iocb13(&mut self) -> Iocb13W<ScrSpec> {
        Iocb13W::new(self, 29)
    }
    #[doc = "Bit 30 - Clear bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iocb14(&mut self) -> Iocb14W<ScrSpec> {
        Iocb14W::new(self, 30)
    }
    #[doc = "Bit 31 - Clear bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iocb15(&mut self) -> Iocb15W<ScrSpec> {
        Iocb15W::new(self, 31)
    }
}
#[doc = "Port bit set/clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {
    const RESET_VALUE: u32 = 0;
}
