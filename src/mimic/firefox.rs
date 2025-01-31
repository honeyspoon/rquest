use super::impersonate_imports::*;
use http2::*;
use tls::*;

macro_rules! mod_generator {
    ($mod_name:ident, $tls_settings:expr, $http2_settings:expr, $header_initializer:ident, $ua:tt) => {
        pub(crate) mod $mod_name {
            use super::*;

            #[inline(always)]
            pub fn settings(with_headers: bool) -> ImpersonateSettings {
                ImpersonateSettings::builder()
                    .tls($tls_settings)
                    .http2($http2_settings)
                    .headers(conditional_headers!(with_headers, || {
                        $header_initializer($ua)
                    }))
                    .build()
            }
        }
    };
}

macro_rules! tls_settings {
    (1) => {{
        FirefoxTlsSettings::builder()
            .cert_compression_algorithm(CERT_COMPRESSION_ALGORITHM)
            .enable_ech_grease(true)
            .pre_shared_key(true)
            .psk_skip_session_tickets(true)
            .key_shares_length_limit(3)
            .build()
            .into()
    }};
    (2) => {{
        FirefoxTlsSettings::builder()
            .curves(CURVES_1)
            .key_shares_length_limit(2)
            .build()
            .into()
    }};
    (3) => {{
        FirefoxTlsSettings::builder()
            .cipher_list(CIPHER_LIST_2)
            .curves(CURVES_1)
            .session_ticket(false)
            .enable_ech_grease(true)
            .psk_dhe_ke(false)
            .key_shares_length_limit(2)
            .build()
            .into()
    }};
}

macro_rules! http2_settings {
    (1) => {{
        Http2Settings::builder()
            .initial_stream_id(3)
            .header_table_size(65536)
            .enable_push(false)
            .initial_stream_window_size(131072)
            .max_frame_size(16384)
            .initial_connection_window_size(12517377 + 65535)
            .headers_priority(HEADER_PRIORITY)
            .headers_pseudo_order(HEADERS_PSEUDO_ORDER)
            .settings_order(SETTINGS_ORDER)
            .build()
    }};
    (2) => {{
        Http2Settings::builder()
            .initial_stream_id(15)
            .header_table_size(65536)
            .initial_stream_window_size(131072)
            .max_frame_size(16384)
            .initial_connection_window_size(12517377 + 65535)
            .headers_priority((13, 41, false))
            .headers_pseudo_order(HEADERS_PSEUDO_ORDER)
            .settings_order(SETTINGS_ORDER)
            .priority(Cow::Borrowed(PRIORITY.as_slice()))
            .build()
    }};
    (3) => {{
        Http2Settings::builder()
            .initial_stream_id(3)
            .header_table_size(65536)
            .enable_push(false)
            .max_concurrent_streams(0)
            .initial_stream_window_size(131072)
            .max_frame_size(16384)
            .initial_connection_window_size(12517377 + 65535)
            .headers_priority(HEADER_PRIORITY)
            .headers_pseudo_order(HEADERS_PSEUDO_ORDER)
            .settings_order(SETTINGS_ORDER)
            .build()
    }};
}

#[inline]
fn header_initializer(ua: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_firefox_accept!(headers);
    header_firefox_sec_fetch!(1, headers);
    header_firefox_ua!(headers, ua);
    headers
}

#[inline]
fn header_initializer_with_zstd(ua: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_firefox_accept!(zstd, headers);
    headers.insert(
        HeaderName::from_static("priority"),
        HeaderValue::from_static("u=0, i"),
    );
    header_firefox_sec_fetch!(2, headers);
    header_firefox_ua!(headers, ua);
    headers
}

mod tls {
    use crate::mimic::tls_imports::*;

    pub const CURVES_1: &[SslCurve] = &[
        SslCurve::X25519,
        SslCurve::SECP256R1,
        SslCurve::SECP384R1,
        SslCurve::SECP521R1,
        SslCurve::FFDHE2048,
        SslCurve::FFDHE3072,
    ];

