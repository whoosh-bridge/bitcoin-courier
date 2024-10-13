use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum NetworkType {
    Mainnet,
    Testnet,
}

#[derive(Copy, Clone)]
pub enum Token {
    BTC,
    ETH,
    USDT,
    BNB,
    SOL,
    USDC,
    XRP,
    DOGE,
    TRX,
    TON,
    ADA,
    AVAX,
    SHIB,
    LINK,
    BCH,
    DOT,
    NEAR,
    SUI,
    LEO,
    DAI,
    LTC,
    TAO,
    APT,
    UNI,
    PEPE,
    ICP,
    FET,
    KAS,
    POL,
    ETC,
    RENDER,
    XLM,
    XMR,
}
