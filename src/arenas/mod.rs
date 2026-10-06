#[cfg(not(any(
    feature = "arena-capture-the-flag",
    feature = "arena-spawn-and-swamp",
    feature = "arena-collect-and-control",
    feature = "season4-pain_and_gain",
)))]
compile_error!(
    "select exactly one arena feature, e.g. --no-default-features --features season4-pain_and_gain"
);

#[cfg(any(
    all(
        feature = "season4-pain_and_gain",
        any(
            feature = "arena-capture-the-flag",
            feature = "arena-spawn-and-swamp",
            feature = "arena-collect-and-control",
        ),
    ),
    all(
        feature = "arena-capture-the-flag",
        any(
            feature = "arena-spawn-and-swamp",
            feature = "arena-collect-and-control",
        ),
    ),
    all(
        feature = "arena-spawn-and-swamp",
        feature = "arena-collect-and-control",
    ),
))]
compile_error!("arena features are mutually exclusive; select exactly one");

#[cfg(feature = "arena-capture-the-flag")]
mod capture_the_flag;
#[cfg(feature = "season4-pain_and_gain")]
mod pain_and_gain;

pub(super) const FEATURE: &str = if cfg!(feature = "season4-pain_and_gain") {
    "season4-pain_and_gain"
} else if cfg!(feature = "arena-capture-the-flag") {
    "arena-capture-the-flag"
} else if cfg!(feature = "arena-spawn-and-swamp") {
    "arena-spawn-and-swamp"
} else {
    "arena-collect-and-control"
};

pub(super) fn tick() {
    #[cfg(feature = "arena-capture-the-flag")]
    capture_the_flag::tick();
    #[cfg(feature = "season4-pain_and_gain")]
    pain_and_gain::tick();
}
