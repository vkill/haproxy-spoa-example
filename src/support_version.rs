use semver::Version;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SupportVersion(Version);

impl SupportVersion {
    pub fn new(v: Version) -> Self {
        Self(v)
    }

    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if let Ok(v) = Version::parse(s).or(Version::parse(format!("{}.0", s).as_ref())) {
            Some(Self(v))
        } else {
            None
        }
    }
}

impl ToString for SupportVersion {
    fn to_string(&self) -> String {
        format!("{}.{}", self.0.major, self.0.minor)
    }
}
