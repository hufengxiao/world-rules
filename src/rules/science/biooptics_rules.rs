//! 生物光学规则
//!
//! 生物体光学现象和原理，包括视觉系统、生物发光、
//! 光感受、光治疗等核心概念。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 生物光学规则集合
pub struct BioopticsRules {
    metadata: RuleMetadata,
}

impl BioopticsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("生物光学规则", "生物体光学现象和原理")
                .with_origin("生物光学")
                .with_tags(vec!["科学".into(), "生命科学".into(), "光学".into()]),
        }
    }

    /// 视觉系统定律
    pub fn visual_system(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("眼睛光学定律", "成像原理", "眼睛成像遵循光学原理"),
            ("屈光定律", "光线折射", "角膜晶状体折射光线"),
            ("调节定律", "焦距调节", "晶状体调节焦距"),
            ("视网膜定律", "感光层", "视网膜是感光层"),
            ("视锥细胞定律", "色觉", "视锥细胞负责色觉"),
            ("视杆细胞定律", "暗视觉", "视杆细胞负责暗视觉"),
            ("视力定律", "分辨能力", "视力衡量分辨能力"),
        ]
    }

    /// 光感受定律
    pub fn photoreception(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("光感受器定律", "光接收", "光感受器接收光信号"),
            ("光化学定律", "光化学反应", "光引起化学反应"),
            ("视色素定律", "吸收光", "视色素吸收光"),
            ("信号转换定律", "光转电", "光信号转为电信号"),
            ("阈值定律", "最小光强", "最小可检测光强"),
            ("适应定律", "光照适应", "眼睛适应不同光照"),
            ("光谱敏感定律", "波长敏感", "不同波长敏感度不同"),
        ]
    }

    /// 生物发光定律
    pub fn bioluminescence(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("荧光定律", "发光蛋白", "荧光蛋白发光"),
            ("化学发光定律", "化学反应", "化学反应产生光"),
            ("荧光素定律", "发光物质", "荧光素产生光"),
            ("荧光酶定律", "催化发光", "荧光酶催化发光"),
            ("生物荧光定律", "发光生物", "某些生物发光"),
            ("冷光定律", "无热发光", "生物发光几乎不产热"),
            ("效率定律", "高效率", "生物发光效率高"),
        ]
    }

    /// 光合作用定律
    pub fn photosynthesis(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("光吸收定律", "色素吸收", "叶绿素吸收光"),
            ("能量转换定律", "光转化学", "光能转为化学能"),
            ("反应中心定律", "光反应中心", "反应中心进行光反应"),
            ("电子传递定律", "电子流动", "光激发电子传递"),
            ("光合色素定律", "吸收光谱", "光合色素吸收特定光"),
            ("光饱和定律", "饱和点", "光强超过饱和点无效"),
            ("光抑制定律", "光损伤", "过强光抑制光合"),
        ]
    }

    /// 光周期定律
    pub fn photoperiod(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("光周期定律", "光照时间", "光照时间影响生物"),
            ("长日植物定律", "需要长光照", "长日植物需要长光照"),
            ("短日植物定律", "需要短光照", "短日植物需要短光照"),
            ("开花诱导定律", "光照诱导", "光照诱导开花"),
            ("生物钟定律", "昼夜节律", "光照调节生物钟"),
            ("季节响应定律", "季节变化", "光周期指示季节"),
            ("光感受定律", "光敏色素", "光敏色素感受光周期"),
        ]
    }

    /// 光治疗定律
    pub fn phototherapy(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("光疗定律", "光治疗", "光用于治疗疾病"),
            ("蓝光疗法定律", "黄疸治疗", "蓝光治疗新生儿黄疸"),
            ("UV疗法定律", "紫外线治疗", "UV治疗皮肤病"),
            ("激光疗法定律", "激光治疗", "激光用于手术和治疗"),
            ("光动力疗法定律", "光敏治疗", "光敏剂加光治疗肿瘤"),
            ("红光疗法定律", "促进愈合", "红光促进伤口愈合"),
            ("光调节定律", "调节生物", "光调节生物功能"),
        ]
    }

    /// 光损伤定律
    pub fn photodamage(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("UV损伤定律", "紫外线损伤", "UV损伤DNA和皮肤"),
            ("光老化定律", "皮肤老化", "光导致皮肤老化"),
            ("光毒性定律", "光敏毒性", "光敏物质光毒性"),
            ("视网膜损伤定律", "光损伤眼", "强光损伤视网膜"),
            ("DNA损伤定律", "光致突变", "UV导致DNA突变"),
            ("氧化应激定律", "光致氧化", "光诱导氧化应激"),
            ("保护机制定律", "光保护", "生物有光保护机制"),
        ]
    }

    /// 生物成像定律
    pub fn bioimaging(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("光学成像定律", "显微镜成像", "光学显微镜成像"),
            ("荧光成像定律", "荧光标记", "荧光标记成像"),
            ("共聚焦成像定律", "三维成像", "共聚焦三维成像"),
            ("活体成像定律", "活体观察", "活体光学成像"),
            ("内窥镜定律", "体内观察", "内窥镜体内观察"),
            ("OCT成像定律", "断层成像", "光学断层成像"),
            ("光声成像定律", "光声结合", "光声结合成像"),
        ]
    }

    /// 颜色感知定律
    pub fn color_perception(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("三色理论定律", "三种视锥", "三种视锥感知颜色"),
            ("对立过程定律", "对立编码", "颜色对立编码"),
            ("颜色空间定律", "颜色表征", "颜色空间表征颜色"),
            ("颜色恒常定律", "颜色恒定", "不同光照下颜色恒定"),
            ("颜色适应定律", "颜色适应", "适应不同光照颜色"),
            ("色盲定律", "颜色缺陷", "某些颜色无法区分"),
            ("颜色视觉定律", "动物差异", "动物颜色视觉差异"),
        ]
    }

    /// 生物光学应用定律
    pub fn biooptics_applications(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("人工视觉定律", "视觉修复", "人工视觉修复失明"),
            ("生物标记定律", "荧光标记", "荧光标记研究生物"),
            ("光遗传定律", "光控细胞", "光控制细胞活动"),
            ("光学诊断定律", "光诊断", "光用于诊断疾病"),
            ("光学传感定律", "生物传感", "光学生物传感器"),
            ("显示定律", "生物显示", "生物光学显示"),
            ("光学仿生定律", "仿生光学", "仿生光学器件"),
        ]
    }
}

