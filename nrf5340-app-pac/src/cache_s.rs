#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Unspecified"]
    pub profiling0: PROFILING,
    _reserved1: [u8; 16usize],
    #[doc = "0x420 - Unspecified"]
    pub profiling1: PROFILING,
    _reserved2: [u8; 208usize],
    #[doc = "0x500 - Enable cache"]
    pub enable: ENABLE,
    #[doc = "0x504 - Invalidate the cache"]
    pub invalidate: INVALIDATE,
    #[doc = "0x508 - Erase the cache"]
    pub erase: ERASE,
    #[doc = "0x50c - Enable the profiling counters"]
    pub profilingenable: PROFILINGENABLE,
    #[doc = "0x510 - Clear the profiling counters"]
    pub profilingclear: PROFILINGCLEAR,
    #[doc = "0x514 - Cache mode. Switching from Cache to RAM mode causes the RAM to be cleared. Switching from RAM to Cache mode causes the cache to be invalidated."]
    pub mode: MODE,
    #[doc = "0x518 - Lock debug mode Ignored in RAM mode."]
    pub debuglock: DEBUGLOCK,
    #[doc = "0x51c - Cache erase status"]
    pub erasestatus: ERASESTATUS,
    #[doc = "0x520 - Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode."]
    pub writelock: WRITELOCK,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PROFILING {
    #[doc = "0x00 - Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    pub ihit: self::profiling::IHIT,
    #[doc = "0x04 - Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    pub imiss: self::profiling::IMISS,
    #[doc = "0x08 - Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    pub dhit: self::profiling::DHIT,
    #[doc = "0x0c - Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    pub dmiss: self::profiling::DMISS,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod profiling;
#[doc = "Enable cache\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable cache"]
pub mod enable;
#[doc = "Invalidate the cache\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [invalidate](invalidate) module"]
pub type INVALIDATE = crate::Reg<u32, _INVALIDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INVALIDATE;
#[doc = "`write(|w| ..)` method takes [invalidate::W](invalidate::W) writer structure"]
impl crate::Writable for INVALIDATE {}
#[doc = "Invalidate the cache"]
pub mod invalidate;
#[doc = "Erase the cache\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [erase](erase) module"]
pub type ERASE = crate::Reg<u32, _ERASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASE;
#[doc = "`write(|w| ..)` method takes [erase::W](erase::W) writer structure"]
impl crate::Writable for ERASE {}
#[doc = "Erase the cache"]
pub mod erase;
#[doc = "Enable the profiling counters\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [profilingenable](profilingenable) module"]
pub type PROFILINGENABLE = crate::Reg<u32, _PROFILINGENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROFILINGENABLE;
#[doc = "`read()` method returns [profilingenable::R](profilingenable::R) reader structure"]
impl crate::Readable for PROFILINGENABLE {}
#[doc = "`write(|w| ..)` method takes [profilingenable::W](profilingenable::W) writer structure"]
impl crate::Writable for PROFILINGENABLE {}
#[doc = "Enable the profiling counters"]
pub mod profilingenable;
#[doc = "Clear the profiling counters\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [profilingclear](profilingclear) module"]
pub type PROFILINGCLEAR = crate::Reg<u32, _PROFILINGCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROFILINGCLEAR;
#[doc = "`write(|w| ..)` method takes [profilingclear::W](profilingclear::W) writer structure"]
impl crate::Writable for PROFILINGCLEAR {}
#[doc = "Clear the profiling counters"]
pub mod profilingclear;
#[doc = "Cache mode. Switching from Cache to RAM mode causes the RAM to be cleared. Switching from RAM to Cache mode causes the cache to be invalidated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Cache mode. Switching from Cache to RAM mode causes the RAM to be cleared. Switching from RAM to Cache mode causes the cache to be invalidated."]
pub mod mode;
#[doc = "Lock debug mode Ignored in RAM mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debuglock](debuglock) module"]
pub type DEBUGLOCK = crate::Reg<u32, _DEBUGLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUGLOCK;
#[doc = "`read()` method returns [debuglock::R](debuglock::R) reader structure"]
impl crate::Readable for DEBUGLOCK {}
#[doc = "`write(|w| ..)` method takes [debuglock::W](debuglock::W) writer structure"]
impl crate::Writable for DEBUGLOCK {}
#[doc = "Lock debug mode Ignored in RAM mode."]
pub mod debuglock;
#[doc = "Cache erase status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [erasestatus](erasestatus) module"]
pub type ERASESTATUS = crate::Reg<u32, _ERASESTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASESTATUS;
#[doc = "`read()` method returns [erasestatus::R](erasestatus::R) reader structure"]
impl crate::Readable for ERASESTATUS {}
#[doc = "`write(|w| ..)` method takes [erasestatus::W](erasestatus::W) writer structure"]
impl crate::Writable for ERASESTATUS {}
#[doc = "Cache erase status"]
pub mod erasestatus;
#[doc = "Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [writelock](writelock) module"]
pub type WRITELOCK = crate::Reg<u32, _WRITELOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRITELOCK;
#[doc = "`read()` method returns [writelock::R](writelock::R) reader structure"]
impl crate::Readable for WRITELOCK {}
#[doc = "`write(|w| ..)` method takes [writelock::W](writelock::W) writer structure"]
impl crate::Writable for WRITELOCK {}
#[doc = "Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode."]
pub mod writelock;
