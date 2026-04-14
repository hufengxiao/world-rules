//! 微生物学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 微生物学定律集合
pub struct MicrobiologyLaws {
    metadata: RuleMetadata,
}

impl MicrobiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "微生物学定律",
                "微生物学基本定律"
            )
            .with_origin("生物学")
            .with_tags(vec!["科学".into(), "生物".into(), "微生物".into()]),
        }
    }

    /// 微生物生长定律
    pub fn growth_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("指数生长定律", "N = N₀×2^n", "微生物指数生长"),
            ("生长曲线定律", "四阶段", "延滞期对数期稳定期衰亡期"),
            ("代时定律", "分裂时间", "微生物分裂时间"),
            ("生长速率定律", "繁殖速率", "微生物繁殖速率"),
            ("生长条件定律", "营养温度", "生长依赖营养温度"),
            ("同步生长定律", "同步分裂", "微生物同步分裂"),
            ("连续培养定律", "恒定条件", "连续培养保持恒定"),
        ]
    }

    /// 微生物代谢定律
    pub fn metabolism_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("发酵定律", "无氧代谢", "微生物发酵代谢"),
            ("呼吸定律", "有氧代谢", "微生物呼吸代谢"),
            ("自养定律", "自养代谢", "自养微生物代谢"),
            ("异养定律", "异养代谢", "异养微生物代谢"),
            ("光合定律", "光合微生物", "光合微生物代谢"),
            ("固氮定律", "氮固定", "微生物固氮"),
            ("厌氧定律", "厌氧代谢", "厌氧微生物代谢"),
        ]
    }

    /// 微生物遗传定律
    pub fn genetics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("突变定律", "自发突变", "微生物自发突变"),
            ("转化定律", "DNA摄入", "细菌转化"),
            ("转导定律", "噬菌体介导", "噬菌体介导基因转移"),
            ("接合定律", "细胞接触", "细菌接合"),
            ("质粒定律", "质粒基因", "质粒携带基因"),
            ("基因工程定律", "重组技术", "微生物基因工程"),
        ]
    }

    /// 病毒定律
    pub fn virus_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("病毒结构定律", "核酸蛋白质", "病毒组成结构"),
            ("病毒复制定律", "宿主依赖", "病毒依赖宿主复制"),
            ("病毒吸附定律", "受体结合", "病毒吸附宿主细胞"),
            ("病毒侵入定律", "细胞进入", "病毒进入细胞"),
            ("病毒释放定律", "细胞释放", "病毒释放机制"),
            ("病毒变异定律", "高频变异", "病毒变异"),
            ("噬菌体定律", "细菌病毒", "噬菌体特性"),
        ]
    }

    /// 微生物分类
    pub fn categories(&self) -> Vec<&'static str> {
        vec![
            "细菌",
            "真菌",
            "病毒",
            "放线菌",
            "蓝细菌",
            "支原体",
            "衣原体",
            "立克次体",
            "螺旋体",
            "原生动物",
        ]
    }

    /// 微生物应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "食品发酵",
            "抗生素生产",
            "疫苗制备",
            "污水处理",
            "生物修复",
            "基因工程",
            "生物技术",
            "农业生产",
        ]
    }
}

impl Default for MicrobiologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MicrobiologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("microbiology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【微生物学定律】\n\n生长定律:\n{}\n\n代谢定律:\n{}\n\n病毒定律:\n{}\n",
            self.growth_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.metabolism_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.virus_laws().iter()
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
    fn test_microbiology_laws() {
        let laws = MicrobiologyLaws::new();
        assert!(!laws.growth_laws().is_empty());
        assert!(!laws.virus_laws().is_empty());
    }
}