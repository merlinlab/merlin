pub mod merlin {
    pub trait Demuxer {

    }

    pub trait Decoder {

    }

    pub trait Encoder {

    }

    pub trait Muxer {

    }

    pub struct Core {

    }

    impl Core {
        fn create_timeline() -> Timeline {

        }
    }

    pub struct Clip {
        filepath: String,
        start_time: i64,
        end_time: i64
    }

    pub struct Timeline {
        clip: Option<Clip>,
    }

    impl Timeline {
        fn duration(&self) -> Option<i64> {
            if let Some(ref c) = self.clip {
                Some(c.end_time - c.start_time)
            } else {
                None
            }
        }
    }

    pub struct Rational {
        num: i32,
        den: i32
    }

    pub struct Frame {
        pts: i64,
        dts: i64,
        width: i32,
        height: i32,
        par: Rational
    }

    pub struct Sample {
        pts: i64,
        dts: i64
    }
}