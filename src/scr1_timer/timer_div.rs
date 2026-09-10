#[doc = "Register `TIMER_DIV` reader"]
pub type R = crate::R<TimerDivSpec>;
#[doc = "Register `TIMER_DIV` writer"]
pub type W = crate::W<TimerDivSpec>;
#[doc = "Field `DIV` reader - Коэффициент деления частоты. Счет выполняется каждые DIV+1 тактов"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Коэффициент деления частоты. Счет выполняется каждые DIV+1 тактов"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Коэффициент деления частоты. Счет выполняется каждые DIV+1 тактов"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Коэффициент деления частоты. Счет выполняется каждые DIV+1 тактов"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, TimerDivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "Делитель частоты. Счет идет каждые DIV+1 такта частоты\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerDivSpec;
impl crate::RegisterSpec for TimerDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_div::R`](R) reader structure"]
impl crate::Readable for TimerDivSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_div::W`](W) writer structure"]
impl crate::Writable for TimerDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_DIV to value 0"]
impl crate::Resettable for TimerDivSpec {}
