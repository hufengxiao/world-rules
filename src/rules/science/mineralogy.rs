//! 矿物学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 矿物学定律集合
pub struct MineralogyLaws {
    metadata: RuleMetadata,
}

impl MineralogyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "矿物学定律",
                "矿物学基本定律"
            )
            .with_origin("地质学")
            .with_tags(vec!["科学".into(), "地质".into(), "矿物".into()]),
        }
    }

    /// 矿物形成定律
    pub fn formation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("结晶定律", "晶体形成", "矿物结晶过程"),
            ("生长定律", "晶体生长", "晶体生长规律"),
            ("相变定律", "矿物相变", "矿物相变过程"),
            ("溶解定律", "溶解沉淀", "溶解沉淀平衡"),
            ("交代定律", "矿物交代", "矿物交代作用"),
            ("重结晶定律", "重结晶", "矿物重结晶过程"),
            ("沉淀定律", "矿物沉淀", "矿物沉淀规律"),
        ]
    }

    /// 矿物结构定律
    pub fn structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("晶体结构定律", "内部结构", "矿物晶体结构"),
            ("晶系定律", "七大晶系", "矿物晶系分类"),
            ("对称定律", "对称性质", "晶体对称性"),
            ("解理定律", "解理特征", "矿物解理性质"),
            ("断口定律", "断口特征", "矿物断口类型"),
            ("硬度定律", "莫氏硬度", "矿物硬度等级"),
            ("密度定律", "比重特征", "矿物比重特性"),
        ]
    }

    /// 矿物分类定律
    pub fn classification_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("化学分类定律", "化学成分", "化学成分分类"),
            ("结构分类定律", "结构类型", "结构分类方法"),
            ("成因分类定律", "形成条件", "成因分类系统"),
            ("矿物族定律", "矿物族", "矿物族分类"),
            ("矿物类定律", "矿物大类", "矿物大类划分"),
            ("变种定律", "矿物变种", "矿物变种分类"),
        ]
    }

    /// 矿物鉴定定律
    pub fn identification_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("鉴定定律", "鉴定方法", "矿物鉴定技术"),
            ("光学定律", "光学性质", "光学鉴定方法"),
            ("化学定律", "化学鉴定", "化学鉴定方法"),
            ("物理定律", "物理性质", "物理性质鉴定"),
            ("X射线定律", "衍射鉴定", "X射线衍射鉴定"),
            ("显微镜定律", "显微鉴定", "显微镜鉴定方法"),
        ]
    }

    /// 矿物大类
    pub fn mineral_classes(&self) -> Vec<&'static str> {
        vec![
            "自然元素",
            "硫化物",
            "氧化物",
            "氢氧化物",
            "卤化物",
            "碳酸盐",
            "硫酸盐",
            "硅酸盐",
        ]
    }

    /// 重要矿物
    pub fn important_minerals(&self) -> Vec<&'static str> {
        vec![
            "石英",
            "长石",
            "云母",
            "方解石",
            "金刚石",
            "石墨",
            "黄铁矿",
            "赤铁矿",
        ]
    }
}

impl Default for MineralogyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MineralogyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("mineralogy")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【矿物学定律】\n\n形成定律:\n{}\n\n结构定律:\n{}\n\n分类定律:\n{}\n",
            self.formation_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.structure_laws().iter()
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
    fn test_mineralogy_laws() {
        let laws = MineralogyLaws::new();
        assert!(!laws.formation_laws().is_empty());
        assert!(!laws.structure_laws().is_empty());
    }
}