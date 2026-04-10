//! 跨栏规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 跨栏规则
pub struct HurdlesRules {
    metadata: RuleMetadata,
}

impl HurdlesRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "跨栏规则",
                "跨栏比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 项目分类
    pub fn event_types(&self) -> Vec<&'static str> {
        vec![
            "男子110米栏",
            "女子100米栏",
            "男子400米栏",
            "女子400米栏",
            "室内60米栏",
        ]
    }

    /// 栏架高度
    pub fn hurdle_heights(&self) -> Vec<&'static str> {
        vec![
            "男子110米栏: 1.067米",
            "女子100米栏: 0.84米",
            "男子400米栏: 0.914米",
            "女子400米栏: 0.762米",
            "室内60米栏: 1.067米(男)/0.84米(女)",
        ]
    }

    /// 栏架数量
    pub fn hurdle_count(&self) -> Vec<&'static str> {
        vec![
            "110米栏: 10个栏架",
            "100米栏: 10个栏架",
            "400米栏: 10个栏架",
            "60米栏: 5个栏架",
            "均匀分布在跑道",
        ]
    }

    /// 栏间距离
    pub fn hurdle_spacing(&self) -> Vec<&'static str> {
        vec![
            "110米栏: 栏间9.14米",
            "100米栏: 栏间8.5米",
            "400米栏: 栏间35米",
            "起跑至第一栏固定距离",
            "最后一栏至终点固定距离",
        ]
    }

    /// 技术要求
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "跨越栏架不踢倒",
            "保持速度和节奏",
            "栏间步数固定",
            "男子110米栏: 3步",
            "女子100米栏: 3步",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "故意踢倒栏架",
            "跨栏时撞倒多个栏",
            "跑出跑道",
            "阻挡其他选手",
            "违反起跑规则",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "个人赛",
            "分组预赛",
            "决赛",
            "室内室外比赛",
            "接力赛(混合)",
        ]
    }

    /// 计时规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "电子计时",
            "风速测量",
            "反应时间检测",
            "起跑犯规检测",
            "成绩精确到0.01秒",
        ]
    }
}

impl Default for HurdlesRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HurdlesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("hurdles")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【跨栏规则】\n\n\
            项目分类:\n{}\n\n\
            栏架高度:\n{}\n\n\
            技术要求:\n{}\n\n\
            犯规规则:\n{}\n",
            self.event_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.hurdle_heights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hurdles_rules() {
        let rules = HurdlesRules::new();
        assert!(!rules.event_types().is_empty());
    }
}