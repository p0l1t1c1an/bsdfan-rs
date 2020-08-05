pub use self::level::Level;

mod level {
    pub struct Level {
        pub num: i32,
        pub min: i32,
        pub max: i32,
    }

    impl Level {
        pub fn new(_num: i32, _min: i32, _max: i32) -> Level {
            Level {
                num: _num,
                min: _min,
                max: _max,
            }
        }
    }
}
