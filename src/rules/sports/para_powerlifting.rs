//! 残疾人力量举规则
//!
//! 残疾人力量举是残奥会专项项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人力量举规则
pub struct ParaPowerliftingRules {
    metadata: RuleMetadata,
}

impl ParaPowerliftingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人力量举规则", "残疾人力量举比赛规则")
                .with_origin("IPF/IPC")
                .with_tags(vec!["体育".into(), "力量举".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "体重分级: 男子10级",
            "体重分级: 女子10级",
            "最低残疾要求: 肢体残疾",
            "残疾类型: 脊髓损伤、截肢等",
            "分级评估: 功能测试",
            "智力残疾: 无专项分级",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "卧推: 单项比赛",
            "残奥会: 男女20个小项",
            "世界锦标赛: 更多级别",
            "团体赛: 按体重级别",
            "比赛制: 3次试举",
            "重量递增制",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "举重台: 标准规格",
            "杠铃: IPF认证",
            "卧推台: 适应性可选",
            "举重服: IPF认证",
            "护具: 腰带、护腕",
            "假肢: 允许固定",
            "禁止: 增强装置",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "卧推台可改装",
            "假肢固定允许",
            "绑带辅助允许",
            "残肢支撑装置",
            "下肢固定装置",
            "视力残疾声音提示",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "起始信号: 裁判发出",
            "下放: 至胸部",
            "暂停: 裁判信号",
            "推起: 完成动作",
            "结束信号: 裁判发出",
            "动作规范要求",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "起始前推举",
            "未暂停",
            "臀部离开台面",
            "头部移动",
            "杠铃脱落",
            "接受非法援助",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "IPF分级认证",
            "最低残疾标准",
            "IPF注册",
            "体重认证",
            "达标成绩",
        ]
    }
}

impl Default for ParaPowerliftingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaPowerliftingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_powerlifting")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人力量举规则】\n\n\
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
    fn test_para_powerlifting_rules_basic() {
        let rules = ParaPowerliftingRules::new();
        assert_eq!(rules.metadata().name, "残疾人力量举规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_powerlifting_classification() {
        let rules = ParaPowerliftingRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("体重")));
        assert!(classification.iter().any(|c| c.contains("残疾")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_powerlifting_events() {
        let rules = ParaPowerliftingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("卧推")));
        assert!(events.iter().any(|e| e.contains("残奥会")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_powerlifting_technique() {
        let rules = ParaPowerliftingRules::new();
        let technique = rules.technique();
        assert!(technique.iter().any(|t| t.contains("起始")));
        assert!(technique.iter().any(|t| t.contains("推起")));
        assert!(technique.len() >= 4);
    }

    #[test]
    fn test_para_powerlifting_category() {
        let rules = ParaPowerliftingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
