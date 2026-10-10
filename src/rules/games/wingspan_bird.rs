//! 展翅翱翔
//!
//! 收集鸟类卡牌构建生态引擎的策略桌游

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WingspanBirdRules,
    name: "展翅翱翔",
    desc: "收集鸟类卡牌构建生态引擎的策略桌游",
    origin: "美国",
    tags: ["桌游", "策略", "引擎"]
}

impl WingspanBirdRules {
    /// 开局布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "每人一副个人棋盘",
            "分配起始鸟类卡",
            "摆放食物与鸟蛋指示物",
            "发初始奖励卡",
        ]
    }

    /// 回合行动
    pub fn actions(&self) -> Vec<&'static str> {
        vec![
            "每回合选一处栖息地",
            "执行该行行动",
            "打出鸟卡获取食物",
            "产蛋或抽卡",
        ]
    }

    /// 栖息地
    pub fn habitats(&self) -> Vec<&'static str> {
        vec!["森林获取食物", "草原产蛋", "湿地抽卡", "各栖息地能力不同"]
    }

    /// 引擎连锁
    pub fn engine(&self) -> Vec<&'static str> {
        vec![
            "打出的鸟激活整行能力",
            "从右向左依次结算",
            "形成资源循环",
            "构筑高效引擎",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "四轮结束即终局",
            "统计鸟分蛋分与食物",
            "含奖励卡与目标分",
            "总分最高者获胜",
        ]
    }
}

impl Rule for WingspanBirdRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("wingspan_bird")
    }

    fn explain(&self) -> String {
        format!(
            "【展翅翱翔】\n{}",
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
                    "回合行动：\\n{}",
                    self.actions()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "栖息地：\\n{}",
                    self.habitats()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "引擎连锁：\\n{}",
                    self.engine()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负结束：\\n{}",
                    self.ending()
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
    fn test_wingspanbirdrules_basic() {
        let rules = WingspanBirdRules::new();
        assert_eq!(rules.metadata().name, "展翅翱翔");
        assert!(!rules.setup().is_empty());
        assert!(!rules.actions().is_empty());
        assert!(!rules.habitats().is_empty());
        assert!(!rules.engine().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_wingspanbirdrules_validation() {
        let rules = WingspanBirdRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("wingspan_bird"));
    }

    #[test]
    fn test_wingspanbirdrules_explain() {
        let rules = WingspanBirdRules::new();
        let e = rules.explain();
        assert!(e.contains("开局布置"));
        assert!(e.contains("回合行动"));
        assert!(e.contains("栖息地"));
    }
}
