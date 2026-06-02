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

    /// 动物生态定律
    pub fn ecology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("生态位定律", "生态位", "物种生态位分化规律"),
            ("竞争排斥定律", "竞争排斥", "生态位相同物种不能共存"),
            ("捕食定律", "捕食关系", "捕食者与猎物动态平衡"),
            ("共生定律", "互利共生", "物种间共生关系规律"),
            ("寄生定律", "寄生关系", "寄生物与宿主协同演化"),
            ("食物链定律", "营养级", "能量沿食物链传递"),
            ("种群调节定律", "种群动态", "种群数量调节机制"),
            ("r-K选择定律", "生活史策略", "r策略与K策略权衡"),
        ]
    }

    /// 动物演化定律
    pub fn evolution_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("自然选择定律", "适者生存", "自然选择驱动演化"),
            ("性选择定律", "配偶选择", "性选择塑造性状"),
            ("遗传漂变定律", "随机漂变", "小种群基因频率随机变化"),
            ("基因流动定律", "基因交流", "种群间基因流动效应"),
            ("协同演化定律", "共同演化", "物种间协同演化关系"),
            ("趋同演化定律", "趋同适应", "不同物种独立演化相似特征"),
            ("趋异演化定律", "适应辐射", "共同祖先分化多样物种"),
            ("分子钟定律", "演化速率", "分子序列演化速率恒定"),
        ]
    }

    /// 动物保护定律
    pub fn conservation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("物种保护定律", "濒危物种", "濒危物种保护规律"),
            ("栖息地保护定律", "栖息地", "栖息地保护与恢复"),
            ("生物多样性定律", "多样性维持", "生物多样性维持机制"),
            ("外来物种定律", "入侵物种", "外来物种入侵规律"),
            ("灭绝债务定律", "延迟灭绝", "栖息地丧失后延迟灭绝"),
            ("最小种群定律", "最小存活", "最小可存活种群规模"),
            ("走廊连接定律", "生态走廊", "生态走廊促进基因流"),
            ("野化放归定律", "放归策略", "人工繁殖动物放归规律"),
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
            "【动物学定律】\n\n形态定律:\n{}\n\n行为定律:\n{}\n\n分类定律:\n{}\n\n生态定律:\n{}\n\n演化定律:\n{}\n\n保护定律:\n{}\n",
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
                .join("\n"),
            self.ecology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.evolution_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.conservation_laws().iter()
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