    pub const CURVES_2: &[SslCurve] = &[
        SslCurve::X25519_MLKEM768,
        SslCurve::X25519,
        SslCurve::SECP256R1,
        SslCurve::SECP384R1,
        SslCurve::SECP521R1,
        SslCurve::FFDHE2048,
        SslCurve::FFDHE3072,
    ];

    pub const CIPHER_LIST_1: &str = join!(
        ":",
        "TLS_AES_128_GCM_SHA256",
        "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_AES_256_GCM_SHA384",
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA",
        "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
        "TLS_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_RSA_WITH_AES_128_CBC_SHA",
        "TLS_RSA_WITH_AES_256_CBC_SHA"
    );

    pub const CIPHER_LIST_2: &str = join!(
        ":",
        "TLS_AES_128_GCM_SHA256",
        "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_AES_256_GCM_SHA384",
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
        "TLS_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_RSA_WITH_AES_128_CBC_SHA",
        "TLS_RSA_WITH_AES_256_CBC_SHA"
    );

    pub const SIGALGS_LIST: &str = join!(
        ":",
        "ecdsa_secp256r1_sha256",
        "ecdsa_secp384r1_sha384",
        "ecdsa_secp521r1_sha512",
        "rsa_pss_rsae_sha256",
        "rsa_pss_rsae_sha384",
        "rsa_pss_rsae_sha512",
        "rsa_pkcs1_sha256",
        "rsa_pkcs1_sha384",
        "rsa_pkcs1_sha512",
        "ecdsa_sha1",
        "rsa_pkcs1_sha1"
    );

    pub const CERT_COMPRESSION_ALGORITHM: &[CertCompressionAlgorithm] = &[
        CertCompressionAlgorithm::Zlib,
        CertCompressionAlgorithm::Brotli,
        CertCompressionAlgorithm::Zstd,
    ];

    pub const DELEGATED_CREDENTIALS: &str = join!(
        ":",
        "ecdsa_secp256r1_sha256",
        "ecdsa_secp384r1_sha384",
        "ecdsa_secp521r1_sha512",
        "ecdsa_sha1"
    );

    pub const RECORD_SIZE_LIMIT: u16 = 0x4001;

    pub const EXTENSION_PERMUTATION_INDICES: &[u8] = &{
        const EXTENSIONS: &[ExtensionType] = &[
            ExtensionType::SERVER_NAME,
            ExtensionType::EXTENDED_MASTER_SECRET,
            ExtensionType::RENEGOTIATE,
            ExtensionType::SUPPORTED_GROUPS,
            ExtensionType::EC_POINT_FORMATS,
            ExtensionType::SESSION_TICKET,
            ExtensionType::APPLICATION_LAYER_PROTOCOL_NEGOTIATION,
            ExtensionType::STATUS_REQUEST,
            ExtensionType::DELEGATED_CREDENTIAL,
            ExtensionType::KEY_SHARE,
            ExtensionType::SUPPORTED_VERSIONS,
            ExtensionType::SIGNATURE_ALGORITHMS,
            ExtensionType::PSK_KEY_EXCHANGE_MODES,
            ExtensionType::RECORD_SIZE_LIMIT,
            ExtensionType::CERT_COMPRESSION,
            ExtensionType::ENCRYPTED_CLIENT_HELLO,
        ];

        let mut indices = [0u8; EXTENSIONS.len()];
        let mut index = usize::MIN;
        while index < EXTENSIONS.len() {
            if let Some(idx) = ExtensionType::index_of(EXTENSIONS[index]) {
                indices[index] = idx as u8;
            }
            index += 1;
        }

        indices
    };

    #[derive(TypedBuilder)]
    pub struct FirefoxTlsSettings {
        #[builder(default = CURVES_2)]
        curves: &'static [SslCurve],

        #[builder(default = SIGALGS_LIST)]
        sigalgs_list: &'static str,

        #[builder(default = CIPHER_LIST_1)]
        cipher_list: &'static str,

        #[builder(default = true)]
        session_ticket: bool,

        #[builder(default = false, setter(into))]
        enable_ech_grease: bool,

        #[builder(default = false, setter(into))]
        pre_shared_key: bool,

