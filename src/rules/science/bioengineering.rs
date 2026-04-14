//! 生物工程定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 生物工程定律集合
pub struct BioengineeringLaws {
    metadata: RuleMetadata,
}

impl BioengineeringLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "生物工程定律",
                "生物工程基本定律"
            )
            .with_origin("工程")
            .with_tags(vec!["科学".into(), "工程".into(), "生物".into()]),
        }
    }

    /// 基因工程定律
    pub fn genetic_engineering_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("基因操作定律", "基因修改", "基因工程技术"),
            ("克隆定律", "DNA克隆", "基因克隆技术"),
            ("表达定律", "基因表达", "基因表达控制"),
            ("载定律体", "载体系统", "基因载体设计"),
            ("转基因定律", "基因转移", "转基因技术"),
            ("基因编辑定律", "精确编辑", "基因编辑技术"),
            ("基因治疗定律", "治疗应用", "基因治疗原理"),
        ]
    }

    /// 蛋白质工程定律
    pub fn protein_engineering_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("蛋白质设计定律", "理性设计", "蛋白质设计方法"),
            ("定向进化定律", "筛选进化", "蛋白质定向进化"),
            ("结构预测定律", "结构计算", "蛋白质结构预测"),
            ("功能改造定律", "功能优化", "蛋白质功能改造"),
            ("稳定性定律", "稳定性增强", "蛋白质稳定性"),
            ("表达定律", "高效表达", "蛋白质高效表达"),
        ]
    }

    /// 细胞工程定律
    pub fn cell_engineering_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("细胞培养定律", "培养条件", "细胞培养技术"),
            ("细胞融合定律", "融合技术", "细胞融合方法"),
            ("干细胞定律", "干细胞应用", "干细胞技术"),
            ("组织工程定律", "组织构建", "组织工程方法"),
            ("细胞治疗定律", "治疗应用", "细胞治疗技术"),
            ("器官培养定律", "器官构建", "器官培养技术"),
        ]
    }

    /// 发酵工程定律
    pub fn fermentation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("发酵定律", "发酵过程", "发酵技术原理"),
            ("菌种定律", "菌种选育", "菌种筛选培养"),
            ("过程控制定律", "参数控制", "发酵过程控制"),
            ("产物定律", "产物回收", "发酵产物提取"),
            ("连续发酵定律", "连续生产", "连续发酵技术"),
            ("固态发酵定律", "固态培养", "固态发酵方法"),
        ]
    }

    /// 生物技术应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "医药生产",
            "农业育种",
            "食品加工",
            "环境保护",
            "能源生产",
            "材料制造",
            "诊断检测",
            "生物制药",
        ]
    }

    /// 生物工程方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "基因克隆",
            "PCR技术",
            "细胞培养",
            "蛋白质纯化",
            "发酵工艺",
            "生物传感器",
            "基因测序",
            "生物信息学",
        ]
    }
}

impl Default for BioengineeringLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BioengineeringLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("bioengineering")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【生物工程定律】\n\n基因工程定律:\n{}\n\n蛋白质工程定律:\n{}\n\n细胞工程定律:\n{}\n",
            self.genetic_engineering_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.protein_engineering_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cell_engineering_laws().iter()
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
    fn test_bioengineering_laws() {
        let laws = BioengineeringLaws::new();
        assert!(!laws.genetic_engineering_laws().is_empty());
        assert!(!laws.cell_engineering_laws().is_empty());
    }
}