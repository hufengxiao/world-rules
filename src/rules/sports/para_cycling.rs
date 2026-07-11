//! 残疾人自行车规则
//!
//! 残疾人自行车比赛是残奥会的重要项目，包括公路赛和场地赛。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人自行车规则
pub struct ParaCyclingRules {
    metadata: RuleMetadata,
}

impl ParaCyclingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人自行车规则", "残疾人自行车比赛规则")
                .with_origin("UCI/IPC")
                .with_tags(vec!["体育".into(), "自行车".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "C级: 截肢/神经损伤（自行车）",
            "C1-C5: 残疾程度递减",
            "H级: 手自行车（脊髓损伤）",
            "H1-H5: 残疾程度递减",
            "T级: 三轮车（平衡障碍）",
            "T1-T2: 残疾程度递减",
            "B级: 视力残疾（双人自行车）",
            "B1-B3: 视力残疾程度递减",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "场地赛: 追逐赛、计时赛、争先赛",
            "公路赛: 个人计时、大组赛",
            "手自行车: 公路赛、计时赛",
            "三轮车: 计时赛、大组赛",
            "双人自行车: 公路赛、计时赛",
            "混合接力: 团体计时赛",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "自行车: 符合UCI规格",
            "手自行车: 卧式或跪式",
            "三轮车: 稳定性改装",
            "双人自行车: 领骑员+运动员",
            "假肢: 可固定在踏板上",
            "头盔: 必须佩戴认证头盔",
            "禁止: 电子辅助设备",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "允许车辆改装适应残疾",
            "假肢固定装置必须安全",
            "视力残疾使用领骑员",
            "出发辅助允许",
            "转换区时间可调整",
            "分组出发安排",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "危险骑行",
            "非法尾随",
            "换道阻挡",
            "假肢脱落影响比赛",
            "接受非法援助",
            "违反分组规定",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "必须通过IPC分级认证",
            "最低残疾标准",
            "UCI执照要求",
            "国家队注册",
            "达标成绩要求",
        ]
    }
}

impl Default for ParaCyclingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaCyclingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_cycling")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人自行车规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            适应性规则:\n{}",
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
            self.adaptations()
                .iter()
                .map(|a| format!("  • {}", a))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_cycling_rules_basic() {
        let rules = ParaCyclingRules::new();
        assert_eq!(rules.metadata().name, "残疾人自行车规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_cycling_classification() {
        let rules = ParaCyclingRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("C级")));
        assert!(classification.iter().any(|c| c.contains("H级")));
        assert!(classification.iter().any(|c| c.contains("B级")));
        assert!(classification.len() >= 6);
    }

    #[test]
    fn test_para_cycling_events() {
        let rules = ParaCyclingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("场地赛")));
        assert!(events.iter().any(|e| e.contains("公路赛")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_cycling_equipment() {
        let rules = ParaCyclingRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("自行车")));
        assert!(equipment.iter().any(|e| e.contains("头盔")));
        assert!(equipment.len() >= 5);
    }

    #[test]
    fn test_para_cycling_category() {
        let rules = ParaCyclingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
