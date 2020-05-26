
pub const BLOOMBERG_CURRENCY_SYMBOLS: [&'static str; 40] = [
    "USD", "EUR", "XAU", "XAG", "XPT", "XPD", "JPY", "GBP", "AUD", "CAD", "CHF", "KRW", "MXN",
    "BRL", "CLP", "COP", "PEN", "CRC", "ARS", "SEK", "DKK", "NOK", "CZK", "SKK", "PLN", "HUF",
    "RUB", "TRY", "ILS", "KES", "ZAR", "MAD", "NZD", "PHP", "SGD", "IDR", "CNY", "INR", "MYR",
    "THB",
];
// USD,EUR,XAU,XAG,XPT,XPD,JPY,GBP,AUD,CAD,CHF,KRW,MXN,BRL,CLP,COP,PEN,CRC,ARS,SEK,DKK,NOK,CZK,SKK,PLN,HUF,RUB,TRY,ILS,KES,ZAR,MAD,NZD,PHP,SGD,IDR,CNY,INR,MYR,THB,
pub const BLOOMBERG_NEWS_SYMBOLS: [&'static str; 5] = [
    "GOVERNMENT_BOND",
    "COMMODITY",
    "COMMON_STOCK",
    "CURRENCY",
    "BLOOMBERG_BARCLAYS_INDEX",
];

pub const BLOOMBERG_COMMODITIES_SYMBOLS: [&'static str; 37] = [
    "CO1", "CL1", "XB1", "NG1", "HO1", "GC1", "SI1", "HG1", "C%201", "W%201", "CC1", "CT1", "LC1",
    "QS1", "JX1", "MO1", "JG1", "LMCADS03", "LMAHDS03", "LMZSDS03", "LMSNDS03", "O%201", "RR1",
    "S%201", "SM1", "BO1", "RS1", "KC1", "SB1", "JO1", "CT1", "OL1", "LB1", "JN1", "DL1", "FC1",
    "LH1",
];

pub const BLOOMBERG_STOCK_HEADER: [&'static str; 15] = [
    "id",
    "short_name",
    "market_cap",
    "co_phone",
    "last_update",
    "average_volume30_day",
    "price",
    "open_price",
    "high_price",
    "low_price",
    "low_price52_week",
    "high_price52_week",
    "number_of_employees",
    "price_earnings_ratio",
    "shares_outstanding",
];

pub const YF_META_HEADER: [&'static str; 9] = [
"symbol",
"exchange",
"instrument",
"currency",
"first_trade_date",
"reg_mkt_time",
"gmtoffset",
"tz",
"exchange_tz",
];


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

pub const NYT_FEED_HEADER: [&'static str; 16] = [
    "slug",
    "first_pub",
    "section",
    "subsec",
    "by",
    "title",
    "subheadline",
    "abs",
    "matrial_type",
    //"geo_tag",
    //"org_tag",
    //"des_tag",
    //"per_tag",
    "source",
    "published",
    "created",
    "updated",
    "url",
    //"thumbnail",
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

pub const GS_HEADER: [&'static str; 6] = [
"node_id",
"date",
"title",
"description",
"has_video",
"has_audio",
];

pub const GUARDIAN_HEADER: [&'static str; 9] = [
"id",
"type",
"section_id",
"section_name",
"t",
"title",
"url",
"is_hosted",
"pillar_id",
];



