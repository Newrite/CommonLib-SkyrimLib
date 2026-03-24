/// C++ `RE::EmotionType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmotionType {
    Neutral = 0,
    Anger = 1,
    Disgust = 2,
    Fear = 3,
    Sad = 4,
    Happy = 5,
    Surprise = 6,
    Puzzled = 7,
}

core_util::impl_enumset_type!(EmotionType => u32);
