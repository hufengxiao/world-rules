//! 免疫生物学规则
//!
//! 免疫系统的生物学原理，包括免疫细胞、免疫应答、
//! 免疫调节、免疫记忆等核心概念。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 免疫生物学规则集合
pub struct ImmunobiologyRules {
    metadata: RuleMetadata,
}

impl ImmunobiologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("免疫生物学规则", "免疫系统基础生物学原理")
                .with_origin("免疫生物学")
                .with_tags(vec!["科学".into(), "生命科学".into(), "免疫学".into()]),
        }
    }

    /// 免疫细胞类型定律
    pub fn immune_cell_types(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("T细胞定律", "适应性免疫", "T细胞介导适应性免疫"),
            ("B细胞定律", "抗体产生", "B细胞产生抗体"),
            ("NK细胞定律", "天然杀伤", "NK细胞天然杀伤肿瘤病毒"),
            ("巨噬细胞定律", "吞噬清除", "巨噬细胞吞噬清除病原"),
            ("树突细胞定律", "抗原呈递", "树突细胞呈递抗原"),
            ("粒细胞定律", "炎症反应", "粒细胞参与炎症"),
            ("肥大细胞定律", "过敏反应", "肥大细胞介导过敏"),
        ]
    }

    /// 免疫应答定律
    pub fn immune_response_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("先天免疫定律", "快速响应", "先天免疫快速响应感染"),
            ("适应性免疫定律", "特异应答", "适应性免疫特异应答"),
            ("体液免疫定律", "抗体介导", "抗体介导体液免疫"),
            ("细胞免疫定律", "细胞介导", "细胞介导细胞免疫"),
            ("免疫激活定律", "抗原刺激", "抗原刺激激活免疫"),
            ("免疫耐受定律", "不攻击自身", "免疫系统不攻击自身"),
            ("免疫记忆定律", "再次应答", "记忆细胞快速再次应答"),
        ]
    }

    /// 抗原呈递定律
    pub fn antigen_presentation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("MHC-I定律", "内源抗原", "MHC-I呈递内源性抗原"),
            ("MHC-II定律", "外源抗原", "MHC-II呈递外源性抗原"),
            ("交叉呈递定律", "外源内源呈递", "交叉呈递外源抗原给MHC-I"),
            ("抗原加工定律", "蛋白酶解", "抗原被蛋白酶降解"),
            ("抗原肽定律", "肽段结合", "抗原肽段结合MHC"),
            ("共刺激定律", "激活信号", "共刺激信号激活T细胞"),
            ("TCR识别定律", "特异性识别", "TCR特异性识别抗原肽"),
        ]
    }

    /// T细胞发育定律
    pub fn t_cell_development(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("胸腺发育定律", "T细胞成熟", "T细胞在胸腺成熟"),
            ("阳性选择定律", "MHC识别", "T细胞识别MHC被保留"),
            ("阴性选择定律", "自身耐受", "自身反应T细胞被清除"),
            ("CD4分化定律", "辅助T细胞", "CD4+T细胞分化为辅助细胞"),
            ("CD8分化定律", "杀伤T细胞", "CD8+T细胞分化为杀伤细胞"),
            ("效应分化定律", "效应功能", "效应T细胞分化"),
            ("记忆形成定律", "记忆T细胞", "记忆T细胞形成"),
        ]
    }

    /// B细胞发育定律
    pub fn b_cell_development(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("骨髓发育定律", "B细胞成熟", "B细胞在骨髓成熟"),
            ("重组定律", "BCR多样性", "BCR基因重组产生多样性"),
            ("类别转换定律", "抗体类型", "抗体类别转换"),
            ("亲和力成熟定律", "抗体优化", "抗体亲和力成熟"),
            ("浆细胞定律", "抗体分泌", "浆细胞分泌大量抗体"),
            ("记忆B细胞定律", "长期记忆", "记忆B细胞长期存在"),
            ("体细胞高频突变定律", "突变优化", "BCR基因高频突变"),
        ]
    }

    /// 免疫调节定律
    pub fn immune_regulation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("调节T细胞定律", "免疫抑制", "Treg细胞抑制免疫"),
            ("细胞因子调节定律", "信号调控", "细胞因子调控免疫"),
            ("抑制性受体定律", "负调控", "抑制性受体负调控"),
            ("免疫检查点定律", "防止过度", "检查点防止过度免疫"),
            ("抑制因子定律", "免疫抑制", "抑制因子抑制免疫"),
            ("反馈调节定律", "反馈控制", "免疫反馈调节"),
            ("网络调节定律", "网络平衡", "免疫网络调节平衡"),
        ]
    }

    /// 免疫病理定律
    pub fn immune_pathology(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("过敏定律", "过度反应", "过敏是免疫过度反应"),
            ("自身免疫定律", "攻击自身", "自身免疫攻击自身组织"),
            ("免疫缺陷定律", "功能缺失", "免疫缺陷导致易感染"),
            ("超敏反应定律", "过敏类型", "超敏反应分类"),
            ("免疫增生定律", "异常增生", "免疫细胞异常增生"),
            ("移植排斥定律", "排斥反应", "免疫系统排斥移植物"),
            ("肿瘤免疫定律", "肿瘤逃逸", "肿瘤逃逸免疫监视"),
        ]
    }

    /// 免疫耐受定律
    pub fn immune_tolerance(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("中枢耐受定律", "发育筛选", "中枢发育时建立耐受"),
            ("外周耐受定律", "成熟后耐受", "成熟后建立耐受"),
            ("克隆删除定律", "细胞删除", "自身反应克隆删除"),
            ("克隆无能定律", "功能失活", "自身反应克隆失活"),
            ("克隆忽视定律", "不识别", "免疫系统忽视自身"),
            ("抑制性耐受定律", "主动抑制", "Treg主动抑制"),
            ("免疫隔离定律", "物理隔离", "免疫赦免部位隔离"),
        ]
    }

    /// 免疫记忆定律
    pub fn immune_memory(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("记忆T细胞定律", "快速应答", "记忆T细胞快速应答"),
            ("记忆B细胞定律", "抗体记忆", "记忆B细胞快速产生抗体"),
            ("长寿命定律", "长期存在", "记忆细胞长期存在"),
            ("再次应答定律", "更快更强", "再次应答更快更强"),
            ("交叉保护定律", "相似保护", "记忆细胞交叉保护"),
            ("记忆维持定律", "维持机制", "记忆细胞维持机制"),
            ("记忆多样性定律", "多种记忆", "多种记忆细胞亚群"),
        ]
    }

    /// 免疫进化定律
    pub fn immune_evolution(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("先天免疫进化定律", "古老系统", "先天免疫进化古老"),
            ("适应性免疫进化定律", "后出现", "适应性免疫较晚出现"),
            ("免疫多样性定律", "多样性进化", "免疫多样性进化"),
            ("免疫逃逸定律", "病原逃逸", "病原进化逃逸免疫"),
            ("免疫压力定律", "进化压力", "免疫压力驱动进化"),
            ("物种差异定律", "物种特异", "免疫系统物种差异"),
            ("免疫基因进化定律", "基因进化", "免疫基因快速进化"),
        ]
    }
}

