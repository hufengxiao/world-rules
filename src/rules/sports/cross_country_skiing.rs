//! 越野滑雪规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 越野滑雪规则
pub struct CrossCountrySkiingRules {
    metadata: RuleMetadata,
}

impl CrossCountrySkiingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "越野滑雪规则",
                "越野滑雪比赛基本规则"
            )
            .with_origin("挪威")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "传统技术比赛",
            "自由技术比赛",
            "追逐赛",
            "接力赛",
            "集体出发赛",
        ]
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "短距离: 1.5公里",
            "中距离: 10公里",
            "长距离: 15-30公里",
            "接力: 4×10公里(男子)",
            "接力: 4×5公里(女子)",
        ]
    }

    /// 传统技术规则
    pub fn classic_technique(&self) -> Vec<&'static str> {
        vec![
            "雪板必须有纹理",
            "滑行区限制",
            "禁止使用滑行技术",
            "直线滑行",
            "双板交替滑行",
        ]
    }

    /// 自由技术规则
    pub fn free_technique(&self) -> Vec<&'static str> {
        vec![
            "允许任何滑行技术",
            "滑冰式滑行",
            "自由滑行区",
            "速度更快",
            "技术多样",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "越野滑雪板",
            "滑雪靴: 轻便",
            "固定器: 脚跟自由",
            "滑雪杖: 长度合适",
            "传统技术板有纹理",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "间隔出发",
            "追逐赛出发时间差",
            "集体出发同组出发",
            "接力交接规则",
            "禁止阻挡他人",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "传统技术比赛使用滑行技术",
            "阻挡他人",
            "违规补给",
            "超出赛道",
            "违规交接",
        ]
    }

    /// 计时规则
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "电子计时",
            "精确到秒",
            "分段计时",
            "总时间计算",
            "成绩公布",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "赛道安全检查",
            "天气条件监控",
            "医疗支持",
            "紧急撤离",
            "补给站设置",
        ]
    }
}

impl Default for CrossCountrySkiingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CrossCountrySkiingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("cross_country_skiing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【越野滑雪规则】\n\n\
            比赛形式:\n{}\n\n\
            传统技术规则:\n{}\n\n\
            自由技术规则:\n{}\n\n\
            犯规规则:\n{}\n",
            self.competition_formats().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.classic_technique().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.free_technique().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_country_skiing_rules() {
        let rules = CrossCountrySkiingRules::new();
        assert!(!rules.competition_formats().is_empty());
    }
}