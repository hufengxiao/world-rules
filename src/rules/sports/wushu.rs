//! 武术规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 武术规则
pub struct WushuRules {
    metadata: RuleMetadata,
}

impl WushuRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "武术规则",
                "武术比赛基本规则"
            )
            .with_origin("中国")
            .with_tags(vec!["体育".into(), "传统".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_categories(&self) -> Vec<&'static str> {
        vec![
            "套路比赛: 套路表演",
            "散打比赛: 实战比赛",
            "太极比赛: 太极套路",
            "传统武术: 传统流派",
            "集体项目: 团体表演",
        ]
    }

    /// 套路类型
    pub fn routine_types(&self) -> Vec<&'static str> {
        vec![
            "长拳: 北派武术",
            "南拳: 南派武术",
            "太极拳: 内家拳",
            "刀术: 刀法套路",
            "剑术: 剑法套路",
            "棍术: 棍法套路",
            "枪术: 枪法套路",
        ]
    }

    /// 比赛场地
    pub fn competition_area(&self) -> Vec<&'static str> {
        vec![
            "套路场地: 14米×8米",
            "散打擂台: 8米×8米",
            "场地平整",
            "安全垫铺设",
            "比赛区域标记",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "动作规范: 5分",
            "演练水平: 3分",
            "难度动作: 2分",
            "创新加分",
            "扣分项目",
        ]
    }

    /// 时间要求
    pub fn time_requirements(&self) -> Vec<&'static str> {
        vec![
            "长拳套路: 1分20秒以上",
            "太极拳套路: 5-6分钟",
            "散打比赛: 3回合每回合2分钟",
            "超时扣分",
            "时间精确测量",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "武术服装",
            "武术器械",
            "护具: 散打比赛",
            "比赛专用器材",
            "安全检查",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "危险动作",
            "违规器械使用",
            "不当行为",
            "干扰比赛",
            "违规服装",
        ]
    }

    /// 评判组成
    pub fn judging_composition(&self) -> Vec<&'static str> {
        vec![
            "主裁判: 比赛指挥",
            "评分裁判: 5人评分",
            "计时员",
            "记录员",
            "仲裁委员会",
        ]
    }
}

impl Default for WushuRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WushuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wushu")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【武术规则】\n\n\
            比赛项目:\n{}\n\n\
            套路类型:\n{}\n\n\
            评分标准:\n{}\n\n\
            禁止行为:\n{}\n",
            self.competition_categories().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.routine_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wushu_rules() {
        let rules = WushuRules::new();
        assert!(!rules.competition_categories().is_empty());
    }
}