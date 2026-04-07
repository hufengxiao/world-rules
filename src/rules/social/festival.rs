//! 节日礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 中国传统节日
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChineseFestival {
    SpringFestival,    // 春节
    LanternFestival,   // 元宵节
    Qingming,          // 清明节
    DragonBoat,        // 端午节
    MidAutumn,         // 中秋节
    DoubleNinth,       // 重阳节
}

impl ChineseFestival {
    pub fn name(&self) -> &'static str {
        match self {
            ChineseFestival::SpringFestival => "春节",
            ChineseFestival::LanternFestival => "元宵节",
            ChineseFestival::Qingming => "清明节",
            ChineseFestival::DragonBoat => "端午节",
            ChineseFestival::MidAutumn => "中秋节",
            ChineseFestival::DoubleNinth => "重阳节",
        }
    }

    pub fn date(&self) -> &'static str {
        match self {
            ChineseFestival::SpringFestival => "农历正月初一",
            ChineseFestival::LanternFestival => "农历正月十五",
            ChineseFestival::Qingming => "公历4月4-6日",
            ChineseFestival::DragonBoat => "农历五月初五",
            ChineseFestival::MidAutumn => "农历八月十五",
            ChineseFestival::DoubleNinth => "农历九月初九",
        }
    }
}

/// 节日礼仪规则
pub struct FestivalEtiquette {
    metadata: RuleMetadata,
    festival: ChineseFestival,
}

impl FestivalEtiquette {
    pub fn new(festival: ChineseFestival) -> Self {
        Self {
            metadata: RuleMetadata::new(
                format!("{}礼仪", festival.name()),
                "传统节日礼仪规范"
            )
            .with_origin("中国")
            .with_tags(vec!["社交".into(), "节日".into()]),
            festival,
        }
    }

    /// 节日习俗
    pub fn customs(&self) -> Vec<&'static str> {
        match self.festival {
            ChineseFestival::SpringFestival => vec![
                "贴春联、贴福字",
                "放鞭炮、烟花",
                "拜年、发红包",
                "吃年夜饭",
                "穿新衣服",
                "祭祖",
            ],
            ChineseFestival::LanternFestival => vec![
                "赏花灯",
                "吃元宵/汤圆",
                "猜灯谜",
                "舞龙舞狮",
            ],
            ChineseFestival::Qingming => vec![
                "扫墓祭祖",
                "踏青",
                "植树",
                "放风筝",
            ],
            ChineseFestival::DragonBoat => vec![
                "吃粽子",
                "赛龙舟",
                "挂艾草、菖蒲",
                "饮雄黄酒",
            ],
            ChineseFestival::MidAutumn => vec![
                "赏月",
                "吃月饼",
                "团圆饭",
                "赏桂花",
            ],
            ChineseFestival::DoubleNinth => vec![
                "登高",
                "赏菊",
                "插茱萸",
                "敬老",
            ],
        }
    }

    /// 礼仪规范
    pub fn etiquette(&self) -> Vec<&'static str> {
        match self.festival {
            ChineseFestival::SpringFestival => vec![
                "拜年要说吉利话",
                "红包用双数金额",
                "红包不能当面拆",
                "除夕夜全家团圆",
                "初一回娘家是禁忌 (传统)",
            ],
            ChineseFestival::Qingming => vec![
                "扫墓时保持庄重",
                "衣着朴素",
                "清理墓地杂草",
                "献花祭拜",
            ],
            ChineseFestival::MidAutumn => vec![
                "与家人团圆",
                "送月饼礼盒",
                "赏月祈福",
            ],
            _ => vec!["遵守传统习俗"],
        }
    }

    /// 禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        match self.festival {
            ChineseFestival::SpringFestival => vec![
                "初一不扫地 (扫走财气)",
                "不打碎东西",
                "不说脏话、不吉利话",
                "不借钱、不讨债",
            ],
            ChineseFestival::Qingming => vec![
                "不穿鲜艳衣服",
                "不在墓地喧哗",
                "不拍墓地照片",
            ],
            _ => vec![],
        }
    }

    /// 传统食物
    pub fn traditional_food(&self) -> Vec<&'static str> {
        match self.festival {
            ChineseFestival::SpringFestival => vec!["饺子", "年糕", "鱼"],
            ChineseFestival::LanternFestival => vec!["元宵", "汤圆"],
            ChineseFestival::Qingming => vec!["青团"],
            ChineseFestival::DragonBoat => vec!["粽子"],
            ChineseFestival::MidAutumn => vec!["月饼", "桂花糕"],
            ChineseFestival::DoubleNinth => vec!["重阳糕"],
        }
    }
}

impl Rule for FestivalEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("festival")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}礼仪】\n\n\
            时间: {}\n\n\
            传统习俗:\n{}\n\n\
            礼仪规范:\n{}\n\n\
            传统食物:\n{}\n",
            self.festival.name(),
            self.festival.date(),
            self.customs().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.etiquette().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.traditional_food().join("、")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_festival() {
        let festival = FestivalEtiquette::new(ChineseFestival::SpringFestival);
        assert!(festival.customs().contains(&"拜年、发红包"));
    }
}