//! 残疾人皮划艇规则
//!
//! 残疾人皮划艇是残奥会水上项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人皮划艇规则
pub struct ParaCanoeRules {
    metadata: RuleMetadata,
}

impl ParaCanoeRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人皮划艇规则", "残疾人皮划艇比赛规则")
                .with_origin("ICF/IPC")
                .with_tags(vec!["体育".into(), "皮划艇".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "KL1: 严重下肢/躯干残疾",
            "KL2: 中度下肢/躯干残疾",
            "KL3: 轻度下肢残疾",
            "VL1-VL3: 皮艇分级",
            "分级评估: 功能测试",
            "性别分组: 男女分开",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "皮艇KL1-KL3: 200米",
            "划艇VL1-VL3: 200米",
            "残奥会: 9个小项",
            "世界锦标赛",
            "世界杯赛",
            "比赛制: 9航道竞速",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "皮艇: ICF认证",
            "划艇: ICF认证",
            "桨: 适应性设计",
            "座椅: 适配装置",
            "固定带: 允许",
            "平衡装置: 允许",
            "救生衣: 强制",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "起点: 静水出发",
            "航道: 分道竞速",
            "技术: 自由划行",
            "转向: 允许",
            "终点: 电子计时",
            "计时精度: 千分之一秒",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "越出航道",
            "碰撞对手",
            "起点犯规",
            "装备违规",
            "接受非法援助",
            "危险划行",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "座椅适配: 根据分级",
            "固定装置: 允许",
            "平衡辅助: 允许",
            "桨具改装: 允许",
            "救生设备: 强制",
            "起航辅助: 允许",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "ICF分级认证",
            "最低残疾标准",
            "国际皮划艇联注册",
            "游泳能力证明",
            "达标成绩",
        ]
    }
}

impl Default for ParaCanoeRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaCanoeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_canoe")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人皮划艇规则】\n\n\
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
    fn test_para_canoe_rules_basic() {
        let rules = ParaCanoeRules::new();
        assert_eq!(rules.metadata().name, "残疾人皮划艇规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_canoe_classification() {
        let rules = ParaCanoeRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("KL1")));
        assert!(classification.iter().any(|c| c.contains("KL3")));
        assert!(classification.iter().any(|c| c.contains("VL")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_canoe_events() {
        let rules = ParaCanoeRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("200米")));
        assert!(events.iter().any(|e| e.contains("残奥会")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_canoe_equipment() {
        let rules = ParaCanoeRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("皮艇")));
        assert!(equipment.iter().any(|e| e.contains("救生衣")));
        assert!(equipment.len() >= 4);
    }

    #[test]
    fn test_para_canoe_category() {
        let rules = ParaCanoeRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
