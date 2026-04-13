//! 剑道规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 剑道规则
pub struct KendoRules {
    metadata: RuleMetadata,
}

impl KendoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "剑道规则",
                "剑道比赛基本规则"
            )
            .with_origin("日本")
            .with_tags(vec!["体育".into(), "武术".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "个人赛",
            "团体赛",
            "锦标赛",
            "演武赛",
            "级别赛",
        ]
    }

    /// 有效打击部位
    pub fn valid_targets(&self) -> Vec<&'static str> {
        vec![
            "面: 头部正面",
            "小手: 手腕",
            "胴: 腰部",
            "突: 胸喉部",
            "必须配合气势",
        ]
    }

    /// 得分条件
    pub fn scoring_conditions(&self) -> Vec<&'static str> {
        vec![
            "打击部位正确",
            "打击时机正确",
            "气势充沛",
            "姿势正确",
            "残心完整",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 5分钟",
            "先得两本获胜",
            "时间结束一本胜",
            "延长赛规则",
            "判定规则",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "竹刀: 模拟刀剑",
            "面: 头部保护",
            "胴: 腰部保护",
            "小手: 手部保护",
            "垂: 腿部保护",
        ]
    }

    /// 级位制度
    pub fn ranking_system(&self) -> Vec<&'static str> {
        vec![
            "初段至八段",
            "级别考试",
            "演武考核",
            "技术要求",
            "精神修养",
        ]
    }

    /// 场地要求
    pub fn dojo_requirements(&self) -> Vec<&'static str> {
        vec![
            "剑道场铺设",
            "比赛场地9×9米",
            "安全设施",
            "通风良好",
            "祭坛设置",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "不当打击",
            "危险动作",
            "推搡行为",
            "不当言语",
            "装备违规",
        ]
    }
}

impl Default for KendoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KendoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kendo")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【剑道规则】\n\n\
            有效打击部位:\n{}\n\n\
            得分条件:\n{}\n\n\
            装备要求:\n{}\n\n\
            禁止行为:\n{}\n",
            self.valid_targets().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_conditions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kendo_rules() {
        let rules = KendoRules::new();
        assert!(!rules.valid_targets().is_empty());
    }
}