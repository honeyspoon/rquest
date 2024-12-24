#![allow(missing_debug_implementations)]

use super::NetworkScheme;
use crate::HttpVersionPref;
use core::error::Error as StdError;
use http::{header::CONTENT_LENGTH, HeaderMap, HeaderName, HeaderValue, Method, Uri, Version};
use http_body::Body;

pub struct InnerRequest<B>
where
    B: Body + Send + 'static + Unpin,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    request: http::Request<B>,
    http_version_pref: Option<HttpVersionPref>,
    network_scheme: Option<NetworkScheme>,
}

impl<B> InnerRequest<B>
where
    B: Body + Send + 'static + Unpin,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    pub fn builder<'a>() -> InnerRequestBuilder<'a> {
        InnerRequestBuilder::new()
    }

    pub fn split(
        self,
    ) -> (
        http::Request<B>,
        Option<NetworkScheme>,
        Option<HttpVersionPref>,
    ) {
        (self.request, self.network_scheme, self.http_version_pref)
    }
}

/// A builder for constructing HTTP requests.
pub struct InnerRequestBuilder<'a> {
    builder: http::request::Builder,
    http_version_pref: Option<HttpVersionPref>,
    network_scheme: Option<NetworkScheme>,
    headers_order: Option<&'a [HeaderName]>,
}

impl<'a> Default for InnerRequestBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> InnerRequestBuilder<'a> {
    /// Create a new `RequestBuilder` with required fields.
    #[inline]
    pub fn new() -> Self {
        Self {
            builder: hyper2::Request::builder(),
            http_version_pref: None,
            network_scheme: None,
            headers_order: None,
        }
    }

    /// Set the method for the request.
    #[inline]
    pub fn method(mut self, method: Method) -> Self {
        self.builder = self.builder.method(method);
        self
    }

    /// Set the URI for the request.
    #[inline]
    pub fn uri(mut self, uri: Uri) -> Self {
        self.builder = self.builder.uri(uri);
        self
    }

    /// Set the version for the request.
    #[inline]
    pub fn version(mut self, version: impl Into<Option<Version>>) -> Self {
        if let Some(version) = version.into() {
            let pref = match version {
                Version::HTTP_11 | Version::HTTP_10 | Version::HTTP_09 => HttpVersionPref::Http1,
                Version::HTTP_2 => HttpVersionPref::Http2,
                _ => HttpVersionPref::default(),
            };
            self.builder = self.builder.version(version);
            self.http_version_pref = Some(pref);
        }
        self
    }

    /// Set the headers for the request.
    #[inline]
    pub fn headers(mut self, mut headers: HeaderMap) -> Self {
        if let Some(h) = self.builder.headers_mut() {
            std::mem::swap(h, &mut headers);
        }
        self
    }

    /// Set the headers order for the request.
    #[inline]
    pub fn headers_order(mut self, order: Option<&'a [HeaderName]>) -> Self {
        self.headers_order = order;
        self
    }

    /// Set network scheme for the request.
    #[inline]
    pub fn network_scheme(mut self, network_scheme: impl Into<Option<NetworkScheme>>) -> Self {
        self.network_scheme = network_scheme.into();
        self
    }

    /// Set the body for the request.
    #[inline]
    pub fn body<B>(mut self, body: B) -> InnerRequest<B>
    where
        B: Body + Send + 'static + Unpin,
        B::Data: Send,
        B::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        // Sort headers if headers_order is provided
        if let Some(order) = self.headers_order {
            let method = self.builder.method_ref().cloned();
            let headers_mut = self.builder.headers_mut();

            if let (Some(headers), Some(method)) = (headers_mut, method) {
                {
                    // Add CONTENT_LENGTH header if required
                    if let Some(len) = http_body::Body::size_hint(&body).exact() {
                        let needs_content_length = len != 0
                            || !matches!(
                                method,
                                Method::GET | Method::HEAD | Method::DELETE | Method::CONNECT
                            );
                        if needs_content_length {
                            headers
                                .entry(CONTENT_LENGTH)
                                .or_insert_with(|| HeaderValue::from(len));
                        }
                    }
                    // Sort headers
                    crate::util::sort_headers(headers, order);
                }
            }
        }

        InnerRequest {
            request: self.builder.body(body).expect("failed to build request"),
            http_version_pref: self.http_version_pref,
            network_scheme: self.network_scheme,
        }
    }
}
