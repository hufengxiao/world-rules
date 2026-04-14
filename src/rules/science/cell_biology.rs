//! 细胞生物学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 细胞生物学定律集合
pub struct CellBiologyLaws {
    metadata: RuleMetadata,
}

impl CellBiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "细胞生物学定律",
                "细胞生物学基本定律"
            )
            .with_origin("生物学")
            .with_tags(vec!["科学".into(), "生物".into(), "细胞".into()]),
        }
    }

    /// 细胞结构定律
    pub fn structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("细胞学说定律", "细胞是基本单位", "所有生物由细胞组成"),
            ("细胞膜定律", "选择性通透", "细胞膜选择性渗透"),
            ("细胞核定律", "遗传中心", "细胞核是遗传中心"),
            ("细胞器定律", "功能分工", "细胞器分工合作"),
            ("细胞骨架定律", "支撑结构", "细胞骨架支撑"),
            ("内膜系统定律", "膜网络", "内膜系统相互联系"),
            ("细胞壁定律", "植物支撑", "植物细胞壁支撑"),
        ]
    }

    /// 细胞分裂定律
    pub fn division_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("有丝分裂定律", "复制分裂", "染色体复制均分"),
            ("减数分裂定律", "减半分裂", "染色体减半"),
            ("细胞周期定律", "G1-S-G2-M", "细胞周期阶段"),
            ("DNA合成定律", "S期复制", "S期DNA复制"),
            ("分裂期定律", "M期分裂", "M期细胞分裂"),
            ("纺锤体定律", "染色体牵引", "纺锤体牵引染色体"),
            ("胞质分裂定律", "细胞分裂", "细胞质分裂"),
        ]
    }

    /// 细胞分化定律
    pub fn differentiation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("分化定律", "基因表达", "基因选择性表达"),
            ("全能性定律", "全能细胞", "细胞全能性"),
            ("多能性定律", "多能细胞", "细胞多能性"),
            ("单能性定律", "单能细胞", "细胞单能性"),
            ("去分化定律", "逆转分化", "细胞去分化"),
            ("转分化定律", "类型转变", "细胞类型转变"),
            ("干细胞定律", "自我更新", "干细胞特性"),
        ]
    }

    /// 细胞信号定律
    pub fn signaling_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("信号传导定律", "信息传递", "细胞信号传导"),
            ("受体定律", "信号接收", "受体接收信号"),
            ("配体定律", "信号分子", "配体传递信号"),
            ("第二信使定律", "信号放大", "信号放大机制"),
            ("级联反应定律", "信号级联", "信号传递级联"),
            ("反馈调节定律", "信号调节", "信号反馈调节"),
            ("跨膜信号定律", "膜信号传递", "跨膜信号传导"),
        ]
    }

    /// 细胞运输定律
    pub fn transport_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("被动运输定律", "自由扩散", "物质被动扩散"),
            ("主动运输定律", "耗能运输", "物质耗能运输"),
            ("协助扩散定律", "载体帮助", "载体协助扩散"),
            ("离子泵定律", "离子运输", "离子泵运输"),
            ("胞吞定律", "物质摄入", "细胞胞吞"),
            ("胞吐定律", "物质排出", "细胞胞吐"),
            ("渗透定律", "水分子移动", "渗透现象"),
        ]
    }

    /// 细胞功能
    pub fn functions(&self) -> Vec<&'static str> {
        vec![
            "物质合成",
            "能量代谢",
            "信息传递",
            "物质运输",
            "细胞运动",
            "细胞死亡",
            "细胞保护",
            "细胞修复",
        ]
    }

    /// 细胞器类型
    pub fn organelles(&self) -> Vec<&'static str> {
        vec![
            "线粒体",
            "叶绿体",
            "内质网",
            "高尔基体",
            "溶酶体",
            "核糖体",
            "中心体",
            "液泡",
        ]
    }
}

impl Default for CellBiologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CellBiologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("cell_biology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【细胞生物学定律】\n\n结构定律:\n{}\n\n分裂定律:\n{}\n\n分化定律:\n{}\n",
            self.structure_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.division_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.differentiation_laws().iter()
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
    fn test_cell_biology_laws() {
        let laws = CellBiologyLaws::new();
        assert!(!laws.structure_laws().is_empty());
        assert!(!laws.division_laws().is_empty());
    }
}