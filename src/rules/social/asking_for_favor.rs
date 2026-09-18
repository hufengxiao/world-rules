//! 求人帮忙
//!
//! 开口请求他人帮助时的礼貌与分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AskingForFavorRules,
    name: "求人帮忙",
    desc: "开口请求他人帮助时的礼貌与分寸",
    origin: "中国",
    tags: ["社交", "请求", "礼貌", "人际"]
}

impl AskingForFavorRules {
    /// 开口时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "先看对方是否方便",
            "别在人忙时强求",
            "大小事分量更当",
            "不把帮忙当义务",
        ]
    }

    /// 礼貌表达
    pub fn wording(&self) -> Vec<&'static str> {
        vec!["请字当头", "说明具体请求", "请问您方便吗", "讲清来意与轻重"]
    }

    /// 体谅对方
    pub fn considerate(&self) -> Vec<&'static str> {
        vec![
            "给对方拒绝空间",
            "不强人所难",
            "对方不便要理解",
            "及时转换话题",
        ]
    }

    /// 事后致谢
    pub fn thanks(&self) -> Vec<&'static str> {
        vec!["帮忙后诚心道谢", "记下人情要回礼", "不理所当然", "有来有往"]
    }
}

impl Rule for AskingForFavorRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("ask_favor")
    }

    fn explain(&self) -> String {
        format!(
            "【求人帮忙】\n{}",
            [
                format!(
                    "开口时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼貌表达：\\n{}",
                    self.wording()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体谅对方：\\n{}",
                    self.considerate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "事后致谢：\\n{}",
                    self.thanks()
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
    fn test_askingforfavorrules_basic() {
        let rules = AskingForFavorRules::new();
        assert_eq!(rules.metadata().name, "求人帮忙");
        assert!(!rules.timing().is_empty());
        assert!(!rules.wording().is_empty());
        assert!(!rules.considerate().is_empty());
        assert!(!rules.thanks().is_empty());
    }

    #[test]
    fn test_askingforfavorrules_validation() {
        let rules = AskingForFavorRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("ask_favor"));
    }

    #[test]
    fn test_askingforfavorrules_explain() {
        let rules = AskingForFavorRules::new();
        let e = rules.explain();
        assert!(e.contains("开口时机"));
        assert!(e.contains("礼貌表达"));
        assert!(e.contains("体谅对方"));
    }
}
