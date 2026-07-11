//! 残疾人射击规则
//!
//! 残疾人射击包括手枪和步枪项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人射击规则
pub struct ParaShootingRules {
    metadata: RuleMetadata,
}

impl ParaShootingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人射击规则", "残疾人射击比赛规则")
                .with_origin("ISSF/IPC")
                .with_tags(vec!["体育".into(), "射击".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "SH1级: 上肢功能正常（可持枪）",
            "SH2级: 上肢残疾（需支架）",
            "SH1手枪: 手枪项目",
            "SH1步枪: 步枪项目",
            "SH2步枪: 使用支架",
            "视觉残疾: 级别V",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "10米气手枪: 60发",
            "10米气步枪: 60发",
            "50米步枪三姿: 3×20发",
            "50米步枪卧射: 60发",
            "25米手枪: 60发",
            "混合团体赛",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "枪支: ISSF认证",
            "弹药: 标准比赛弹药",
            "射击服: 允许（步枪）",
            "射击鞋: 允许",
            "支架: SH2级必需",
            "轮椅/椅子: 标准",
            "禁止: 电子辅助瞄准",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "SH2级使用射击支架",
            "允许特殊座椅改装",
            "桌子支撑允许",
            "枪具固定装置",
            "视觉辅助设备（V级）",
            "射击时间延长（部分项目）",
        ]
    }

    /// 计分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "靶面: 10环制",
            "每发: 0.1-10.9分",
            "决赛: 淘汰制",
            "最高分: 项目不同",
            "十环: 10.0-10.9分",
            "决赛: 24发（气枪）",
        ]
    }

    /// 安全规则
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "枪口始终指向安全方向",
            "手指远离扳机（未瞄准）",
            "装弹前检查枪支",
            "服从裁判命令",
            "禁止酒后射击",
            "安全区域规定",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "超时射击",
            "装备违规",
            "接受非法指导",
            "违反安全规定",
            "分级违规",
            "干扰他人",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "IPC分级认证",
            "ISSF执照",
            "最低残疾标准",
            "安全认证",
            "达标成绩",
        ]
    }
}

impl Default for ParaShootingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaShootingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_shooting")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人射击规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            安全规则:\n{}",
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
            self.safety()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_shooting_rules_basic() {
        let rules = ParaShootingRules::new();
        assert_eq!(rules.metadata().name, "残疾人射击规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_shooting_classification() {
        let rules = ParaShootingRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("SH1")));
        assert!(classification.iter().any(|c| c.contains("SH2")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_shooting_events() {
        let rules = ParaShootingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("气手枪")));
        assert!(events.iter().any(|e| e.contains("气步枪")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_shooting_safety() {
        let rules = ParaShootingRules::new();
        let safety = rules.safety();
        assert!(safety.iter().any(|s| s.contains("枪口")));
        assert!(safety.len() >= 4);
    }

    #[test]
    fn test_para_shooting_category() {
        let rules = ParaShootingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
