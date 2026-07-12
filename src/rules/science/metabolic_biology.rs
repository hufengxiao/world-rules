//! 代谢生物学定律
//!
//! 代谢生物学研究生物体的代谢过程和代谢调控，
//! 包括能量代谢、物质代谢、代谢网络和代谢调控机制。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 代谢生物学定律集合
pub struct MetabolicBiologyLaws {
    metadata: RuleMetadata,
}

impl MetabolicBiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("代谢生物学定律", "代谢生物学基本定律和代谢调控机制")
                .with_origin("生物学")
                .with_tags(vec!["科学".into(), "生物".into(), "代谢".into()]),
        }
    }

    /// 能量代谢定律
    pub fn energy_metabolism_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("能量守恒定律", "生物能量守恒", "生物系统能量守恒"),
            ("ATP循环定律", "能量货币", "ATP作为能量载体"),
            ("氧化磷酸化定律", "ATP合成", "电子传递链合成ATP"),
            ("糖酵解定律", "葡萄糖分解", "葡萄糖分解产能"),
            ("三羧酸循环定律", "TCA循环", "柠檬酸循环产能"),
            ("呼吸定律", "细胞呼吸", "细胞呼吸产能"),
            ("光合作用定律", "能量捕获", "光合作用捕获能量"),
            ("热力学定律", "生物热力学", "生物系统热力学"),
        ]
    }

    /// 物质代谢定律
    pub fn substance_metabolism_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("糖代谢定律", "碳水化合物代谢", "糖类物质代谢"),
            ("脂代谢定律", "脂肪代谢", "脂质代谢过程"),
            ("蛋白质代谢定律", "氨基酸代谢", "蛋白质合成分解"),
            ("核酸代谢定律", "核苷酸代谢", "核酸代谢过程"),
            ("维生素代谢定律", "维生素代谢", "维生素代谢利用"),
            ("矿物质代谢定律", "矿物质代谢", "矿物质代谢利用"),
            ("废物排泄定律", "代谢废物", "代谢废物排泄"),
            ("合成代谢定律", "物质合成", "生物合成过程"),
        ]
    }

    /// 代谢调控定律
    pub fn metabolic_regulation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("酶调控定律", "酶活性调节", "酶活性调节代谢"),
            ("激素调控定律", "激素调节", "激素调控代谢"),
            ("反馈调控定律", "反馈机制", "反馈抑制调控"),
            ("变构调控定律", "变构效应", "变构调节代谢"),
            ("共价修饰定律", "修饰调控", "共价修饰调节"),
            ("基因调控定律", "表达调控", "基因表达调控代谢"),
            ("转录调控定律", "转录调节", "转录水平调控"),
            ("翻译调控定律", "翻译调节", "翻译水平调控"),
        ]
    }

    /// 代谢网络定律
    pub fn metabolic_network_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("代谢通路定律", "代谢路径", "代谢反应通路"),
            ("代谢网络定律", "网络结构", "代谢反应网络"),
            ("代谢节点定律", "关键节点", "代谢网络关键节点"),
            ("代谢通量定律", "代谢流量", "代谢反应通量"),
            ("代谢分支定律", "分支途径", "代谢途径分支"),
            ("代谢循环定律", "循环通路", "代谢循环过程"),
            ("代谢整合定律", "网络整合", "代谢网络整合"),
            ("代谢适应性定律", "适应调节", "代谢适应变化"),
        ]
    }

    /// 代谢类型
    pub fn metabolism_types(&self) -> Vec<&'static str> {
        vec![
            "基础代谢: 维持生命最低能量消耗",
            "静息代谢: 静息状态下能量消耗",
            "活动代谢: 活动增加的能量消耗",
            "运动代谢: 运动时能量代谢",
            "合成代谢: 物质合成代谢",
            "分解代谢: 物质分解代谢",
            "有氧代谢: 有氧条件代谢",
            "无氧代谢: 无氧条件代谢",
        ]
    }

    /// 代谢疾病
    pub fn metabolic_diseases(&self) -> Vec<&'static str> {
        vec![
            "糖尿病: 糖代谢紊乱疾病",
            "肥胖症: 脂肪代谢紊乱",
            "代谢综合征: 多种代谢紊乱组合",
            "甲状腺疾病: 甲状腺代谢异常",
            "高脂血症: 血脂代谢紊乱",
            "痛风: 尿酸代谢紊乱",
            "肝性脑病: 肝代谢功能障碍",
            "营养不良: 营养代谢异常",
        ]
    }

    /// 代谢指标
    pub fn metabolic_indicators(&self) -> Vec<&'static str> {
        vec![
            "血糖: 血液中葡萄糖浓度",
            "血脂: 血液中脂质浓度",
            "尿酸: 血液中尿酸浓度",
            "肝功能: 肝脏代谢功能指标",
            "肾功能: 肾脏代谢功能指标",
            "甲状腺功能: 甲状腺激素水平",
            "代谢率: 基础代谢率测定",
            "体成分: 身体组成比例分析",
        ]
    }
}

impl Default for MetabolicBiologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MetabolicBiologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("metabolic_biology")
    }

    fn explain(&self) -> String {
        format!(
            "【代谢生物学定律】\n\n\
            能量代谢定律:\n{}\n\n\
            物质代谢定律:\n{}\n\n\
            代谢调控定律:\n{}\n\n\
            代谢网络定律:\n{}\n\n\
            代谢类型:\n{}\n\n\
            代谢疾病:\n{}\n",
            self.energy_metabolism_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.substance_metabolism_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.metabolic_regulation_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.metabolic_network_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.metabolism_types()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.metabolic_diseases()
                .iter()
                .map(|d| format!("  • {}", d))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metabolic_biology_laws() {
        let rules = MetabolicBiologyLaws::new();
        assert_eq!(rules.energy_metabolism_laws().len(), 8);
        assert_eq!(rules.substance_metabolism_laws().len(), 8);
        assert_eq!(rules.metabolic_regulation_laws().len(), 8);
        assert_eq!(rules.metabolic_network_laws().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_energy_laws() {
        let rules = MetabolicBiologyLaws::new();
        let laws = rules.energy_metabolism_laws();
        assert!(laws.iter().any(|(n, _, _)| n.contains("ATP")));
    }

    #[test]
    fn test_metabolic_types() {
        let rules = MetabolicBiologyLaws::new();
        assert_eq!(rules.metabolism_types().len(), 8);
    }
}