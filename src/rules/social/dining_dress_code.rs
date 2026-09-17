//! 用餐着装礼仪
//!
//! 不同餐饮场合的着装规范与得体装扮礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DiningDressCodeRules,
    name: "用餐着装礼仪",
    desc: "不同餐饮场合的着装规范与得体装扮礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "着装", "用餐"]
}

impl DiningDressCodeRules {
    /// 场合区分
    pub fn occasions(&self) -> Vec<&'static str> {
        vec![
            "日常随便可穿休闲装",
            "正式晚宴需着正式装",
            "商务聚餐着装庄重得体",
            "留意主办方的着装提示",
        ]
    }

    /// 得体准则
    pub fn tidiness(&self) -> Vec<&'static str> {
        vec![
            "衣物整洁无破损",
            "避免过于暴露随意",
            "鞋袜齐整不乱",
            "妆容香水适度",
        ]
    }

    /// 西装细节
    pub fn suit(&self) -> Vec<&'static str> {
        vec![
            "正式场合注意衣领整洁",
            "领带与场合相配",
            "避免用餐时衣物凌乱",
            "室内及时脱下帽子",
        ]
    }

    /// 明智判断
    pub fn judgment(&self) -> Vec<&'static str> {
        vec![
            "不确定场合询问主办",
            "宁可稍正式不随便",
            "餐具摆放体现礼节",
            "吃饭不脱鞋不随便给人添乱",
        ]
    }
}

impl Rule for DiningDressCodeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining_dress")
    }

    fn explain(&self) -> String {
        format!(
            "【用餐着装礼仪】\n{}",
            [
                format!(
                    "场合区分：\\n{}",
                    self.occasions()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "得体准则：\\n{}",
                    self.tidiness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "西装细节：\\n{}",
                    self.suit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "明智判断：\\n{}",
                    self.judgment()
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
    fn test_diningdresscoderules_basic() {
        let rules = DiningDressCodeRules::new();
        assert_eq!(rules.metadata().name, "用餐着装礼仪");
        assert!(!rules.occasions().is_empty());
        assert!(!rules.tidiness().is_empty());
        assert!(!rules.suit().is_empty());
        assert!(!rules.judgment().is_empty());
    }

    #[test]
    fn test_diningdresscoderules_validation() {
        let rules = DiningDressCodeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining_dress"));
    }

    #[test]
    fn test_diningdresscoderules_explain() {
        let rules = DiningDressCodeRules::new();
        let e = rules.explain();
        assert!(e.contains("场合区分"));
        assert!(e.contains("得体准则"));
        assert!(e.contains("西装细节"));
    }
}
