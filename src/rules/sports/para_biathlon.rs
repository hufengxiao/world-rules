//! 残疾人冬季两项规则
//!
//! 残疾人冬季两项结合越野滑雪和射击。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人冬季两项规则
pub struct ParaBiathlonRules {
    metadata: RuleMetadata,
}

impl ParaBiathlonRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人冬季两项规则", "残疾人冬季两项比赛规则")
                .with_origin("IPC/IBU")
                .with_tags(vec![
                    "体育".into(),
                    "滑雪".into(),
                    "射击".into(),
                    "残奥".into(),
                ]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "视力残疾: B1-B3级",
            "站姿: LW2-LW9级",
            "坐姿: LW10-LW12级",
            "分级评估: 功能测试",
            "距离系数: 按分级调整",
            "性别分组: 男女分开",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "短距离: 6-10公里",
            "个人赛: 12.5-15公里",
            "追逐赛: 10-12.5公里",
            "接力赛: 4×2.5公里",
            "残奥会: 18个小项",
            "世锦赛: 更多级别",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑雪板: 适应性设计",
            "坐式滑雪器: 坐姿运动员",
            "滑雪杖: 可改装",
            "步枪: 气步枪或小口径",
            "瞄准装置: 声音辅助（视力残疾）",
            "保护装备: 头盔推荐",
        ]
    }

    /// 射击规则
    pub fn shooting(&self) -> Vec<&'static str> {
        vec![
            "靶标: 10米或50米",
            "靶标大小: 根据分级",
            "姿势: 卧射/坐射",
            "弹药: 5发",
            "罚圈: 150米/罚时1分钟",
            "声音瞄准: 视力残疾专用",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "滑雪: 传统式/自由式",
            "射击间隔: 根据项目",
            "靶标: 5个目标",
            "计时: 精确到0.1秒",
            "罚时: 1分钟/罚圈",
            "比赛轮次: 单轮/追逐",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "未通过检查点",
            "射击姿势违规",
            "弹药超量",
            "装备违规",
            "阻挡对手",
            "接受非法援助",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "视力残疾: 声音瞄准系统",
            "站姿: 义肢适配",
            "坐姿: 坐式滑雪器",
            "距离调整: 按分级",
            "靶标调整: 视力残疾",
            "射击辅助: 允许",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "IPC分级认证",
            "最低残疾标准",
            "IBU注册",
            "射击安全认证",
            "达标成绩",
        ]
    }
}

impl Default for ParaBiathlonRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaBiathlonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_biathlon")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人冬季两项规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            射击规则:\n{}\n\n\
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
            self.shooting()
                .iter()
                .map(|s| format!("  • {}", s))
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
    fn test_para_biathlon_rules_basic() {
        let rules = ParaBiathlonRules::new();
        assert_eq!(rules.metadata().name, "残疾人冬季两项规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_biathlon_classification() {
        let rules = ParaBiathlonRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("视力")));
        assert!(classification.iter().any(|c| c.contains("站姿")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_biathlon_events() {
        let rules = ParaBiathlonRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("短距离")));
        assert!(events.iter().any(|e| e.contains("接力")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_biathlon_shooting() {
        let rules = ParaBiathlonRules::new();
        let shooting = rules.shooting();
        assert!(shooting.iter().any(|s| s.contains("靶标")));
        assert!(shooting.iter().any(|s| s.contains("罚")));
        assert!(shooting.len() >= 4);
    }

    #[test]
    fn test_para_biathlon_category() {
        let rules = ParaBiathlonRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
