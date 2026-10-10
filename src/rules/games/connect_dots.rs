//! 点格棋
//!
//! 在格点间画线围成方格的两人纸笔策略游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ConnectDotsRules,
    name: "点格棋",
    desc: "在格点间画线围成方格的两人纸笔策略游戏",
    origin: "法国",
    tags: ["纸笔", "策略", "图形"]
}

impl ConnectDotsRules {
    /// 棋盘准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "纸上画点阵方阵",
            "常用五点见方或更大",
            "相邻点间可连线",
            "线须落在点与点之间",
        ]
    }

    /// 轮流画线
    pub fn drawing(&self) -> Vec<&'static str> {
        vec![
            "每回合连接相邻两点",
            "画过的线不可重复",
            "只能画水平或竖直线",
            "不得画对角线",
        ]
    }

    /// 围格得分
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "补齐方格第四条边即得该格",
            "得格者在格内写下标记",
            "得格后可立即再画一线",
            "连围多格可连续行动",
        ]
    }

    /// 策略要领
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "避免给对手留下第三边",
            "链条式围格可一气呵成",
            "牺牲小格换取主动权",
            "残局须精确计算得失",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "所有方格围满即终局",
            "统计各自占据的方格数",
            "方格多者获胜",
            "同数则平局",
        ]
    }
}

impl Rule for ConnectDotsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("connect_dots")
    }

    fn explain(&self) -> String {
        format!(
            "【点格棋】\n{}",
            [
                format!(
                    "棋盘准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "轮流画线：\\n{}",
                    self.drawing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "围格得分：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "策略要领：\\n{}",
                    self.strategy()
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
    fn test_connectdotsrules_basic() {
        let rules = ConnectDotsRules::new();
        assert_eq!(rules.metadata().name, "点格棋");
        assert!(!rules.setup().is_empty());
        assert!(!rules.drawing().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.strategy().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_connectdotsrules_validation() {
        let rules = ConnectDotsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("connect_dots"));
    }

    #[test]
    fn test_connectdotsrules_explain() {
        let rules = ConnectDotsRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘准备"));
        assert!(e.contains("轮流画线"));
        assert!(e.contains("围格得分"));
    }
}
