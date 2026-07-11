//! 残疾人帆船规则
//!
//! 残疾人帆船是残奥会水上项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人帆船规则
pub struct ParaSailingRules {
    metadata: RuleMetadata,
}

impl ParaSailingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人帆船规则", "残疾人帆船比赛规则")
                .with_origin("IFDS/IPC")
                .with_tags(vec!["体育".into(), "帆船".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "1级: 严重肢体残疾",
            "2级: 中度肢体残疾",
            "3级: 轻度肢体残疾",
            "视觉残疾级: 配备助手",
            "分级评估: 功能测试",
            "性别分组: 混合比赛",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "单人艇: 2.4mR",
            "双人艇: SKUD18",
            "三人艇: Sonar",
            "残奥会: 3个小项",
            "世界锦标赛",
            "洲际锦标赛",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "帆船: IFDS认证船型",
            "座椅: 适配装置",
            "操纵系统: 可改装",
            "固定装置: 允许",
            "自动舵: 允许（视觉残疾）",
            "安全装备: 救生衣强制",
            "通讯设备: 允许",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "航线: 标准绕标",
            "起点: 电子计时",
            "比赛轮次: 多轮积分",
            "计时: 精确到秒",
            "积分制: 低分制",
            "终点: 电子计时",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "碰撞犯规",
            "起点犯规",
            "航线违规",
            "装备违规",
            "超时未到终点",
            "接受非法援助",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "座椅适配: 根据分级",
            "操纵装置改装",
            "自动舵系统",
            "固定装置允许",
            "助手配备（视觉残疾）",
            "声音信号系统",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "IFDS分级认证",
            "最低残疾标准",
            "国际帆船联注册",
            "游泳能力证明",
            "安全培训证书",
        ]
    }
}

impl Default for ParaSailingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaSailingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_sailing")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人帆船规则】\n\n\
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
    fn test_para_sailing_rules_basic() {
        let rules = ParaSailingRules::new();
        assert_eq!(rules.metadata().name, "残疾人帆船规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_sailing_classification() {
        let rules = ParaSailingRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("1级")));
        assert!(classification.iter().any(|c| c.contains("3级")));
        assert!(classification.iter().any(|c| c.contains("残疾")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_sailing_events() {
        let rules = ParaSailingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("单人艇")));
        assert!(events.iter().any(|e| e.contains("残奥会")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_sailing_equipment() {
        let rules = ParaSailingRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("帆船")));
        assert!(equipment.iter().any(|e| e.contains("救生衣")));
        assert!(equipment.len() >= 4);
    }

    #[test]
    fn test_para_sailing_category() {
        let rules = ParaSailingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
