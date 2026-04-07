//! 生物学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 生物学定律类型
#[derive(Debug, Clone)]
pub enum BiologyLawType {
    /// 孟德尔遗传定律
    MendelLaw,
    /// 自然选择学说
    NaturalSelection,
    /// 中心法则
    CentralDogma,
    /// 细胞学说
    CellTheory,
}

impl BiologyLawType {
    pub fn name(&self) -> &'static str {
        match self {
            BiologyLawType::MendelLaw => "孟德尔遗传定律",
            BiologyLawType::NaturalSelection => "自然选择学说",
            BiologyLawType::CentralDogma => "中心法则",
            BiologyLawType::CellTheory => "细胞学说",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            BiologyLawType::MendelLaw => "基因的分离和自由组合规律",
            BiologyLawType::NaturalSelection => "适者生存，优胜劣汰",
            BiologyLawType::CentralDogma => "DNA→RNA→蛋白质",
            BiologyLawType::CellTheory => "细胞是生命的基本单位",
        }
    }
}

/// 生物学规则
pub struct BiologyRules {
    metadata: RuleMetadata,
}

impl BiologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "生物学定律",
                "生物学基本定律和理论"
            )
            .with_origin("生物学")
            .with_tags(vec!["科学".into(), "生物学".into()]),
        }
    }

    /// 孟德尔定律详解
    pub fn mendel_laws(&self) -> Vec<&'static str> {
        vec![
            "分离定律: 等位基因在形成配子时分离",
            "自由组合定律: 不同性状的基因自由组合",
            "显性与隐性: 显性基因掩盖隐性基因表达",
            "基因型与表现型的区别",
        ]
    }

    /// 中心法则
    pub fn central_dogma(&self) -> Vec<&'static str> {
        vec![
            "DNA复制: DNA → DNA",
            "转录: DNA → mRNA",
            "翻译: mRNA → 蛋白质",
            "逆转录: RNA → DNA (某些病毒)",
        ]
    }

    /// 细胞学说要点
    pub fn cell_theory(&self) -> Vec<&'static str> {
        vec![
            "所有生物由细胞组成",
            "细胞是生命的基本单位",
            "所有细胞来自已有细胞",
            "细胞包含遗传信息",
        ]
    }

    /// 自然选择要点
    pub fn natural_selection(&self) -> Vec<&'static str> {
        vec![
            "变异: 个体间存在差异",
            "遗传: 部分变异可遗传",
            "选择: 环境选择有利变异",
            "适应: 有利特征在种群中积累",
        ]
    }

    /// DNA结构
    pub fn dna_structure(&self) -> Vec<&'static str> {
        vec![
            "双螺旋结构",
            "四种碱基: A、T、G、C",
            "碱基配对: A-T, G-C",
            "磷酸-糖骨架",
            "氢键连接",
        ]
    }

    /// 遗传密码
    pub fn genetic_code(&self) -> Vec<&'static str> {
        vec![
            "三联体密码: 3个碱基编码1个氨基酸",
            "64种密码子",
            "起始密码: AUG",
            "终止密码: UAA、UAG、UGA",
            "密码子简并性: 多数氨基酸有多个密码子",
        ]
    }

    /// 所有定律
    pub fn all_laws(&self) -> Vec<BiologyLawType> {
        vec![
            BiologyLawType::CellTheory,
            BiologyLawType::MendelLaw,
            BiologyLawType::NaturalSelection,
            BiologyLawType::CentralDogma,
        ]
    }
}

impl Default for BiologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("biology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【生物学定律】\n\n\
            {}\n\n\
            细胞学说:\n{}\n\n\
            孟德尔遗传定律:\n{}\n\n\
            自然选择学说:\n{}\n\n\
            中心法则:\n{}\n\n\
            DNA结构:\n{}\n\n\
            遗传密码:\n{}\n",
            self.all_laws().iter()
                .map(|l| format!("▶ {}: {}", l.name(), l.description()))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cell_theory().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.mendel_laws().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.natural_selection().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.central_dogma().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.dna_structure().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.genetic_code().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biology_rules() {
        let rules = BiologyRules::new();
        assert_eq!(rules.all_laws().len(), 4);
    }
}