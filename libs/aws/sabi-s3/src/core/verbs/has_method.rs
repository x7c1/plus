use reqwest::Method;

pub trait HasMethod<MARKER> {
    const METHOD: Method;
}