        #[builder(default = false, setter(into))]
        psk_skip_session_tickets: bool,

        #[builder(default = DELEGATED_CREDENTIALS, setter(into))]
        delegated_credentials: &'static str,

        #[builder(default = RECORD_SIZE_LIMIT, setter(into))]
        record_size_limit: u16,

        #[builder(default, setter(into))]
        key_shares_length_limit: Option<u8>,

        #[builder(default = true, setter(into))]
        psk_dhe_ke: bool,

        #[builder(default, setter(into))]
        cert_compression_algorithm: Option<&'static [CertCompressionAlgorithm]>,

        #[builder(default = EXTENSION_PERMUTATION_INDICES, setter(into))]
        extension_permutation_indices: &'static [u8],
    }

    impl From<FirefoxTlsSettings> for TlsSettings {
        fn from(val: FirefoxTlsSettings) -> Self {
            TlsSettings::builder()
                .curves(Cow::Borrowed(val.curves))
                .sigalgs_list(Cow::Borrowed(val.sigalgs_list))
                .cipher_list(Cow::Borrowed(val.cipher_list))
                .session_ticket(val.session_ticket)
                .delegated_credentials(Cow::Borrowed(val.delegated_credentials))
                .record_size_limit(val.record_size_limit)
                .enable_ocsp_stapling(true)
                .enable_ech_grease(val.enable_ech_grease)
                .alpn_protos(AlpnProtos::All)
                .cert_compression_algorithm(val.cert_compression_algorithm.map(Cow::Borrowed))
                .min_tls_version(TlsVersion::TLS_1_2)
                .max_tls_version(TlsVersion::TLS_1_3)
                .key_shares_length_limit(val.key_shares_length_limit)
                .pre_shared_key(val.pre_shared_key)
                .psk_skip_session_ticket(val.psk_skip_session_tickets)
                .psk_dhe_ke(val.psk_dhe_ke)
                .extension_permutation_indices(Cow::Borrowed(val.extension_permutation_indices))
                .build()
        }
    }
}

mod http2 {
    use crate::mimic::http2_imports::*;

    pub const HEADER_PRIORITY: (u32, u8, bool) = (0, 41, false);

    pub const HEADERS_PSEUDO_ORDER: [PseudoOrder; 4] = [Method, Path, Authority, Scheme];

    pub const SETTINGS_ORDER: [SettingsOrder; 8] = [
        HeaderTableSize,
        EnablePush,
        MaxConcurrentStreams,
        InitialWindowSize,
        MaxFrameSize,
        MaxHeaderListSize,
        UnknownSetting8,
        UnknownSetting9,
    ];

    pub static PRIORITY: LazyLock<[Priority; 6]> = LazyLock::new(|| {
        [
            Priority::new(
                StreamId::from(3),
                StreamDependency::new(StreamId::zero(), 200, false),
            ),
            Priority::new(
                StreamId::from(5),
                StreamDependency::new(StreamId::zero(), 100, false),
            ),
            Priority::new(
                StreamId::from(7),
                StreamDependency::new(StreamId::zero(), 0, false),
            ),
            Priority::new(
                StreamId::from(9),
                StreamDependency::new(StreamId::from(7), 0, false),
            ),
            Priority::new(
                StreamId::from(11),
                StreamDependency::new(StreamId::from(3), 0, false),
            ),
            Priority::new(
                StreamId::from(13),
                StreamDependency::new(StreamId::zero(), 240, false),
            ),
        ]
    });
}

mod_generator!(
    ff109,
    tls_settings!(2),
    http2_settings!(2),
    header_initializer,
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/109.0"
);

mod_generator!(
    ff117,
    tls_settings!(2),
    http2_settings!(2),
    header_initializer,
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/117.0"
);

mod_generator!(
    ff128,
    tls_settings!(3),
    http2_settings!(3),
    header_initializer_with_zstd,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:128.0) Gecko/20100101 Firefox/128.0"
);

mod_generator!(
    ff133,
    tls_settings!(1),
    http2_settings!(1),
    header_initializer_with_zstd,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:133.0) Gecko/20100101 Firefox/133.0"
);
