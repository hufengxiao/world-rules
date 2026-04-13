//! 彈弓规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 彈弓规则 ( Taiwanese Slingshot)
pub struct TaiwaneseSlingshotRules {
    metadata: RuleMetadata,
}

impl TaiwaneseSlingshotRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "彈弓规则",
                "台湾传统彈弓运动规则"
            )
            .with_origin("台湾")
            .with_tags(vec!["体育".into(), "传统".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "精准射击比赛",
            "距离比赛",
            "速度比赛",
            "团体比赛",
            "表演比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛距离设定",
            "目标大小",
            "射击次数",
            "时间限制",
            "安全规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "握弓技术",
            "拉弹技术",
            "瞄准技术",
            "发射技术",
            "姿势控制",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "彈弓本体",
            "弹丸材料",
            "靶标设置",
            "场地装备",
            "安全装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "射击方向控制",
            "安全区域",
            "护具佩戴",
            "观众安全",
            "场地检查",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "命中靶心得分",
            "距离远近评分",
            "精准度评分",
            "时间成绩",
            "综合评分",
        ]
    }

    /// 级别体系
    pub fn skill_levels(&self) -> Vec<&'static str> {
        vec![
            "初学者级别",
            "中级水平",
            "高级水平",
            "专业级别",
            "大师级别",
        ]
    }
}

impl Default for TaiwaneseSlingshotRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TaiwaneseSlingshotRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("taiwanese_slingshot")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【彈弓规则】\n\n\
            比赛类型:\n{}\n\n\
            技术动作:\n{}\n\n\
            安全规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taiwanese_slingshot_rules() {
        let rules = TaiwaneseSlingshotRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}