#[doc = "Register `STOP` writer"]
pub type W = crate::W<StopSpec>;
impl core::fmt::Debug for crate::generic::Reg<StopSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Переход в режим “Стоп”. Осуществляется записью “1” Отключает тактирования системной шины.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StopSpec;
impl crate::RegisterSpec for StopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`stop::W`](W) writer structure"]
impl crate::Writable for StopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STOP to value 0"]
impl crate::Resettable for StopSpec {}
