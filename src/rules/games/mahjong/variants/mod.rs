//! 中国麻将变体规则模块
//!
//! 包含全国各地流行的麻将玩法规则

pub mod anhui;
pub mod beijing;
pub mod changsha;
pub mod chaoshan;
pub mod chongqing;
pub mod dongbei;
pub mod fuzhou;
pub mod guangdong;
pub mod guangxi;
pub mod guiyang;
pub mod hainan;
pub mod hangzhou;
pub mod hebei;
pub mod hunan;
pub mod inner_mongolia;
pub mod kejia;
pub mod kunming;
pub mod nanchang;
pub mod nanjing;
pub mod ningxia;
pub mod shanghai;
pub mod shanxi;
pub mod sichuan;
pub mod suzhou;
pub mod taiwan;
pub mod tianjin;
pub mod wuhan;
pub mod xian;
pub mod xinjiang;
pub mod zhengzhou;

// 重新导出所有变体规则
pub use anhui::AnhuiMahjongRules;
pub use beijing::BeijingMahjongRules;
pub use changsha::ChangshaMahjongRules;
pub use chaoshan::ChaoshanMahjongRules;
pub use chongqing::ChongqingMahjongRules;
pub use dongbei::DongbeiMahjongRules;
pub use fuzhou::FuzhouMahjongRules;
pub use guangdong::GuangdongMahjongRules;
pub use guangxi::GuangxiMahjongRules;
pub use guiyang::GuiyangMahjongRules;
pub use hainan::HainanMahjongRules;
pub use hangzhou::HangzhouMahjongRules;
pub use hebei::HebeiMahjongRules;
pub use hunan::HunanMahjongRules;
pub use inner_mongolia::InnerMongoliaMahjongRules;
pub use kejia::KejiaMahjongRules;
pub use kunming::KunmingMahjongRules;
pub use nanchang::NanchangMahjongRules;
pub use nanjing::NanjingMahjongRules;
pub use ningxia::NingxiaMahjongRules;
pub use shanghai::ShanghaiMahjongRules;
pub use shanxi::ShanxiMahjongRules;
pub use sichuan::SichuanDetailedMahjongRules;
pub use suzhou::SuzhouMahjongRules;
pub use taiwan::TaiwanMahjongRules;
pub use tianjin::TianjinMahjongRules;
pub use wuhan::WuhanMahjongRules;
pub use xian::XianMahjongRules;
pub use xinjiang::XinjiangMahjongRules;
pub use zhengzhou::ZhengzhouMahjongRules;
