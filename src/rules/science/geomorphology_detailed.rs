//! 地貌学详细规则
//!
//! 地貌学研究地球表面的形态特征、形成过程和演化规律，
//! 包括侵蚀地貌、堆积地貌、构造地貌和各种特殊地貌类型。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 地貌学详细规则集合
pub struct GeomorphologyDetailedRules {
    metadata: RuleMetadata,
}

impl GeomorphologyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("地貌学详细规则", "地貌学详细定律和地貌形成过程")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "地貌".into()]),
        }
    }

    /// 侵蚀地貌规则
    pub fn erosion_landform_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("河流侵蚀定律", "下蚀侧蚀", "河流向下和向两侧侵蚀河床"),
            ("溯源侵蚀定律", "向源后退", "河流向源头方向延伸侵蚀"),
            ("风力侵蚀定律", "风蚀作用", "风力吹扬磨蚀地表物质"),
            ("冰川侵蚀定律", "刨蚀作用", "冰川运动刨蚀基岩形成地貌"),
            ("波浪侵蚀定律", "海蚀作用", "波浪撞击侵蚀海岸形成海蚀地貌"),
            ("重力侵蚀定律", "重力作用", "重力导致滑坡崩塌等侵蚀"),
            ("化学侵蚀定律", "溶蚀作用", "水对可溶性岩石的化学溶蚀"),
            ("冻融侵蚀定律", "冻融循环", "冻融交替破坏岩石结构"),
        ]
    }

    /// 堆积地貌规则
    pub fn deposition_landform_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("河流堆积定律", "沉积物沉降", "流速降低导致沉积物堆积"),
            ("风力堆积定律", "风积作用", "风力减弱导致沙尘堆积"),
            ("冰川堆积定律", "冰碛沉积", "冰川消融沉积冰碛物"),
            ("波浪堆积定律", "海积作用", "波浪搬运沉积物形成海岸"),
            ("湖泊堆积定律", "湖积作用", "湖泊沉积形成湖积地貌"),
            ("洪积堆积定律", "洪积扇", "洪水在山口堆积洪积扇"),
            ("三角洲定律", "河口沉积", "河流入海沉积形成三角洲"),
            ("冲积平原定律", "平原形成", "河流长期堆积形成平原"),
        ]
    }

    /// 河流地貌规则
    pub fn fluvial_landform_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("河谷发育定律", "河谷形态", "河流侵蚀形成V形和U形河谷"),
            ("河床形态定律", "河床演变", "河床弯曲分叉形成各种形态"),
            ("阶地形成定律", "河流阶地", "河流下切形成多级阶地"),
            ("河漫滩定律", "洪水沉积", "洪水泛滥形成河漫滩"),
            ("冲沟发育定律", "沟谷系统", "降雨侵蚀形成冲沟网络"),
            ("河谷不对称定律", "河谷倾斜", "河谷两侧坡度不对称"),
            ("河流袭夺定律", "河流改道", "河流袭夺改变水系格局"),
            ("水系类型定律", "水系形态", "树枝状格状等水系类型"),
        ]
    }

    /// 冰川地貌规则
    pub fn glacial_landform_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("冰川谷定律", "U形谷", "冰川侵蚀形成U形谷"),
            ("冰斗定律", "冰斗形成", "冰川源头侵蚀形成冰斗"),
            ("角峰定律", "角峰形态", "多个冰斗侵蚀形成角峰"),
            ("刃脊定律", "刀刃状脊", "两侧冰川侵蚀形成刃脊"),
            ("冰碛地貌定律", "冰碛丘陵", "冰川消融形成冰碛地貌"),
            ("鼓丘定律", "椭圆丘陵", "冰川堆积形成鼓丘"),
            ("冰蚀湖定律", "冰蚀湖盆", "冰川侵蚀形成湖盆"),
            ("冰川遗迹定律", "冰川痕迹", "冰川活动留下的地貌痕迹"),
        ]
    }

    /// 海岸地貌规则
    pub fn coastal_landform_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("海蚀崖定律", "悬崖后退", "波浪侵蚀形成海蚀崖"),
            ("海蚀平台定律", "平台形成", "海蚀崖后退形成平台"),
            ("海滩定律", "沙滩堆积", "波浪堆积沙粒形成海滩"),
            ("沙坝定律", "沙坝形成", "波浪堆积形成沙坝沙嘴"),
            ("海岸分类定律", "海岸类型", "岩石海岸沙质海岸分类"),
            ("海岸演变定律", "海岸变迁", "海岸随时间侵蚀堆积变化"),
            ("珊瑚礁定律", "礁体生长", "珊瑚生长形成珊瑚礁海岸"),
            ("三角港定律", "喇叭河口", "强潮海岸形成三角港"),
        ]
    }

    /// 风成地貌规则
    pub fn aeolian_landform_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("沙丘定律", "沙丘形态", "风力堆积形成各种沙丘"),
            ("新月形沙丘定律", "新月形态", "单向风形成新月形沙丘"),
            ("纵向沙丘定律", "长条沙丘", "双向风形成纵向沙丘"),
            ("风蚀洼地定律", "洼地形成", "风力侵蚀形成洼地"),
            ("雅丹地貌定律", "风蚀残丘", "风力侵蚀软硬岩层形成雅丹"),
            ("沙漠定律", "沙漠分布", "干旱区形成沙漠地貌"),
            ("戈壁定律", "砾漠形成", "风力吹走细粒物质形成戈壁"),
            ("黄土定律", "黄土堆积", "风力搬运堆积形成黄土"),
        ]
    }

    /// 喀斯特地貌规则
    pub fn karst_landform_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("溶蚀定律", "化学溶解", "水溶解石灰岩形成溶蚀地貌"),
            ("溶洞定律", "洞穴形成", "地下水溶蚀形成溶洞"),
            ("石林定律", "石柱形态", "地表溶蚀形成石林"),
            ("峰丛定律", "峰丛洼地", "石灰岩区形成峰丛地貌"),
            ("天坑定律", "塌陷坑", "溶洞塌陷形成天坑"),
            ("喀斯特平原定律", "溶蚀平原", "长期溶蚀形成喀斯特平原"),
            ("喀斯特水文定律", "地下水系", "喀斯特区地下水运动规律"),
            ("喀斯特演化定律", "地貌演化", "喀斯特地貌发育阶段规律"),
        ]
    }

    /// 构造地貌规则
    pub fn tectonic_landform_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("断层地貌定律", "断崖形成", "断层活动形成断层崖"),
            ("褶皱地貌定律", "褶皱山系", "褶皱形成背斜山向斜谷"),
            ("火山地貌定律", "火山形态", "火山喷发形成火山地貌"),
            ("地震地貌定律", "地震变形", "地震形成地貌变形"),
            ("板块边界地貌定律", "边界地貌", "板块边界形成特殊地貌"),
            ("裂谷定律", "裂谷形成", "板块张裂形成裂谷"),
            ("造山带定律", "山脉形成", "板块碰撞形成造山带"),
            ("盆地定律", "盆地形成", "构造下沉形成盆地"),
        ]
    }

    /// 地貌演化理论
    pub fn evolution_theories(&self) -> Vec<&'static str> {
        vec![
            "戴维斯地貌循环论: 地貌经历幼年期壮年期老年期演化",
            "彭克地貌演化论: 坡地平行后退形成地貌演化",
            "地貌均衡论: 侵蚀基准面控制地貌发育",
            "地貌临界论: 地貌系统存在临界阈值",
            "地貌复杂响应: 地貌系统对扰动的复杂响应",
            "地貌尺度定律: 不同尺度地貌形成规律不同",
            "地貌过程定律: 地貌形成过程组合控制形态",
            "地貌时间定律: 地貌演化需要漫长地质时间",
        ]
    }

    /// 地貌研究方法
    pub fn research_methods(&self) -> Vec<&'static str> {
        vec![
            "地貌形态测量: 测量地貌形态特征和空间分布",
            "地貌过程分析: 分析地貌形成过程和机制",
            "地貌年代测定: 测定地貌形成年代和演化历史",
            "地貌模拟实验: 实验模拟地貌形成过程",
            "遥感地貌解译: 遥感影像解译地貌信息",
            "地貌GIS分析: GIS空间分析地貌数据",
            "地貌野外调查: 野外实地调查地貌特征",
            "地貌模型建立: 建立地貌形成演化模型",
        ]
    }
}

