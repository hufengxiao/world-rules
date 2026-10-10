//! 关门骰子游戏
//!
//! 掷骰组合翻下数字板，尽可能关掉全部格子的传统骰桌游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ShutTheBoxRules,
    name: "关门骰子游戏",
    desc: "掷骰组合翻下数字板，尽可能关掉全部格子的传统骰桌游戏",
    origin: "欧洲",
    tags: ["骰子", "数字", "传统"]
}

impl ShutTheBoxRules {
    /// 开局准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "木板标有1至9或1至12的数字",
            "每个数字上方有小翻盖",
            "开局所有翻盖竖立可翻",
            "备两枚骰子一个记分本",
        ]
    }

    /// 掷骰翻盖
    pub fn rolling(&self) -> Vec<&'static str> {
        vec![
            "掷两枚骰子读出点数",
            "可翻下点数之和的盖",
            "或翻下点数相同的两盖",
            "任选合法组合执行",
        ]
    }

    /// 合法组合
    pub fn combinations(&self) -> Vec<&'static str> {
        vec![
            "翻下的数字之和等于骰点",
            "至少翻下一枚翻盖",
            "数字必须尚竖立",
            "无合法组合则本轮结束",
        ]
    }

    /// 单骰规则
    pub fn single_die(&self) -> Vec<&'static str> {
        vec![
            "剩7以下数字时改用单骰",
            "掷一骰翻对应数字",
            "仍可选组合方式",
            "始终无解即停手",
        ]
    }

    /// 计分结束
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "未翻下的数字求和为罚分",
            "全部翻下称关门得零分",
            "轮流进行多局",
            "累计罚分最少者胜",
        ]
    }
}

impl Rule for ShutTheBoxRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("shut_the_box")
    }

    fn explain(&self) -> String {
        format!(
            "【关门骰子游戏】\n{}",
            [
                format!(
                    "开局准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "掷骰翻盖：\\n{}",
                    self.rolling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合法组合：\\n{}",
                    self.combinations()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "单骰规则：\\n{}",
                    self.single_die()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分结束：\\n{}",
                    self.scoring()
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
    fn test_shuttheboxrules_basic() {
        let rules = ShutTheBoxRules::new();
        assert_eq!(rules.metadata().name, "关门骰子游戏");
        assert!(!rules.setup().is_empty());
        assert!(!rules.rolling().is_empty());
        assert!(!rules.combinations().is_empty());
        assert!(!rules.single_die().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_shuttheboxrules_validation() {
        let rules = ShutTheBoxRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("shut_the_box"));
    }

    #[test]
    fn test_shuttheboxrules_explain() {
        let rules = ShutTheBoxRules::new();
        let e = rules.explain();
        assert!(e.contains("开局准备"));
        assert!(e.contains("掷骰翻盖"));
        assert!(e.contains("合法组合"));
    }
}
