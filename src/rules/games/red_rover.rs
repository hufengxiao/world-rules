//! 红队过河
//!
//! 两队拉人对抗的儿童团体游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RedRoverRules,
    name: "红队过河",
    desc: "两队拉人对抗的儿童团体游戏",
    origin: "欧美",
    tags: ["儿童", "团体", "户外"]
}

impl RedRoverRules {
    /// 分队站位
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "分成两队手拉手站立",
            "两队相距数米面对面",
            "各队选出队长",
            "约定胜负判定方式",
        ]
    }

    /// 叫阵规则
    pub fn calling(&self) -> Vec<&'static str> {
        vec![
            "一队齐声叫对方名字",
            "被叫者须冲向对方队",
            "尝试冲破对方手链",
            "轮流交换叫阵",
        ]
    }

    /// 突破与拉人
    pub fn breakthrough(&self) -> Vec<&'static str> {
        vec![
            "冲断手链则该队得一人",
            "被拦住则加入对方队",
            "拉人后重新列队",
            "继续下一轮叫阵",
        ]
    }

    /// 胜负判定
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "一方人数被拉空即败",
            "或约定时间人数多者胜",
            "注意安全勿用力过猛",
            "地面平整防摔倒",
        ]
    }
}

impl Rule for RedRoverRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("red_rover")
    }

    fn explain(&self) -> String {
        format!(
            "【红队过河】\n{}",
            [
                format!(
                    "分队站位：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "叫阵规则：\\n{}",
                    self.calling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "突破与拉人：\\n{}",
                    self.breakthrough()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负判定：\\n{}",
                    self.winning()
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
    fn test_redroverrules_basic() {
        let rules = RedRoverRules::new();
        assert_eq!(rules.metadata().name, "红队过河");
        assert!(!rules.setup().is_empty());
        assert!(!rules.calling().is_empty());
        assert!(!rules.breakthrough().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_redroverrules_validation() {
        let rules = RedRoverRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("red_rover"));
    }

    #[test]
    fn test_redroverrules_explain() {
        let rules = RedRoverRules::new();
        let e = rules.explain();
        assert!(e.contains("分队站位"));
        assert!(e.contains("叫阵规则"));
        assert!(e.contains("突破与拉人"));
    }
}
