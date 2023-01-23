use crate::{arc, at, av::audio, define_cls, define_obj_type, objc};

define_obj_type!(TimeEffect(audio::Unit));

impl arc::A<TimeEffect> {
    #[objc::msg_send(initWithAudioComponentDescription:)]
    pub fn init_with_audio_component_description(
        self,
        description: at::audio::ComponentDescription,
    ) -> arc::R<TimeEffect>;
}

/// Unit that processes audio in non real-time
///
/// An TimeEffect represents an audio unit of type `aufc`.
/// These effects do not process audio in real-time. The varispeed
/// unit is an example of a time effect unit.
///
/// AVAudioUnitTimeEffect
impl TimeEffect {
    define_cls!(AV_AUDIO_UNIT_TIME_EFFECT);

    #[objc::msg_send(bypass)]
    pub fn bypass(&self) -> bool;

    #[objc::msg_send(setBypass:)]
    pub fn set_bypass(&mut self, value: bool);

    pub fn with_component_description(
        description: at::audio::ComponentDescription,
    ) -> arc::R<Self> {
        Self::alloc().init_with_audio_component_description(description)
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_UNIT_TIME_EFFECT: &'static objc::Class<TimeEffect>;
}
