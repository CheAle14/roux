pub(crate) mod defaults;
/// Error responses.
pub mod error;
/// Url building.
pub(crate) mod url;
pub use error::RouxError;
/// Options
pub mod option;
pub use option::FeedOption;
pub use option::TimePeriod;

pub(crate) mod ser_map;

macro_rules! maybe_async_handler {
    ($vis:vis fn $fn_name:ident (&$self:ident, $builder:ident, $handler:ident) $err:ty $body:block) => {
        #[cfg(feature = "blocking")]
        maybe_async_handler!(@is_sync $vis fn $fn_name (&$self, $builder, $handler) $err $body);


        #[cfg(not(feature = "blocking"))]
        maybe_async_handler!(@is_async $vis fn $fn_name (&$self, $builder, $handler) $err $body);
    };
    (@is_async $vis:vis fn $fn_name:ident (&$self:ident, $builder:ident, $handler:ident) $err:ty $body:block) => {
        #[maybe_async::maybe_async]
        $vis async fn $fn_name<FReq, FRespFut, FResp, T>(
            &$self,
            $builder: &FReq,
            $handler: &FResp,
        ) -> Result<T, $err>
        where
            FReq: Fn() -> RequestBuilder,
            FRespFut: std::future::Future<Output = reqwest::Result<T>>,
            FResp: Fn(Response) -> FRespFut,

        $body
    };
    (@is_sync $vis:vis fn $fn_name:ident (&$self:ident, $builder:ident, $handler:ident) $err:ty $body:block) => {
        #[maybe_async::maybe_async]
        $vis fn $fn_name<FReq, FResp, T>(
            &$self,
            $builder: &FReq,
            $handler: &FResp,
        ) -> Result<T, $err>
        where
            FReq: Fn() -> RequestBuilder,
            FResp: Fn(Response) -> reqwest::Result<T>,

        $body
    };
}

pub(crate) use maybe_async_handler;
