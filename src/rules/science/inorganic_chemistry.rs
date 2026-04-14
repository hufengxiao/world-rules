//! 无机化学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 无机化学定律集合
pub struct InorganicChemistryLaws {
    metadata: RuleMetadata,
}

impl InorganicChemistryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "无机化学定律",
                "无机化学基本定律"
            )
            .with_origin("化学")
            .with_tags(vec!["科学".into(), "化学".into(), "无机".into()]),
        }
    }

    /// 周期表定律
    pub fn periodic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("周期定律", "元素周期性", "元素性质周期性变化"),
            ("原子半径定律", "周期变化", "原子半径周期变化"),
            ("电负性定律", "鲍林电负性", "元素吸引电子能力"),
            ("电离能定律", "周期变化", "电离能周期变化"),
            ("电子亲和能定律", "电子获得", "原子获得电子能量"),
            ("氧化态定律", "氧化数", "元素常见氧化态"),
            ("金属非金属定律", "分区规律", "周期表金属非金属分布"),
            ("元素族定律", "同族相似", "同族元素性质相似"),
        ]
    }

    /// 化学键定律
    pub fn bonding_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("离子键定律", "电子转移", "电子完全转移"),
            ("共价键定律", "电子共享", "电子共享"),
            ("金属键定律", "电子海", "金属电子海模型"),
            ("氢键定律", "特殊相互作用", "氢与电负性元素作用"),
            ("范德华力定律", "分子间力", "分子间弱相互作用"),
            ("配位键定律", "中心原子配体", "配位化合物形成"),
            ("键能定律", "键强度", "化学键强度度量"),
            ("键长定律", "键距离", "化学键长度"),
        ]
    }

    /// 晶体定律
    pub fn crystal_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("晶体结构定律", "周期排列", "离子晶体周期排列"),
            ("离子晶体定律", "静电引力", "离子晶体静电引力"),
            ("配位数定律", "周围离子数", "离子周围配位离子数"),
            ("晶格能定律", "形成能量", "离子晶体形成能"),
            ("晶胞定律", "最小单元", "晶体最小重复单元"),
            ("晶格定律", "布拉维格子", "14种布拉维格子"),
            ("晶体缺陷定律", "缺陷存在", "晶体中存在缺陷"),
            ("同晶定律", "结构相同", "不同化合物结构相同"),
        ]
    }

    /// 配位化学定律
    pub fn coordination_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("配位数定律", "配体数量", "中心原子配体数量"),
            ("配位几何定律", "空间构型", "配位化合物几何形状"),
            ("螯合定律", "多齿配体", "螯合效应稳定性"),
            ("配位场定律", "d轨道分裂", "配位场理论"),
            ("晶体场定律", "能量分裂", "晶体场理论"),
            ("18电子定律", "稳定规则", "过渡金属稳定规则"),
            ("配位稳定性定律", "稳定常数", "配位化合物稳定性"),
        ]
    }

    /// 无机反应定律
    pub fn reaction_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("氧化还原定律", "电子转移", "无机氧化还原反应"),
            ("沉淀定律", "溶解度", "沉淀反应条件"),
            ("中和定律", "酸碱反应", "酸碱中和反应"),
            ("置换定律", "元素置换", "活泼元素置换"),
            ("复分解定律", "离子交换", "离子交换反应"),
            ("水解定律", "盐水解", "盐溶液水解"),
            ("络合定律", "络合物形成", "络合物形成反应"),
        ]
    }

    /// 无机化合物类型
    pub fn compound_types(&self) -> Vec<&'static str> {
        vec![
            "氧化物",
            "酸",
            "碱",
            "盐",
            "氢化物",
            "卤化物",
            "硫化物",
            "氮化物",
            "碳化物",
            "配位化合物",
            "簇合物",
            "金属有机化合物",
        ]
    }

    /// 元素分类
    pub fn element_categories(&self) -> Vec<&'static str> {
        vec![
            "主族元素",
            "过渡元素",
            "镧系元素",
            "锕系元素",
            "碱金属",
            "碱土金属",
            "稀有气体",
            "卤素",
        ]
    }
}

impl Default for InorganicChemistryLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for InorganicChemistryLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("inorganic_chemistry")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【无机化学定律】\n\n周期定律:\n{}\n\n化学键定律:\n{}\n\n配位定律:\n{}\n",
            self.periodic_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bonding_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.coordination_laws().iter()
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
    fn test_inorganic_chemistry_laws() {
        let laws = InorganicChemistryLaws::new();
        assert!(!laws.periodic_laws().is_empty());
        assert!(!laws.bonding_laws().is_empty());
    }
}