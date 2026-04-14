//! 动物学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 动物学定律集合
pub struct ZoologyLaws {
    metadata: RuleMetadata,
}

impl ZoologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "动物学定律",
                "动物学基本定律"
            )
            .with_origin("生物学")
            .with_tags(vec!["科学".into(), "生物".into(), "动物".into()]),
        }
    }

    /// 动物形态定律
    pub fn morphology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("形态定律", "形态特征", "动物形态多样性"),
            ("适应定律", "形态适应", "形态与环境适应"),
            ("对称定律", "对称类型", "动物对称形式"),
            ("体节定律", "体节结构", "体节分化规律"),
            ("器官定律", "器官系统", "器官系统演化"),
            ("骨骼定律", "骨骼类型", "骨骼结构演化"),
            ("肌肉定律", "肌肉系统", "肌肉结构特征"),
        ]
    }

    /// 动物行为定律
    pub fn behavior_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("本能定律", "本能行为", "先天行为模式"),
            ("学习定律", "学习行为", "后天行为习得"),
            ("社会定律", "社会行为", "动物社会结构"),
            ("交流定律", "信号传递", "动物交流方式"),
            ("觅食定律", "觅食策略", "觅食行为规律"),
            ("繁殖定律", "繁殖行为", "繁殖策略选择"),
            ("迁徙定律", "迁徙规律", "迁徙行为模式"),
            ("领地定律", "领地行为", "领地占有行为"),
        ]
    }

    /// 动物分类定律
    pub fn classification_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("分类定律", "分类系统", "动物分类方法"),
            ("进化定律", "进化关系", "进化系统发育"),
            ("多样性定律", "物种多样", "动物多样性"),
            ("特化定律", "特化适应", "物种特化规律"),
            ("辐射定律", "适应辐射", "物种辐射演化"),
            ("灭绝定律", "物种灭绝", "物种灭绝规律"),
        ]
    }

    /// 动物生理定律
    pub fn physiology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("代谢定律", "新陈代谢", "代谢过程"),
            ("体温定律", "体温调节", "体温调节机制"),
            ("循环定律", "循环系统", "血液循环系统"),
            ("呼吸定律", "呼吸方式", "呼吸系统演化"),
            ("消化定律", "消化系统", "消化系统结构"),
            ("神经定律", "神经系统", "神经系统复杂度"),
            ("内分泌定律", "激素调节", "内分泌系统"),
        ]
    }

    /// 动物类群
    pub fn animal_groups(&self) -> Vec<&'static str> {
        vec![
            "哺乳动物",
            "鸟类",
            "爬行动物",
            "两栖动物",
            "鱼类",
            "昆虫",
            "甲壳动物",
            "软体动物",
        ]
    }

    /// 动物栖息地
    pub fn habitats(&self) -> Vec<&'static str> {
        vec![
            "陆地",
            "海洋",
            "淡水",
            "空中",
            "地下",
            "热带雨林",
            "沙漠",
            "极地",
        ]
    }
}

impl Default for ZoologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ZoologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("zoology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【动物学定律】\n\n形态定律:\n{}\n\n行为定律:\n{}\n\n分类定律:\n{}\n",
            self.morphology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.behavior_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.classification_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zoology_laws() {
        let laws = ZoologyLaws::new();
        assert!(!laws.morphology_laws().is_empty());
        assert!(!laws.behavior_laws().is_empty());
    }
}