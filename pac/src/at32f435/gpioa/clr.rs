#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `IOCB0` writer - Clear bit 0"]
pub type Iocb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB1` writer - Clear bit 1"]
pub type Iocb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB2` writer - Clear bit 1"]
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
impl core::fmt::Debug for crate::generic::Reg<ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iocb0(&mut self) -> Iocb0W<ClrSpec> {
        Iocb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb1(&mut self) -> Iocb1W<ClrSpec> {
        Iocb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb2(&mut self) -> Iocb2W<ClrSpec> {
        Iocb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iocb3(&mut self) -> Iocb3W<ClrSpec> {
        Iocb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iocb4(&mut self) -> Iocb4W<ClrSpec> {
        Iocb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iocb5(&mut self) -> Iocb5W<ClrSpec> {
        Iocb5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iocb6(&mut self) -> Iocb6W<ClrSpec> {
        Iocb6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iocb7(&mut self) -> Iocb7W<ClrSpec> {
        Iocb7W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iocb8(&mut self) -> Iocb8W<ClrSpec> {
        Iocb8W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iocb9(&mut self) -> Iocb9W<ClrSpec> {
        Iocb9W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iocb10(&mut self) -> Iocb10W<ClrSpec> {
        Iocb10W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iocb11(&mut self) -> Iocb11W<ClrSpec> {
        Iocb11W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iocb12(&mut self) -> Iocb12W<ClrSpec> {
        Iocb12W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iocb13(&mut self) -> Iocb13W<ClrSpec> {
        Iocb13W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iocb14(&mut self) -> Iocb14W<ClrSpec> {
        Iocb14W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iocb15(&mut self) -> Iocb15W<ClrSpec> {
        Iocb15W::new(self, 15)
    }
}
#[doc = "GPIO bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for ClrSpec {
    const RESET_VALUE: u32 = 0;
}
