use serde::Serialize;

pub mod absences;
pub mod announced_tests;
pub mod classes;
pub mod evaluations;
pub mod lessons;
pub mod messages;
pub mod schools;
pub mod token;
pub mod user_info;

// uri
// method
// query
// headers
pub trait Endpoint {
    type Args;

    /// if differs from BASE_URL
    fn base_url(args: impl AsRef<str>) -> String {
        base::school_base(args.as_ref())
        // base::IDP.into()
    }

    /// after BASE_URL
    fn path(args: &Self::Args) -> String;

    fn method() -> http::Method {
        http::Method::GET
    }

    fn query(_input: &Self::Args) -> crate::Res<impl Serialize> {
        Ok(Vec::<String>::new())
    }

    /// Gather the request headers to set.
    fn headers(_input: &impl Serialize) -> crate::Res<Option<http::HeaderMap>> {
        Ok(None)
    }
}

pub mod base {
    pub const IDP: &str = "https://idp.e-kreta.hu";
    pub const ADMIN: &str = "https://eugyintezes.e-kreta.hu";

    pub fn school_base(school_id: impl AsRef<str>) -> String {
        format!("https://{}.e-kreta.hu", school_id.as_ref()).into()
    }
}