impl Default for GeomorphologyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GeomorphologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("geomorphology_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【地貌学详细规则】\n\n\
            侵蚀地貌规则:\n{}\n\n\
            堆积地貌规则:\n{}\n\n\
            河流地貌规则:\n{}\n\n\
            冰川地貌规则:\n{}\n\n\
            海岸地貌规则:\n{}\n\n\
            风成地貌规则:\n{}\n\n\
            喀斯特地貌规则:\n{}\n\n\
            构造地貌规则:\n{}\n\n\
            地貌演化理论:\n{}\n\n\
            地貌研究方法:\n{}",
            self.erosion_landform_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.deposition_landform_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.fluvial_landform_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.glacial_landform_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.coastal_landform_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aeolian_landform_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.karst_landform_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tectonic_landform_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.evolution_theories()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.research_methods()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geomorphology_detailed_rules() {
        let rules = GeomorphologyDetailedRules::new();
        assert_eq!(rules.erosion_landform_rules().len(), 8);
        assert_eq!(rules.deposition_landform_rules().len(), 8);
        assert_eq!(rules.fluvial_landform_rules().len(), 8);
        assert_eq!(rules.glacial_landform_rules().len(), 8);
        assert_eq!(rules.coastal_landform_rules().len(), 8);
        assert_eq!(rules.aeolian_landform_rules().len(), 8);
        assert_eq!(rules.karst_landform_rules().len(), 8);
        assert_eq!(rules.tectonic_landform_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_erosion_rules() {
        let rules = GeomorphologyDetailedRules::new();
        let laws = rules.erosion_landform_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("侵蚀")));
    }

    #[test]
    fn test_evolution_theories() {
        let rules = GeomorphologyDetailedRules::new();
        assert_eq!(rules.evolution_theories().len(), 8);
    }

    #[test]
    fn test_research_methods() {
        let rules = GeomorphologyDetailedRules::new();
        assert_eq!(rules.research_methods().len(), 8);
    }
}