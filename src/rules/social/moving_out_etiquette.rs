//! 搬家告别礼仪
//!
//! 搬迁前后的社交礼仪规则，包括提前告知邻居、妥善交接房屋、与旧邻道别及向新邻问好，保持良好邻里关系。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MovingOutEtiquetteRules,
    name: "搬家告别礼仪",
    desc: "搬迁前后的社交礼仪规则，包括提前告知邻居、妥善交接房屋、与旧邻道别及向新邻问好，保持良好邻里关系。",
    origin: "城市居住流动中的邻里交往场景",
    tags: ["社交", "邻里", "搬迁", "礼仪"]
}

impl MovingOutEtiquetteRules {
    /// 搬前告知
    pub fn notice(&self) -> Vec<&'static str> {
        vec![
            "提前告知房东与邻居",
            "说明搬家时间减少打扰",
            "错开休息时段搬运",
            "妥善处理大件与噪音",
        ]
    }

    /// 房屋交接
    pub fn handover(&self) -> Vec<&'static str> {
        vec![
            "打扫干净再交还",
            "结清水电燃气费用",
            "清点归还钥匙与物品",
            "损坏处如实说明",
        ]
    }

    /// 与旧邻道别
    pub fn farewell(&self) -> Vec<&'static str> {
        vec![
            "向相熟邻居当面告别",
            "留下联系方式保持来往",
            "感谢曾经的照应",
            "可赠小礼聊表心意",
        ]
    }

    /// 与新邻相处
    pub fn greeting(&self) -> Vec<&'static str> {
        vec![
            "搬入后主动打招呼",
            "介绍自己减少陌生感",
            "装修搬迁提前告知",
            "遵守公共秩序不扰邻",
        ]
    }
}

impl Rule for MovingOutEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("moving_out")
    }

    fn explain(&self) -> String {
        format!(
            "【搬家告别礼仪】\n{}",
            [
                format!(
                    "搬前告知：\\n{}",
                    self.notice()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "房屋交接：\\n{}",
                    self.handover()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "与旧邻道别：\\n{}",
                    self.farewell()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "与新邻相处：\\n{}",
                    self.greeting()
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
    fn test_movingoutetiquetterules_basic() {
        let rules = MovingOutEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "搬家告别礼仪");
        assert!(!rules.notice().is_empty());
        assert!(!rules.handover().is_empty());
        assert!(!rules.farewell().is_empty());
        assert!(!rules.greeting().is_empty());
    }

    #[test]
    fn test_movingoutetiquetterules_validation() {
        let rules = MovingOutEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("moving_out"));
    }

    #[test]
    fn test_movingoutetiquetterules_explain() {
        let rules = MovingOutEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("搬前告知"));
        assert!(e.contains("房屋交接"));
        assert!(e.contains("与旧邻道别"));
    }
}
