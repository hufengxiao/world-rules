//! KTV 唱歌礼仪
//!
//! 在 KTV 或卡拉OK场所的社交礼仪规则，包括点歌顺序、麦克风使用、捧场互动与音量控制，让聚会氛围融洽。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: KaraokeEtiquetteRules,
    name: "KTV 唱歌礼仪",
    desc: "在 KTV 或卡拉OK场所的社交礼仪规则，包括点歌顺序、麦克风使用、捧场互动与音量控制，让聚会氛围融洽。",
    origin: "东亚聚会文化中的常见社交场景",
    tags: ["社交", "聚会", "娱乐", "礼仪"]
}

impl KaraokeEtiquetteRules {
    /// 点歌秩序
    pub fn ordering(&self) -> Vec<&'static str> {
        vec![
            "轮流点歌不霸占歌单",
            "顾及他人喜好搭配曲目",
            "不反复插播同一首歌",
            "给每人留出演唱机会",
        ]
    }

    /// 演唱礼节
    pub fn singing(&self) -> Vec<&'static str> {
        vec![
            "麦克风用完递给下一位",
            "不抢麦不压他人声",
            "音量适中不嘶吼",
            "唱错也不嘲笑起哄",
        ]
    }

    /// 捧场互动
    pub fn support(&self) -> Vec<&'static str> {
        vec![
            "认真听他人演唱",
            "适时鼓掌喝彩",
            "可轻声合唱但不喧宾夺主",
            "不低头玩手机冷场",
        ]
    }

    /// 公共分寸
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "控制音量不扰邻包",
            "爱护设备不乱摔麦克风",
            "饮食注意不弄脏包厢",
            "结束时整理随身物品",
        ]
    }
}

impl Rule for KaraokeEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("karaoke")
    }

    fn explain(&self) -> String {
        format!(
            "【KTV 唱歌礼仪】\n{}",
            [
                format!(
                    "点歌秩序：\\n{}",
                    self.ordering()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "演唱礼节：\\n{}",
                    self.singing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "捧场互动：\\n{}",
                    self.support()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共分寸：\\n{}",
                    self.manner()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_karaokeetiquetterules_basic() {
        let rules = KaraokeEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "KTV 唱歌礼仪");
        assert!(!rules.ordering().is_empty());
        assert!(!rules.singing().is_empty());
        assert!(!rules.support().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_karaokeetiquetterules_validation() {
        let rules = KaraokeEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("karaoke"));
    }

    #[test]
    fn test_karaokeetiquetterules_explain() {
        let rules = KaraokeEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("点歌秩序"));
        assert!(e.contains("演唱礼节"));
        assert!(e.contains("捧场互动"));
    }
}
