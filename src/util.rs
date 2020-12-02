pub struct Section {
    _dummy: (),
}

impl Section {
    pub fn start() -> Self {
        dicetest::hints::indent();
        Section { _dummy: () }
    }
}

impl Drop for Section {
    fn drop(&mut self) {
        dicetest::hints::unindent();
    }
}

#[macro_export]
macro_rules! hint_section {
    ($($arg:tt)*) => (
        dicetest::hint!($($arg)*);
        let _section = $crate::util::Section::start();
    )
}
