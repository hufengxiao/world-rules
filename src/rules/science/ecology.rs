//! 生态学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 生态学定律集合
pub struct EcologyLaws {
    metadata: RuleMetadata,
}

impl EcologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "生态学定律",
                "生态学基本定律"
            )
            .with_origin("生物学")
            .with_tags(vec!["科学".into(), "生物".into(), "生态".into()]),
        }
    }

    /// 生态系统定律
    pub fn ecosystem_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("能量流动定律", "单向流动", "能量在生态系统中单向流动"),
            ("物质循环定律", "循环利用", "物质在生态系统中循环"),
            ("生态金字塔定律", "能量递减", "营养级越高能量越少"),
            ("食物链定律", "营养关系", "捕食与被捕食关系"),
            ("食物网定律", "复杂关系", "多食物链交织"),
            ("生态平衡定律", "动态平衡", "生态系统动态平衡"),
            ("生态系统稳定性定律", "抵抗恢复", "生态系统稳定"),
            ("生态位定律", "功能位置", "物种功能定位"),
        ]
    }

    /// 种群生态定律
    pub fn population_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("种群增长定律", "指数增长", "种群指数增长"),
            ("环境容纳量定律", "K值限制", "环境最大容纳量"),
            ("逻辑斯蒂增长定律", "S型曲线", "种群S型增长"),
            ("种群密度定律", "数量密度", "种群数量密度"),
            ("年龄结构定律", "年龄分布", "种群年龄分布"),
            ("性别比例定律", "性别比", "种群性别比例"),
            ("出生率死亡率定律", "增长决定", "出生率死亡率决定增长"),
            ("迁移定律", "种群流动", "种群迁入迁出"),
        ]
    }

    /// 群落生态定律
    pub fn community_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("竞争定律", "资源竞争", "物种竞争资源"),
            ("捕食定律", "捕食关系", "捕食者与被捕食者"),
            ("共生定律", "互利共生", "物种互利共生"),
            ("寄生定律", "寄生关系", "寄生者与宿主"),
            ("演替定律", "群落变化", "群落演替过程"),
            ("优势种定律", "主导物种", "群落主导物种"),
            ("多样性定律", "物种丰富", "物种多样性"),
            ("边缘效应定律", "边缘丰富", "群落边缘多样性高"),
        ]
    }

    /// 生物地球化学循环定律
    pub fn biogeochemical_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("碳循环定律", "碳交换", "碳在生态系统循环"),
            ("氮循环定律", "氮固定释放", "氮元素循环"),
            ("水循环定律", "蒸发降水", "水循环过程"),
            ("磷循环定律", "沉积循环", "磷元素循环"),
            ("硫循环定律", "硫交换", "硫元素循环"),
            ("氧循环定律", "光合呼吸", "氧元素循环"),
        ]
    }

    /// 保护生态定律
    pub fn conservation_laws(&self) -> Vec<&'static str> {
        vec![
            "生物多样性保护",
            "栖息地保护",
            "濒危物种保护",
            "生态恢复",
            "可持续发展",
            "生态系统服务",
            "生态红线",
            "自然保护区",
        ]
    }

    /// 生态因子
    pub fn ecological_factors(&self) -> Vec<&'static str> {
        vec![
            "温度",
            "光照",
            "水分",
            "土壤",
            "空气",
            "生物因子",
            "限制因子",
            "生态幅",
        ]
    }
}

impl Default for EcologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EcologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("ecology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【生态学定律】\n\n生态系统定律:\n{}\n\n种群定律:\n{}\n\n群落定律:\n{}\n",
            self.ecosystem_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.population_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.community_laws().iter()
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
    fn test_ecology_laws() {
        let laws = EcologyLaws::new();
        assert!(!laws.ecosystem_laws().is_empty());
        assert!(!laws.population_laws().is_empty());
    }
}