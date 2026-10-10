//! 番摊
//!
//! 中国传统猜数博戏，从一堆物品中按四数递减猜余数

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FanTanRules,
    name: "番摊",
    desc: "中国传统猜数博戏，从一堆物品中按四数递减猜余数",
    origin: "中国",
    tags: ["传统", "猜数", "博戏"]
}

impl FanTanRules {
    /// 开局布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "庄家备一堆棋子或铜钱",
            "以碗或杯盖住此堆",
            "赌客各自下注押数",
            "押一至四的余数",
        ]
    }

    /// 开摊计数
    pub fn counting(&self) -> Vec<&'static str> {
        vec![
            "庄家揭碗公开棋子",
            "以四枚一组依次拨开",
            "数至最后一组余数",
            "余数即开奖结果",
        ]
    }

    /// 押注方式
    pub fn betting(&self) -> Vec<&'static str> {
        vec![
            "可押单一数字",
            "亦可押角或通吃",
            "押中单一赔付一赔三",
            "不同押法赔率各异",
        ]
    }

    /// 结算规则
    pub fn settlement(&self) -> Vec<&'static str> {
        vec![
            "余数为一至四对应开奖",
            "押中者按赔率取彩",
            "未中者注归庄家",
            "庄家按序完成赔付",
        ]
    }

    /// 秩序要求
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "开摊前停止下注",
            "计数过程公开透明",
            "不得中途改注",
            "先后顺序由庄家裁定",
        ]
    }
}

impl Rule for FanTanRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("fan_tan")
    }

    fn explain(&self) -> String {
        format!(
            "【番摊】\n{}",
            [
                format!(
                    "开局布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "开摊计数：\\n{}",
                    self.counting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "押注方式：\\n{}",
                    self.betting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结算规则：\\n{}",
                    self.settlement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "秩序要求：\\n{}",
                    self.order()
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
    fn test_fantanrules_basic() {
        let rules = FanTanRules::new();
        assert_eq!(rules.metadata().name, "番摊");
        assert!(!rules.setup().is_empty());
        assert!(!rules.counting().is_empty());
        assert!(!rules.betting().is_empty());
        assert!(!rules.settlement().is_empty());
        assert!(!rules.order().is_empty());
    }

    #[test]
    fn test_fantanrules_validation() {
        let rules = FanTanRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("fan_tan"));
    }

    #[test]
    fn test_fantanrules_explain() {
        let rules = FanTanRules::new();
        let e = rules.explain();
        assert!(e.contains("开局布置"));
        assert!(e.contains("开摊计数"));
        assert!(e.contains("押注方式"));
    }
}
