//! 考古学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 考古学定律集合
pub struct ArchaeologyLaws {
    metadata: RuleMetadata,
}

impl ArchaeologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "考古学定律",
                "考古学基本定律"
            )
            .with_origin("社会科学")
            .with_tags(vec!["科学".into(), "考古".into()]),
        }
    }

    /// 地层定律
    pub fn stratigraphy_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地层叠加定律", "上层较晚", "上层年代晚于下层"),
            ("地层连续定律", "连续堆积", "地层连续堆积"),
            ("地层间断定律", "堆积中断", "地层堆积中断"),
            ("地层对比定律", "地层对比", "不同地点地层对比"),
            ("地层标志定律", "标志地层", "地层标志物"),
            ("地层侵蚀定律", "地层缺失", "地层侵蚀缺失"),
        ]
    }

    /// 类型定律
    pub fn typology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("类型演变定律", "形态变化", "器物形态演变"),
            ("类型分类定律", "类型划分", "器物类型划分"),
            ("类型组合定律", "组合特征", "器物组合特征"),
            ("类型序列定律", "时间序列", "类型时间序列"),
            ("类型分布定律", "地域分布", "类型地域分布"),
            ("类型标准定律", "标准器物", "类型标准器"),
        ]
    }

    /// 定年定律
    pub fn dating_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("相对定年定律", "相对年代", "相对年代确定"),
            ("绝对定年定律", "绝对年代", "绝对年代测定"),
            ("碳十四定律", "C14测年", "放射性碳测年"),
            ("热释光定律", "TL测年", "热释光测年"),
            ("树轮定律", "树木年轮", "树木年轮定年"),
            ("考古地磁定律", "地磁测年", "考古地磁定年"),
            ("钾氩定律", "K-Ar测年", "钾氩测年法"),
        ]
    }

    /// 考古解释定律
    pub fn interpretation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("功能解释定律", "器物功能", "器物使用功能"),
            ("行为解释定律", "人类行为", "人类行为推测"),
            ("文化解释定律", "文化意义", "文化意义解读"),
            ("社会解释定律", "社会组织", "社会组织推测"),
            ("环境解释定律", "环境关系", "人地关系解读"),
            ("经济解释定律", "经济活动", "经济活动推测"),
        ]
    }

    /// 考古时期
    pub fn periods(&self) -> Vec<&'static str> {
        vec![
            "旧石器时代",
            "新石器时代",
            "青铜时代",
            "铁器时代",
            "历史时期",
            "古典时期",
            "中世纪",
            "现代时期",
        ]
    }

    /// 考古方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "田野发掘",
            "地表调查",
            "遥感探测",
            "地理信息",
            "实验室分析",
            "文物保护",
            "数字考古",
            "公众考古",
        ]
    }
}

impl Default for ArchaeologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ArchaeologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("archaeology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【考古学定律】\n\n地层定律:\n{}\n\n类型定律:\n{}\n\n定年定律:\n{}\n",
            self.stratigraphy_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.typology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dating_laws().iter()
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
    fn test_archaeology_laws() {
        let laws = ArchaeologyLaws::new();
        assert!(!laws.stratigraphy_laws().is_empty());
        assert!(!laws.typology_laws().is_empty());
    }
}