//! 残疾人高山滑雪规则
//!
//! 残疾人高山滑雪是冬季残奥会的重要项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人高山滑雪规则
pub struct ParaAlpineSkiingRules {
    metadata: RuleMetadata,
}

impl ParaAlpineSkiingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人高山滑雪规则", "残疾人高山滑雪比赛规则")
                .with_origin("IPC/ISF")
                .with_tags(vec!["体育".into(), "滑雪".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "视力残疾: B1-B3级",
            "站姿: LW1-LW9级",
            "坐姿: LW10-LW12级",
            "分级评估: 功能测试",
            "装备适配: 根据分级",
            "性别分组: 男女分开",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "滑降: 速度项目",
            "超级大回转: 速度技术结合",
            "大回转: 技术项目",
            "回转: 技术项目",
            "全能: 滑降+回转",
            "残奥会: 30个小项",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑雪板: 适应性设计",
            "坐式滑雪器: 坐姿运动员",
            "固定器: 专用固定",
            "滑雪杖: 可改装",
            "引导员: 视力残疾必备",
            "保护装备: 头盔强制",
            "义肢: 允许使用",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "起点: 电子计时",
            "赛道: 设置旗门",
            "旗门通过: 必须通过",
            "终点: 电子计时",
            "计时精度: 百分之一秒",
            "比赛轮次: 1-2轮",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "漏旗门",
            "未通过旗门",
            "起点犯规",
            "危险滑行",
            "干扰对手",
            "装备违规",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "视力残疾: 引导员系统",
            "站姿: 义肢适配",
            "坐姿: 坐式滑雪器",
            "上肢残疾: 单杖技术",
            "声音提示: 允许",
            "起点辅助: 允许",
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

impl Default for ParaAlpineSkiingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaAlpineSkiingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_alpine_skiing")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人高山滑雪规则】\n\n\
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
    fn test_para_alpine_skiing_rules_basic() {
        let rules = ParaAlpineSkiingRules::new();
        assert_eq!(rules.metadata().name, "残疾人高山滑雪规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_alpine_skiing_classification() {
        let rules = ParaAlpineSkiingRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("视力")));
        assert!(classification.iter().any(|c| c.contains("站姿")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_alpine_skiing_events() {
        let rules = ParaAlpineSkiingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("滑降")));
        assert!(events.iter().any(|e| e.contains("回转")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_alpine_skiing_equipment() {
        let rules = ParaAlpineSkiingRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("滑雪板")));
        assert!(equipment.iter().any(|e| e.contains("头盔")));
        assert!(equipment.len() >= 4);
    }

    #[test]
    fn test_para_alpine_skiing_category() {
        let rules = ParaAlpineSkiingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
