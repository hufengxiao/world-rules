//! 细胞生物学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 细胞生物学定律集合
pub struct CellBiologyLaws {
    metadata: RuleMetadata,
}

impl CellBiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("细胞生物学定律", "细胞生物学基本定律")
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

    /// 细胞凋亡定律
    pub fn apoptosis_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("程序性死亡定律", "细胞凋亡", "细胞程序性死亡过程"),
            ("凋亡信号定律", "信号通路", "凋亡信号传导途径"),
            ("caspase定律", "蛋白酶", "凋亡执行蛋白酶"),
            ("线粒体凋亡定律", "细胞色素C", "线粒体凋亡途径"),
            ("死亡受体定律", "外源途径", "死亡受体介导凋亡"),
        ]
    }

    /// 细胞代谢定律
    pub fn cell_metabolism_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("细胞呼吸定律", "有氧代谢", "细胞有氧呼吸产生ATP"),
            ("糖酵解定律", "无氧代谢", "细胞质中糖酵解"),
            ("三羧酸定律", "线粒体", "线粒体中TCA循环"),
            ("氧化磷酸化定律", "电子传递", "线粒体电子传递链"),
            ("脂肪酸氧化定律", "β氧化", "脂肪酸分解代谢"),
        ]
    }

    /// 细胞周期调控定律
    pub fn cell_cycle_regulation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("细胞周期蛋白定律", "cyclin", "周期蛋白周期性变化"),
            ("CDK定律", "激酶", "周期蛋白依赖性激酶"),
            ("检查点定律", "监控点", "细胞周期检查点控制"),
            ("p53定律", "抑癌基因", "p53调控细胞周期"),
            ("Rb定律", "视网膜母细胞瘤", "Rb蛋白调控细胞周期"),
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

    /// 细胞器
    pub fn cell_organelles(&self) -> Vec<&'static str> {
        vec![
            "线粒体: 细胞的发电站进行有氧呼吸产生ATP",
            "内质网: 粗面内质网合成蛋白光面内质网合成脂质",
            "高尔基体: 对蛋白质进行加工修饰分选和运输",
            "溶酶体: 含水解酶分解细胞内废物和外来物质",
            "核糖体: 由rRNA和蛋白质组成翻译mRNA合成蛋白质",
            "细胞骨架: 微管微丝和中间纤维维持细胞形态",
        ]
    }

    /// 细胞信号
    pub fn cell_signaling(&self) -> Vec<&'static str> {
        vec![
            "信号转导: 细胞外信号转化为细胞内响应的过程",
            "G蛋白偶联受体: 七次跨膜受体激活G蛋白传递信号",
            "受体酪氨酸激酶: 配体结合后自磷酸化启动信号级联",
            "第二信使: cAMP、Ca²⁺、IP3等细胞内信号分子",
            "MAPK通路: 从细胞表面到细胞核的重要信号通路",
            "细胞凋亡信号: 内源和外源途径激活caspase级联",
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
            "【细胞生物学定律】\n\n结构定律:\n{}\n\n分裂定律:\n{}\n\n分化定律:\n{}\n\n凋亡定律:\n{}\n\n细胞代谢定律:\n{}\n\n周期调控定律:\n{}\n",
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
                .join("\n"),
            self.apoptosis_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cell_metabolism_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cell_cycle_regulation_laws().iter()
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
