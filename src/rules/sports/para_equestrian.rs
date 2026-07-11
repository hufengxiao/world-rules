//! 残疾人马术规则
//!
//! 残疾人马术比赛主要是盛装舞步项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人马术规则
pub struct ParaEquestrianRules {
    metadata: RuleMetadata,
}

impl ParaEquestrianRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人马术规则", "残疾人马术比赛规则")
                .with_origin("FEI/IPC")
                .with_tags(vec!["体育".into(), "马术".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "Grade I: 严重肢体残疾",
            "Grade II: 中等肢体残疾",
            "Grade III: 轻度肢体残疾",
            "Grade IV: 视力残疾或轻度残疾",
            "Grade V: 最轻度残疾",
            "分级评估: 功能测试",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "个人盛装舞步: 按级别",
            "团体赛: 混合级别",
            "自由样式舞步: 音乐伴奏",
            "锦标赛: 个人/团体",
            "残奥会: 每级别两套动作",
            "马匹选择: 适应性要求",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "马匹: FEI认证",
            "马鞍: 适应性改装",
            "马具: 标准规格",
            "辅助装置: 允许",
            "固定装置: 平衡辅助",
            "马鞭: 允许（长度限制）",
            "禁止: 电子辅助",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "允许适应性马鞍",
            "辅助扶手允许",
            "固定装置: 平衡",
            "视觉辅助: VI级",
            "声音信号允许",
            "教练引导（热身）",
        ]
    }

    /// 评分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "裁判评分: 0-10分",
            "动作评分: 准确性+质量",
            "集体评分: 整体表现",
            "百分比: 最终得分",
            "扣分: 错误",
            "最高: 100%",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "装备违规",
            "马匹虐待",
            "超出时间",
            "接受非法指导",
            "分级违规",
            "马匹失控",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "FEI分级认证",
            "马匹认证",
            "最低残疾标准",
            "FEI注册",
            "达标成绩",
        ]
    }
}

impl Default for ParaEquestrianRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaEquestrianRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_equestrian")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人马术规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            评分规则:\n{}",
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
            self.scoring()
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
    fn test_para_equestrian_rules_basic() {
        let rules = ParaEquestrianRules::new();
        assert_eq!(rules.metadata().name, "残疾人马术规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_equestrian_classification() {
        let rules = ParaEquestrianRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("Grade")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_equestrian_events() {
        let rules = ParaEquestrianRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("盛装舞步")));
        assert!(events.iter().any(|e| e.contains("团体")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_equestrian_scoring() {
        let rules = ParaEquestrianRules::new();
        let scoring = rules.scoring();
        assert!(scoring.iter().any(|s| s.contains("裁判")));
        assert!(scoring.iter().any(|s| s.contains("10分")));
        assert!(scoring.len() >= 4);
    }

    #[test]
    fn test_para_equestrian_category() {
        let rules = ParaEquestrianRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
