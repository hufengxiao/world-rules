//! 瘟疫危机
//!
//! 玩家协作对抗全球疫情的合作类桌游

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PandemicCoopRules,
    name: "瘟疫危机",
    desc: "玩家协作对抗全球疫情的合作类桌游",
    origin: "美国",
    tags: ["桌游", "合作", "策略"]
}

impl PandemicCoopRules {
    /// 开局布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "棋盘为世界城市连线",
            "每人扮演不同职业角色",
            "摆放疫情方块与指示物",
            "依难度设定疫情牌堆",
        ]
    }

    /// 回合行动
    pub fn actions(&self) -> Vec<&'static str> {
        vec![
            "每回合可执行四个行动",
            "移动至相邻城市",
            "治疗消除疫情方块",
            "建研究所或交换卡牌",
        ]
    }

    /// 疫情扩散
    pub fn spread(&self) -> Vec<&'static str> {
        vec![
            "抽牌引发城市感染",
            "翻出疫情牌触发连锁",
            "方块堆满即爆发",
            "爆发过多则失败",
        ]
    }

    /// 研发解药
    pub fn cure(&self) -> Vec<&'static str> {
        vec![
            "集齐同色五张城市卡",
            "在研究站研发解药",
            "四种病毒皆须攻克",
            "方可宣告胜利",
        ]
    }

    /// 胜负条件
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "研发出全部解药即胜",
            "方块耗尽或爆发过多则败",
            "牌堆抽尽亦判负",
            "玩家须紧密协作",
        ]
    }
}

impl Rule for PandemicCoopRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("pandemic_coop")
    }

    fn explain(&self) -> String {
        format!(
            "【瘟疫危机】\n{}",
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
                    "疫情扩散：\\n{}",
                    self.spread()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "研发解药：\\n{}",
                    self.cure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负条件：\\n{}",
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
    fn test_pandemiccooprules_basic() {
        let rules = PandemicCoopRules::new();
        assert_eq!(rules.metadata().name, "瘟疫危机");
        assert!(!rules.setup().is_empty());
        assert!(!rules.actions().is_empty());
        assert!(!rules.spread().is_empty());
        assert!(!rules.cure().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_pandemiccooprules_validation() {
        let rules = PandemicCoopRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("pandemic_coop"));
    }

    #[test]
    fn test_pandemiccooprules_explain() {
        let rules = PandemicCoopRules::new();
        let e = rules.explain();
        assert!(e.contains("开局布置"));
        assert!(e.contains("回合行动"));
        assert!(e.contains("疫情扩散"));
    }
}
