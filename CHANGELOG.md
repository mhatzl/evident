# Changelog

## 0.1.0 (2023-09-14)


### ⚠ BREAKING CHANGES

* make setup macros more consistent
* make crate_name on events static str

### Features

* add (un)subscribe to subscription struct ([a8a38a9](https://github.com/mhatzl/evident/commit/a8a38a97af18df2475620d54f86f5651da7bd76c))
* add ci pipeline ([26fe5c7](https://github.com/mhatzl/evident/commit/26fe5c736910428f0d8a0de554357695ba790ea8))
* add derives to CapturedEvent ([d344b62](https://github.com/mhatzl/evident/commit/d344b6233a114ffc6cc8ac69cbe3ee9aa1498922))
* add event filter ([e052054](https://github.com/mhatzl/evident/commit/e0520544f859d8d0936016b2152e2f38f58c716f))
* add get_event_id() to FinalizedEvent ([a354707](https://github.com/mhatzl/evident/commit/a354707e6ed7fb879345a403fade0d39cec23ae9))
* add macro to create static publishers ([f81fc20](https://github.com/mhatzl/evident/commit/f81fc20fd0945e804f76bb560c5b8416702b2569))
* add minimal working pub-sub test ([3c551e6](https://github.com/mhatzl/evident/commit/3c551e640384230021aefc785c1b9d5f87a39365))
* add missed captures counter ([9e754e3](https://github.com/mhatzl/evident/commit/9e754e35fc405268bf4e1ecc6f9279f7a74aca4c))
* add requirement tracing with mantra ([#4](https://github.com/mhatzl/evident/issues/4)) ([ec25303](https://github.com/mhatzl/evident/commit/ec2530317523d65b545581ed57f81be3c5d890d6))
* add shutdown fn for graceful shutdown ([36357fb](https://github.com/mhatzl/evident/commit/36357fb838a2dffe008315c5df13beabbe432814))
* add thread id and creation datetime to events ([ed5e3a1](https://github.com/mhatzl/evident/commit/ed5e3a1415a14e9721673a4329b952cd7a162342))
* allow non-const values for channel bounds ([81ca08a](https://github.com/mhatzl/evident/commit/81ca08a3efee71d88aef102f388985ee86a8715b))
* allow start/stop of capturing during runtime ([407844b](https://github.com/mhatzl/evident/commit/407844b1a63a8bf8abfb9ce997ccfb54dfbfc60b))
* allow to define where event timestamps are set ([8eff566](https://github.com/mhatzl/evident/commit/8eff56628f1f14b1fb21a7a262972b7355f35d75))
* bump crate version ([8e0cc1e](https://github.com/mhatzl/evident/commit/8e0cc1eb6365a8ab3c5213617cb50fb9a1517c69))
* combine event location parameter into origin ([ded9b81](https://github.com/mhatzl/evident/commit/ded9b8154c3889d6085bfc87e16b1d7897c5c5fb))
* impl Display for FinalizedEvent ([a3e199f](https://github.com/mhatzl/evident/commit/a3e199f984f732abf49f059bae3b519584ee216f))
* impl partialEq for Event and EventEntry ([738dc00](https://github.com/mhatzl/evident/commit/738dc00e6c5810eb523bcd203071db70377ef24a))
* implement Hash for EventEntry ([f3f1704](https://github.com/mhatzl/evident/commit/f3f17047368abe1a09f343de38ac4d79d31e4d7e))
* improve documentation for macros ([5a080a0](https://github.com/mhatzl/evident/commit/5a080a08e1a7fa145edf0c0a0cf32511a93a6057))
* make crate_name on events static str ([4fc2d7a](https://github.com/mhatzl/evident/commit/4fc2d7a19d66595adbf6eb307c7e205f84b4923c))
* make Event::new() public ([c678530](https://github.com/mhatzl/evident/commit/c678530af7181d294905a6003c3659927a59425f))
* make Id, Event, and EventEntry generic ([f872d9d](https://github.com/mhatzl/evident/commit/f872d9de33c3d6e05eac6633d4b92403bf02f48c))
* make module_path & filename static str ([eb58bec](https://github.com/mhatzl/evident/commit/eb58bec70ac613074864a7826b16eeb041a456d1))
* make msg type generic ([#3](https://github.com/mhatzl/evident/issues/3)) ([e6615be](https://github.com/mhatzl/evident/commit/e6615be3a811a8641b1c0a536d78db67a9e3e272))
* make setup macros more consistent ([1558b45](https://github.com/mhatzl/evident/commit/1558b45f419837dcbf40283a6105a4f4276cc282))
* move capture time creation into capture thread ([5446795](https://github.com/mhatzl/evident/commit/54467958a30215bcce3d99cb4f0c926e92d1a408))
* offer and handle a stop-capturing ID ([17d3bba](https://github.com/mhatzl/evident/commit/17d3bba04cc6c1e3a02681f14bad44b923a3040a))
* re-export crates used in API ([0821e12](https://github.com/mhatzl/evident/commit/0821e12cc55abcb2ef6edeb22c03dc1c5486dcf9))
* remove crate name from origin ([6ee7a53](https://github.com/mhatzl/evident/commit/6ee7a53ea3b17e42005831f967bd6fc3d1834907))
* remove unused feature from uuid dependency ([9a5e941](https://github.com/mhatzl/evident/commit/9a5e9411ccea8db73e70faf816f56009e96a3ed1))
* return CaptureEvent struct on finalize() ([0935729](https://github.com/mhatzl/evident/commit/0935729fb3ed730d94b0908efb4b26174d241c74))
* switch filtering to entry instead of event ([b9964d2](https://github.com/mhatzl/evident/commit/b9964d26eeef95e56c66132ddc8a61ec55fd89a5))
* use functions instead of trait for set_event ([072a251](https://github.com/mhatzl/evident/commit/072a2519088eec6a845d58ace18559869d763f64))
* use impl Into&lt;K&gt; for (un)subscribe functions ([6375d1a](https://github.com/mhatzl/evident/commit/6375d1aed8095bfcbeb4afbc6c3e887aad160b3d))
* use SystemTime instead of chrono datetime ([0daec4a](https://github.com/mhatzl/evident/commit/0daec4a0237c08e672762310de06b23f31d3463a))


### Bug Fixes

* add impl Eq for EventEntry in creation macro ([236794c](https://github.com/mhatzl/evident/commit/236794c71d1cb262e7693130346a5dd17f5dd4f0))
* clippy warning ([2bad97b](https://github.com/mhatzl/evident/commit/2bad97bdda72f56df87c709dd718099c1163b577))
* clippy warnings ([190223c](https://github.com/mhatzl/evident/commit/190223c4a4dd1219b64368d930dded4743d965f1))
* ensure that stop capturing id is not filtered ([427b3e8](https://github.com/mhatzl/evident/commit/427b3e839ed16fc3659add69c93ad26e0100dd4f))
* guarantee immediate capturing state change ([7c41a65](https://github.com/mhatzl/evident/commit/7c41a657d829acbfcdd0b5291ed4c377abc1cc8c))
* ignore sample for stop_capturing() ([3498f51](https://github.com/mhatzl/evident/commit/3498f5182181596ee8baf615f536eb8ad6af5278))
* improve documentation and add README ([eba0582](https://github.com/mhatzl/evident/commit/eba058268ef8f9fcfbc05b96d0cf5022000cb420))
* make CapturedEvent fields public ([ade7d6f](https://github.com/mhatzl/evident/commit/ade7d6fc31e02bcb88da25503c95fc01ba8c835c))
* move subscription to own module ([2c11434](https://github.com/mhatzl/evident/commit/2c114344d1cb286c027a7c77f2ced98515a341a6))
* put event in Arc for subscriptions ([67fb00d](https://github.com/mhatzl/evident/commit/67fb00d4798beea54df87ec3f9183e0cf2911190))
* reduce needed clones ([151e2b0](https://github.com/mhatzl/evident/commit/151e2b0cb45551719818b9bb8ebc4cfec80102f2))
* remove Arc&lt;RwLock<&gt;> on capture channel ([e15a16e](https://github.com/mhatzl/evident/commit/e15a16eab8ee1475521f1bed342e3ad51dadb9b2))
* remove Drop as mandatory trait for IntermEvents ([9b697e5](https://github.com/mhatzl/evident/commit/9b697e5fa479e61c9f793a9c3d4e6f7879e5d370))
* remove intern into() from into_event_id() ([0c911b5](https://github.com/mhatzl/evident/commit/0c911b593c66f57021d19a4e1eda848c8023f728))
* remove into() for msg in set_event macro ([9988f56](https://github.com/mhatzl/evident/commit/9988f56bacd125a1422b48b03725a32ed1cd2da1))
* remove unused code ([420e6d9](https://github.com/mhatzl/evident/commit/420e6d970d4e1a2118865bcea86e86ed5a99c9d2))
* remove vscode folder from git ([2d8869e](https://github.com/mhatzl/evident/commit/2d8869e89878958e5a419526e875dbca23fb14cb))
* set capture-channel to None on stop-capturing ([287b802](https://github.com/mhatzl/evident/commit/287b8025209696b2972a5c2d122a696180aa3174))
* set prefix for internal macro ([6ae4ef8](https://github.com/mhatzl/evident/commit/6ae4ef81d91c1d80cda0bd64771db6044607d502))
* use `as_deref()` to get thread name ([a5b1424](https://github.com/mhatzl/evident/commit/a5b1424db324bbeaa475b3edec10349c7c956399))
* use better name for set_event with msg ([65f92ab](https://github.com/mhatzl/evident/commit/65f92ab9c84922a0cc2a6d6c62c0dcb4e17fa53a))
* use crate name in set_event macro example ([a581d7d](https://github.com/mhatzl/evident/commit/a581d7de894afc04311b60ce0f0cedd9fbcd9961))
* use try_send() for subs in non-blocking mode ([6e40914](https://github.com/mhatzl/evident/commit/6e40914691f295650b6b6a89af6c5ba16ec03727))
