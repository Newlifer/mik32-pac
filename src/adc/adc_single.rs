#[doc = "Register `ADC_SINGLE` writer"]
pub type W = crate::W<AdcSingleSpec>;
#[doc = "Field `SINGLE` writer - Запуск однократного измерения"]
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Запуск однократного измерения"]
    #[inline(always)]
    pub fn single(&mut self) -> SingleW<'_, AdcSingleSpec> {
        SingleW::new(self, 0)
    }
}
#[doc = "Управление однократным измерением\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_single::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcSingleSpec;
impl crate::RegisterSpec for AdcSingleSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`adc_single::W`](W) writer structure"]
impl crate::Writable for AdcSingleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_SINGLE to value 0"]
impl crate::Resettable for AdcSingleSpec {}
