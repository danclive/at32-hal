#[doc = "Register `CLR1` reader"]
pub type R = crate::R<Clr1Spec>;
#[doc = "Register `CLR1` writer"]
pub type W = crate::W<Clr1Spec>;
#[doc = "Field `FERRFC1` reader - Stream 1 clear FIFO error interrupt flag"]
pub type Ferrfc1R = crate::BitReader;
#[doc = "Field `FERRFC1` writer - Stream 1 clear FIFO error interrupt flag"]
pub type Ferrfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC1` reader - Stream 1 clear direct mode error interrupt flag"]
pub type Dmerrfc1R = crate::BitReader;
#[doc = "Field `DMERRFC1` writer - Stream 1 clear direct mode error interrupt flag"]
pub type Dmerrfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC1` reader - Stream 1 clear transfer error interrupt flag"]
pub type Dterrfc1R = crate::BitReader;
#[doc = "Field `DTERRFC1` writer - Stream 1 clear transfer error interrupt flag"]
pub type Dterrfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC1` reader - Stream 1 clear half data transfer interrupt flag"]
pub type Hdtfc1R = crate::BitReader;
#[doc = "Field `HDTFC1` writer - Stream 1 clear half data transfer interrupt flag"]
pub type Hdtfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC1` reader - Stream 1 clear full data transfer complete interrupt flag"]
pub type Fdtfc1R = crate::BitReader;
#[doc = "Field `FDTFC1` writer - Stream 1 clear full data transfer complete interrupt flag"]
pub type Fdtfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRFC2` reader - Stream 2 clear FIFO error interrupt flag"]
pub type Ferrfc2R = crate::BitReader;
#[doc = "Field `FERRFC2` writer - Stream 2 clear FIFO error interrupt flag"]
pub type Ferrfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC2` reader - Stream 2 clear direct mode error interrupt flag"]
pub type Dmerrfc2R = crate::BitReader;
#[doc = "Field `DMERRFC2` writer - Stream 2 clear direct mode error interrupt flag"]
pub type Dmerrfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC2` reader - Stream 2 clear transfer error interrupt flag"]
pub type Dterrfc2R = crate::BitReader;
#[doc = "Field `DTERRFC2` writer - Stream 2 clear transfer error interrupt flag"]
pub type Dterrfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC2` reader - Stream 2 clear half data transfer interrupt flag"]
pub type Hdtfc2R = crate::BitReader;
#[doc = "Field `HDTFC2` writer - Stream 2 clear half data transfer interrupt flag"]
pub type Hdtfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC2` reader - Stream 2 clear full data transfer complete interrupt flag"]
pub type Fdtfc2R = crate::BitReader;
#[doc = "Field `FDTFC2` writer - Stream 2 clear full data transfer complete interrupt flag"]
pub type Fdtfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRFC3` reader - Stream 3 clear FIFO error interrupt flag"]
pub type Ferrfc3R = crate::BitReader;
#[doc = "Field `FERRFC3` writer - Stream 3 clear FIFO error interrupt flag"]
pub type Ferrfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC3` reader - Stream 3 clear direct mode error interrupt flag"]
pub type Dmerrfc3R = crate::BitReader;
#[doc = "Field `DMERRFC3` writer - Stream 3 clear direct mode error interrupt flag"]
pub type Dmerrfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC3` reader - Stream 3 clear transfer error interrupt flag"]
pub type Dterrfc3R = crate::BitReader;
#[doc = "Field `DTERRFC3` writer - Stream 3 clear transfer error interrupt flag"]
pub type Dterrfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC3` reader - Stream 3 clear half data transfer interrupt flag"]
pub type Hdtfc3R = crate::BitReader;
#[doc = "Field `HDTFC3` writer - Stream 3 clear half data transfer interrupt flag"]
pub type Hdtfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC3` reader - Stream 3 clear full data transfer complete interrupt flag"]
pub type Fdtfc3R = crate::BitReader;
#[doc = "Field `FDTFC3` writer - Stream 3 clear full data transfer complete interrupt flag"]
pub type Fdtfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRFC4` reader - Stream 4 clear FIFO error interrupt flag"]
pub type Ferrfc4R = crate::BitReader;
#[doc = "Field `FERRFC4` writer - Stream 4 clear FIFO error interrupt flag"]
pub type Ferrfc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC4` reader - Stream 4 clear direct mode error interrupt flag"]
pub type Dmerrfc4R = crate::BitReader;
#[doc = "Field `DMERRFC4` writer - Stream 4 clear direct mode error interrupt flag"]
pub type Dmerrfc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC4` reader - Stream 4 clear transfer error interrupt flag"]
pub type Dterrfc4R = crate::BitReader;
#[doc = "Field `DTERRFC4` writer - Stream 4 clear transfer error interrupt flag"]
pub type Dterrfc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC4` reader - Stream 4 clear half data transfer interrupt flag"]
pub type Hdtfc4R = crate::BitReader;
#[doc = "Field `HDTFC4` writer - Stream 4 clear half data transfer interrupt flag"]
pub type Hdtfc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC4` reader - Stream 4 clear full data transfer complete interrupt flag"]
pub type Fdtfc4R = crate::BitReader;
#[doc = "Field `FDTFC4` writer - Stream 4 clear full data transfer complete interrupt flag"]
pub type Fdtfc4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stream 1 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc1(&self) -> Ferrfc1R {
        Ferrfc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 1 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc1(&self) -> Dmerrfc1R {
        Dmerrfc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 1 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc1(&self) -> Dterrfc1R {
        Dterrfc1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 1 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc1(&self) -> Hdtfc1R {
        Hdtfc1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 1 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc1(&self) -> Fdtfc1R {
        Fdtfc1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 2 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc2(&self) -> Ferrfc2R {
        Ferrfc2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream 2 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc2(&self) -> Dmerrfc2R {
        Dmerrfc2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream 2 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc2(&self) -> Dterrfc2R {
        Dterrfc2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream 2 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc2(&self) -> Hdtfc2R {
        Hdtfc2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream 2 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc2(&self) -> Fdtfc2R {
        Fdtfc2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream 3 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc3(&self) -> Ferrfc3R {
        Ferrfc3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream 3 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc3(&self) -> Dmerrfc3R {
        Dmerrfc3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream 3 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc3(&self) -> Dterrfc3R {
        Dterrfc3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream 3 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc3(&self) -> Hdtfc3R {
        Hdtfc3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream 3 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc3(&self) -> Fdtfc3R {
        Fdtfc3R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream 4 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc4(&self) -> Ferrfc4R {
        Ferrfc4R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream 4 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc4(&self) -> Dmerrfc4R {
        Dmerrfc4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream 4 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc4(&self) -> Dterrfc4R {
        Dterrfc4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream 4 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc4(&self) -> Hdtfc4R {
        Hdtfc4R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream 4 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc4(&self) -> Fdtfc4R {
        Fdtfc4R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLR1")
            .field("fdtfc4", &self.fdtfc4())
            .field("hdtfc4", &self.hdtfc4())
            .field("dterrfc4", &self.dterrfc4())
            .field("dmerrfc4", &self.dmerrfc4())
            .field("ferrfc4", &self.ferrfc4())
            .field("fdtfc3", &self.fdtfc3())
            .field("hdtfc3", &self.hdtfc3())
            .field("dterrfc3", &self.dterrfc3())
            .field("dmerrfc3", &self.dmerrfc3())
            .field("ferrfc3", &self.ferrfc3())
            .field("fdtfc2", &self.fdtfc2())
            .field("hdtfc2", &self.hdtfc2())
            .field("dterrfc2", &self.dterrfc2())
            .field("dmerrfc2", &self.dmerrfc2())
            .field("ferrfc2", &self.ferrfc2())
            .field("fdtfc1", &self.fdtfc1())
            .field("hdtfc1", &self.hdtfc1())
            .field("dterrfc1", &self.dterrfc1())
            .field("dmerrfc1", &self.dmerrfc1())
            .field("ferrfc1", &self.ferrfc1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stream 1 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc1(&mut self) -> Ferrfc1W<Clr1Spec> {
        Ferrfc1W::new(self, 0)
    }
    #[doc = "Bit 2 - Stream 1 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc1(&mut self) -> Dmerrfc1W<Clr1Spec> {
        Dmerrfc1W::new(self, 2)
    }
    #[doc = "Bit 3 - Stream 1 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc1(&mut self) -> Dterrfc1W<Clr1Spec> {
        Dterrfc1W::new(self, 3)
    }
    #[doc = "Bit 4 - Stream 1 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc1(&mut self) -> Hdtfc1W<Clr1Spec> {
        Hdtfc1W::new(self, 4)
    }
    #[doc = "Bit 5 - Stream 1 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc1(&mut self) -> Fdtfc1W<Clr1Spec> {
        Fdtfc1W::new(self, 5)
    }
    #[doc = "Bit 6 - Stream 2 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc2(&mut self) -> Ferrfc2W<Clr1Spec> {
        Ferrfc2W::new(self, 6)
    }
    #[doc = "Bit 8 - Stream 2 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc2(&mut self) -> Dmerrfc2W<Clr1Spec> {
        Dmerrfc2W::new(self, 8)
    }
    #[doc = "Bit 9 - Stream 2 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc2(&mut self) -> Dterrfc2W<Clr1Spec> {
        Dterrfc2W::new(self, 9)
    }
    #[doc = "Bit 10 - Stream 2 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc2(&mut self) -> Hdtfc2W<Clr1Spec> {
        Hdtfc2W::new(self, 10)
    }
    #[doc = "Bit 11 - Stream 2 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc2(&mut self) -> Fdtfc2W<Clr1Spec> {
        Fdtfc2W::new(self, 11)
    }
    #[doc = "Bit 16 - Stream 3 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc3(&mut self) -> Ferrfc3W<Clr1Spec> {
        Ferrfc3W::new(self, 16)
    }
    #[doc = "Bit 18 - Stream 3 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc3(&mut self) -> Dmerrfc3W<Clr1Spec> {
        Dmerrfc3W::new(self, 18)
    }
    #[doc = "Bit 19 - Stream 3 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc3(&mut self) -> Dterrfc3W<Clr1Spec> {
        Dterrfc3W::new(self, 19)
    }
    #[doc = "Bit 20 - Stream 3 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc3(&mut self) -> Hdtfc3W<Clr1Spec> {
        Hdtfc3W::new(self, 20)
    }
    #[doc = "Bit 21 - Stream 3 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc3(&mut self) -> Fdtfc3W<Clr1Spec> {
        Fdtfc3W::new(self, 21)
    }
    #[doc = "Bit 22 - Stream 4 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc4(&mut self) -> Ferrfc4W<Clr1Spec> {
        Ferrfc4W::new(self, 22)
    }
    #[doc = "Bit 24 - Stream 4 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc4(&mut self) -> Dmerrfc4W<Clr1Spec> {
        Dmerrfc4W::new(self, 24)
    }
    #[doc = "Bit 25 - Stream 4 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc4(&mut self) -> Dterrfc4W<Clr1Spec> {
        Dterrfc4W::new(self, 25)
    }
    #[doc = "Bit 26 - Stream 4 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc4(&mut self) -> Hdtfc4W<Clr1Spec> {
        Hdtfc4W::new(self, 26)
    }
    #[doc = "Bit 27 - Stream 4 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc4(&mut self) -> Fdtfc4W<Clr1Spec> {
        Fdtfc4W::new(self, 27)
    }
}
#[doc = "Interrupt flag clear register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clr1Spec;
impl crate::RegisterSpec for Clr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr1::R`](R) reader structure"]
impl crate::Readable for Clr1Spec {}
#[doc = "`write(|w| ..)` method takes [`clr1::W`](W) writer structure"]
impl crate::Writable for Clr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR1 to value 0"]
impl crate::Resettable for Clr1Spec {
    const RESET_VALUE: u32 = 0;
}
