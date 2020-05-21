
pub const CURRENCY_SYMBOLS_YF: [&'static str; 23] = ["USD", "EUR", "JPY", "GBP", "AUD", "CAD", "BTC", "ETH", "NZD", "SEK", "CHF", "HUF", "CNY", "HKD", "SGD", "INR", "MXN", "PHP", "IDR", "THB", "MYR", "ZAR", "RUB"];

pub const COMMODITIES_SYMBOLS_YF: [&'static str; 23] = [
    "ES", "YM", "NQ", "RTY", "ZB", "ZN", "ZF", "ZT", "GC", "SI", "HG", "PA", "CL", "HO", "NG",
    "RB", "BZ", "C", "KW", "SM", "BO", "S", "CT",
];

pub const REUTERS_COUNTRIES: [&'static str; 17] = [
    "cn", "de", "in", "jp", "uk", "us", "af", "ar", "ara", "br", "ca", "es", "fr", "it", "lta",
    "mx", "ru",
];


pub const YF_HEADER: [&'static str; 7] = ["symb", "t", "o", "h", "l", "c", "v"];

pub const SA_HEADER: [&'static str; 8] = [
    "id",
    "author_id",
    "publish_on",
    "title",
    "slug",
    "ncomments",
    "author_name",
    "path",
];

pub const REUTERS_HEADER: [&'static str; 7] = [
    "id",
    "updated",
    "headline",
    "reason",
    "path",
    "channel_name",
    "channel_path",
];

pub const WSJ_HEADER: [&'static str; 9] = [
    "id",
    "created",
    "name",
    "description",
    "duration",
    "column",
    "doctype",
    "email",
    "thumbnail",
];

pub const NYT_FEED_HEADER: [&'static str; 21] = [
    "slug",
    "first_pub",
    "section",
    "subsec",
    "by",
    "title",
    "subheadline",
    "abs",
    "matrial_type",
    "geo_tag",
    "org_tag",
    "des_tag",
    "per_tag",
    "source",
    "published",
    "created",
    "updated",
    "url",
    "thumbnail",
    "kicker",
    "item_type",
];

pub const NYT_ARCHIVE_HEADER: [&'static str; 12] = [
    "id", "wc", "by", "pub", "doctype", "page", "headline", "kicker", "snippet", "abstract", "url",
    "source",
];


pub const SEC13F_HEADER: [&'static str; 11] = [
    "nameOfIssuer",
    "titleOfClass",
    "cusip",
    "value",
    "sshPrnamt",
    "sshPrnamtType",
    "investmentDiscretion",
    "otherManager",
    "Sole",
    "Shared",
    "None",
];
