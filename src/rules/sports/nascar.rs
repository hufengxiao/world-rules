//! NASCAR赛车规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// NASCAR赛车规则
pub struct NASCARRules {
    metadata: RuleMetadata,
}

impl NASCARRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "NASCAR赛车规则",
                "美国 NASCAR赛车比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "赛车".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            " Cup系列赛",
            " Xfinity系列赛",
            "卡车系列赛",
            "地区系列赛",
            "比赛分类",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "椭圆赛道比赛",
            "圈数规定",
            "出发规则",
            "旗语规则",
            "终点判定",
        ]
    }

    /// 技术规定
    pub fn technical_rules(&self) -> Vec<&'static str> {
        vec![
            "车辆规格规定",
            "发动机规格",
            "车身改装",
            "安全改装",
            "技术检查",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "防护头盔必须",
            "安全服装",
            "安全车规则",
            "红旗规则",
            "医疗支持",
        ]
    }

    /// 积分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "比赛积分",
            "阶段积分",
            "胜利积分",
            "年度积分",
            "积分排名",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            " NASCAR赛车",
            "防护头盔",
            "比赛服装",
            "手套护具",
            "安全装备",
        ]
    }

    /// 犯规规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "危险驾驶",
            "违规超车",
            "技术违规",
            "罚时规则",
            "取消资格",
        ]
    }
}

impl Default for NASCARRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NASCARRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nascar")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【 NASCAR赛车规则】\n\n\
            比赛类型:\n{}\n\n\
            技术规定:\n{}\n\n\
            积分系统:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technical_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nascar_rules() {
        let rules = NASCARRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}