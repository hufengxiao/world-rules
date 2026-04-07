//! 中国麻将变体规则模块
//!
//! 包含全国各地流行的麻将玩法规则

pub mod guangdong;
pub mod wuhan;
pub mod shanghai;
pub mod taiwan;
pub mod beijing;
pub mod dongbei;
pub mod changsha;
pub mod hangzhou;
pub mod nanjing;
pub mod chaoshan;
pub mod tianjin;
pub mod chongqing;
pub mod kunming;
pub mod guiyang;
pub mod fuzhou;
pub mod nanchang;
pub mod guangxi;
pub mod xinjiang;
pub mod sichuan;
pub mod zhengzhou;
pub mod xian;
pub mod kejia;
pub mod hainan;
pub mod anhui;
pub mod suzhou;

// 重新导出所有变体规则
pub use guangdong::GuangdongMahjongRules;
pub use wuhan::WuhanMahjongRules;
pub use shanghai::ShanghaiMahjongRules;
pub use taiwan::TaiwanMahjongRules;
pub use beijing::BeijingMahjongRules;
pub use dongbei::DongbeiMahjongRules;
pub use changsha::ChangshaMahjongRules;
pub use hangzhou::HangzhouMahjongRules;
pub use nanjing::NanjingMahjongRules;
pub use chaoshan::ChaoshanMahjongRules;
pub use tianjin::TianjinMahjongRules;
pub use chongqing::ChongqingMahjongRules;
pub use kunming::KunmingMahjongRules;
pub use guiyang::GuiyangMahjongRules;
pub use fuzhou::FuzhouMahjongRules;
pub use nanchang::NanchangMahjongRules;
pub use guangxi::GuangxiMahjongRules;
pub use xinjiang::XinjiangMahjongRules;
pub use sichuan::SichuanDetailedMahjongRules;
pub use zhengzhou::ZhengzhouMahjongRules;
pub use xian::XianMahjongRules;
pub use kejia::KejiaMahjongRules;
pub use hainan::HainanMahjongRules;
pub use anhui::AnhuiMahjongRules;
pub use suzhou::SuzhouMahjongRules;