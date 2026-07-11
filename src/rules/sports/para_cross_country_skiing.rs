//! 残疾人越野滑雪规则
//!
//! 残疾人越野滑雪是冬季残奥会的核心项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人越野滑雪规则
pub struct ParaCrossCountrySkiingRules {
    metadata: RuleMetadata,
}

impl ParaCrossCountrySkiingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人越野滑雪规则", "残疾人越野滑雪比赛规则")
                .with_origin("IPC/ISF")
                .with_tags(vec!["体育".into(), "滑雪".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "视力残疾: B1-B3级",
            "站姿: LW2-LW9级",
            "坐姿: LW10-LW12级",
            "分级评估: 功能测试",
            "距离系数: 按分级",
            "性别分组: 男女分开",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "短距离: 1-1.5公里",
            "中距离: 5-10公里",
            "长距离: 15-20公里",
            "接力赛: 4×2.5公里",
            "公开赛: 性别混合",
            "残奥会: 20个小项",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑雪板: 适应性设计",
            "坐式滑雪器: 坐姿运动员",
            "滑雪杖: 可改装",
            "固定器: 专用设计",
            "引导员: 视力残疾必备",
            "保护装备: 头盔推荐",
            "义肢: 允许使用",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "技术: 传统式/自由式",
            "起点: 电子计时",
            "赛道: 标记清晰",
            "通过检查点: 强制",
            "终点: 电子计时",
            "计时精度: 百分之一秒",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "离开赛道",
            "未通过检查点",
            "起点犯规",
            "阻挡对手",
            "装备违规",
            "接受非法援助",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "视力残疾: 引导员系统",
            "站姿: 义肢适配",
            "坐姿: 坐式滑雪器",
            "单杖技术: 上肢残疾",
            "声音提示: 允许",
            "距离调整: 按分级",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "IPC分级认证",
            "最低残疾标准",
            "国际雪联注册",
            "达标成绩",
            "引导员资格认证",
        ]
    }
}

impl Default for ParaCrossCountrySkiingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaCrossCountrySkiingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_cross_country_skiing")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人越野滑雪规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            技术规则:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
                .iter()
                .map(|eq| format!("  • {}", eq))
                .collect::<Vec<_>>()
                .join("\n"),
            self.technique()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_cross_country_skiing_rules_basic() {
        let rules = ParaCrossCountrySkiingRules::new();
        assert_eq!(rules.metadata().name, "残疾人越野滑雪规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_cross_country_skiing_classification() {
        let rules = ParaCrossCountrySkiingRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("视力")));
        assert!(classification.iter().any(|c| c.contains("站姿")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_cross_country_skiing_events() {
        let rules = ParaCrossCountrySkiingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("短距离")));
        assert!(events.iter().any(|e| e.contains("接力")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_cross_country_skiing_technique() {
        let rules = ParaCrossCountrySkiingRules::new();
        let technique = rules.technique();
        assert!(technique
            .iter()
            .any(|t| t.contains("传统式") || t.contains("自由式")));
        assert!(technique.iter().any(|t| t.contains("计时")));
        assert!(technique.len() >= 4);
    }

    #[test]
    fn test_para_cross_country_skiing_category() {
        let rules = ParaCrossCountrySkiingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
