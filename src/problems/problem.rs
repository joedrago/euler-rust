pub trait Problem {
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn test(&self);
    fn answer(&self);
}
