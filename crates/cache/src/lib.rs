//! TBD
#![doc(html_favicon_url = "https://salvo.rs/favicon-32x32.png")]
#![doc(html_logo_url = "https://salvo.rs/images/logo.svg")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(private_in_public, unreachable_pub)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

use salvo_core::http::header::{self, HeaderMap, HeaderName, HeaderValue};
use salvo_core::http::headers::{
    AccessControlAllowHeaders, AccessControlAllowMethods, AccessControlExposeHeaders, HeaderMapExt, Origin,
};
use salvo_core::http::{Method, Request, Response, StatusCode};
use salvo_core::{async_trait, Depot, FlowCtrl, Handler};

/// A constructed via `salvo_cache::Cache::builder()`.
#[derive(Clone, Debug)]
pub struct Cache {
}

impl Cache {
    /// Create new `Cache`.
    #[inline]
    pub fn new() -> Self {
        Cache {
        }
    }
}