impl Default for BioopticsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BioopticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("biooptics")
    }

    fn explain(&self) -> String {
        format!(
            "【生物光学规则】\n\n\
            生物光学研究生物体的光学现象，是视觉科学和光学医学的基础。\n\n\
            视觉系统:\n{}\n\n\
            光感受:\n{}\n\n\
            生物发光:\n{}\n\n\
            光合作用:\n{}\n\n\
            光周期:\n{}\n\n\
            光治疗:\n{}\n\n\
            光损伤:\n{}\n\n\
            生物成像:\n{}\n\n\
            颜色感知:\n{}\n\n\
            生物光学应用:\n{}",
            self.visual_system()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.photoreception()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bioluminescence()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.photosynthesis()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.photoperiod()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.phototherapy()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.photodamage()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bioimaging()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.color_perception()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.biooptics_applications()
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
    fn test_biooptics_rules() {
        let rules = BioopticsRules::new();
        assert_eq!(rules.visual_system().len(), 7);
        assert_eq!(rules.photoreception().len(), 7);
        assert_eq!(rules.bioluminescence().len(), 7);
        assert_eq!(rules.photosynthesis().len(), 7);
        assert_eq!(rules.photoperiod().len(), 7);
        assert_eq!(rules.phototherapy().len(), 7);
        assert_eq!(rules.photodamage().len(), 7);
        assert_eq!(rules.bioimaging().len(), 7);
        assert_eq!(rules.color_perception().len(), 7);
        assert_eq!(rules.biooptics_applications().len(), 7);
    }

    #[test]
    fn test_biooptics_metadata() {
        let rules = BioopticsRules::new();
        assert_eq!(rules.metadata().name, "生物光学规则");
    }
}