impl Default for ImmunobiologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ImmunobiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("immunobiology")
    }

    fn explain(&self) -> String {
        format!(
            "【免疫生物学规则】\n\n\
            免疫生物学研究免疫系统的生物学原理，是理解免疫功能和疾病的基础。\n\n\
            免疫细胞类型:\n{}\n\n\
            免疫应答:\n{}\n\n\
            抗原呈递:\n{}\n\n\
            T细胞发育:\n{}\n\n\
            B细胞发育:\n{}\n\n\
            免疫调节:\n{}\n\n\
            免疫病理:\n{}\n\n\
            免疫耐受:\n{}\n\n\
            免疫记忆:\n{}\n\n\
            免疫进化:\n{}",
            self.immune_cell_types()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.immune_response_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.antigen_presentation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.t_cell_development()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.b_cell_development()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.immune_regulation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.immune_pathology()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.immune_tolerance()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.immune_memory()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.immune_evolution()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immunobiology_rules() {
        let rules = ImmunobiologyRules::new();
        assert_eq!(rules.immune_cell_types().len(), 7);
        assert_eq!(rules.immune_response_laws().len(), 7);
        assert_eq!(rules.antigen_presentation().len(), 7);
        assert_eq!(rules.t_cell_development().len(), 7);
        assert_eq!(rules.b_cell_development().len(), 7);
        assert_eq!(rules.immune_regulation().len(), 7);
        assert_eq!(rules.immune_pathology().len(), 7);
        assert_eq!(rules.immune_tolerance().len(), 7);
        assert_eq!(rules.immune_memory().len(), 7);
        assert_eq!(rules.immune_evolution().len(), 7);
    }

    #[test]
    fn test_immunobiology_metadata() {
        let rules = ImmunobiologyRules::new();
        assert_eq!(rules.metadata().name, "免疫生物学规则");
    }
}
