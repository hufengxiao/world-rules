//! 铁人三项详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 铁人三项详细规则
pub struct TriathlonDetailedRules {
    metadata: RuleMetadata,
}

impl TriathlonDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "铁人三项详细规则",
                "铁人三项比赛详细规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "综合".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "标准距离: 51.5公里",
            "半程铁人三项",
            "全程铁人三项: 226公里",
            "超长距离",
            "短距离",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "游泳: 1.5公里/3.8公里",
            "自行车: 40公里/180公里",
            "跑步: 10公里/42公里",
            "转换区",
            "项目顺序",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "项目顺序固定",
            "转换区规则",
            "时间限制",
            "出发规则",
            "终点判定",
        ]
    }

    /// 游泳规则
    pub fn swimming_rules(&self) -> Vec<&'static str> {
        vec![
            "自由泳为主",
            "允许站立休息",
            "辅助装备规定",
            "游泳路线",
            "安全规则",
        ]
    }

    /// 自行车规则
    pub fn cycling_rules(&self) -> Vec<&'static str> {
        vec![
            "公路自行车",
            "禁止阻挡",
            "跟骑规则",
            "补给规定",
            "自行车检查",
        ]
    }

    /// 跑步规则
    pub fn running_rules(&self) -> Vec<&'static str> {
        vec![
            "公路跑步",
            "跑步路线",
            "补给站",
            "步行允许",
            "终点冲刺",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "游泳装备",
            "自行车装备",
            "跑步装备",
            "转换区装备",
            "比赛服装",
        ]
    }
}

impl Default for TriathlonDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TriathlonDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("triathlon_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【铁人三项详细规则】\n\n\
            比赛类型:\n{}\n\n\
            比赛项目:\n{}\n\n\
            游泳规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.swimming_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triathlon_detailed_rules() {
        let rules = TriathlonDetailedRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}