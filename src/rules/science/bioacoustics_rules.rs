//! 生物声学规则
//!
//! 生物体声学现象和原理，包括听觉系统、生物发声、
//! 声通信、声纳等核心概念。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 生物声学规则集合
pub struct BioacousticsRules {
    metadata: RuleMetadata,
}

impl BioacousticsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("生物声学规则", "生物体声学现象和原理")
                .with_origin("生物声学")
                .with_tags(vec!["科学".into(), "生命科学".into(), "声学".into()]),
        }
    }

    /// 听觉系统定律
    pub fn auditory_system(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("外耳定律", "声音收集", "外耳收集声音"),
            ("中耳定律", "声音传导", "中耳传导声波"),
            ("内耳定律", "声音转换", "内耳转换声信号"),
            ("耳蜗定律", "频率分析", "耳蜗分析声音频率"),
            ("听觉阈值定律", "最小声音", "最小可检测声音"),
            ("听觉范围定律", "频率范围", "可听频率范围"),
            ("听觉定位定律", "声源定位", "确定声源位置"),
        ]
    }

    /// 声音产生定律
    pub fn sound_production(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("振动定律", "发声原理", "振动产生声音"),
            ("共鸣定律", "放大声音", "共鸣腔放大声音"),
            ("发声器官定律", "特定结构", "特定器官发声"),
            ("频率定律", "振动频率", "频率决定音调"),
            ("振幅定律", "振动强度", "振幅决定响度"),
            ("谐波定律", "复合声音", "谐波构成音色"),
            ("发声效率定律", "能量效率", "发声能量效率"),
        ]
    }

    /// 动物发声定律
    pub fn animal_sound(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("鸟类发声定律", "鸣管发声", "鸟类用鸣管发声"),
            ("哺乳动物发声定律", "声带发声", "哺乳动物用声带"),
            ("昆虫发声定律", "摩擦发声", "昆虫摩擦发声"),
            ("鱼类发声定律", "多种方式", "鱼类多种发声方式"),
            ("蛙类发声定律", "鸣囊发声", "蛙类用鸣囊"),
            ("鲸类发声定律", "低频声", "鲸类发低频声"),
            ("蝙蝠发声定律", "超声波", "蝙蝠发超声波"),
        ]
    }

    /// 声通信定律
    pub fn acoustic_communication(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("求偶声定律", "吸引配偶", "发声吸引配偶"),
            ("警示声定律", "危险警示", "警示同伴危险"),
            ("领地声定律", "宣示领地", "发声宣示领地"),
            ("亲子声定律", "亲子交流", "亲子间声交流"),
            ("群内声定律", "群体协调", "群体协调交流"),
            ("种间声定律", "种间交流", "不同物种交流"),
            ("编码定律", "信息编码", "声音编码信息"),
        ]
    }

    /// 声纳定律
    pub fn sonar(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("回声定位定律", "反射声", "利用反射声定位"),
            ("蝙蝠声纳定律", "主动声纳", "蝙蝠主动声纳"),
            ("鲸类声纳定律", "低频声纳", "鲸类低频声纳"),
            ("频率定律", "声纳频率", "声纳频率特性"),
            ("距离定律", "距离计算", "时间差计算距离"),
            ("方向定律", "方向确定", "方向定位"),
            ("目标识别定律", "目标特征", "识别目标特征"),
        ]
    }

    /// 声学环境定律
    pub fn acoustic_environment(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("传播定律", "声音传播", "声音在环境中传播"),
            ("衰减定律", "声音衰减", "声音传播中衰减"),
            ("反射定律", "声音反射", "声音反射回声"),
            ("折射定律", "声音折射", "声音折射弯曲"),
            ("散射定律", "声音散射", "声音散射"),
            ("背景噪声定律", "环境噪声", "环境背景噪声"),
            ("声影定律", "声音遮挡", "障碍物遮挡声音"),
        ]
    }

    /// 声学行为定律
    pub fn acoustic_behavior(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("发声节律定律", "发声模式", "发声节律模式"),
            ("声学习定律", "学习发声", "某些动物学习发声"),
            ("声模仿定律", "模仿声音", "模仿其他声音"),
            ("声适应定律", "适应环境", "适应声学环境"),
            ("声选择定律", "进化选择", "声音的进化选择"),
            ("声竞争定律", "声竞争", "发声竞争"),
            ("声合作定律", "声合作", "发声合作"),
        ]
    }

    /// 声学测量定律
    pub fn acoustic_measurement(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("频谱分析定律", "频率分析", "声音频率分析"),
            ("声强定律", "强度测量", "声强测量"),
            ("声压定律", "压力测量", "声压测量"),
            ("时域分析定律", "时间分析", "声音时间分析"),
            ("声学记录定律", "声音记录", "记录生物声音"),
            ("声学成像定律", "声学图像", "声学成像"),
            ("声学统计定律", "统计分析", "声学统计分析"),
        ]
    }

    /// 噪声定律
    pub fn noise(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("噪声暴露定律", "噪声损伤", "噪声损伤听力"),
            ("噪声适应定律", "适应噪声", "适应噪声环境"),
            ("噪声回避定律", "回避噪声", "回避噪声区域"),
            ("噪声污染定律", "生态影响", "噪声污染生态影响"),
            ("噪声屏蔽定律", "屏蔽效应", "噪声屏蔽信号"),
            ("噪声阈值定律", "损伤阈值", "噪声损伤阈值"),
            ("噪声防护定律", "防护机制", "噪声防护机制"),
        ]
    }

    /// 生物声学应用定律
    pub fn bioacoustics_applications(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("物种识别定律", "声音识别", "声音识别物种"),
            ("种群监测定律", "声音监测", "声音监测种群"),
            ("行为研究定律", "声学研究", "声学研究行为"),
            ("生态评估定律", "声学评估", "声学评估生态"),
            ("听力测试定律", "听力评估", "听力功能测试"),
            ("助听器定律", "听力辅助", "助听器辅助听力"),
            ("声学仿生定律", "仿生应用", "仿生声学应用"),
        ]
    }
}

impl Default for BioacousticsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BioacousticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("bioacoustics")
    }

    fn explain(&self) -> String {
        format!(
            "【生物声学规则】\n\n\
            生物声学研究生物体的声学现象，是听觉科学和声学生物学的基础。\n\n\
            听觉系统:\n{}\n\n\
            声音产生:\n{}\n\n\
            动物发声:\n{}\n\n\
            声通信:\n{}\n\n\
            声纳:\n{}\n\n\
            声学环境:\n{}\n\n\
            声学行为:\n{}\n\n\
            声学测量:\n{}\n\n\
            噪声:\n{}\n\n\
            生物声学应用:\n{}",
            self.auditory_system()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.sound_production()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.animal_sound()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.acoustic_communication()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.sonar()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.acoustic_environment()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.acoustic_behavior()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.acoustic_measurement()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.noise()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bioacoustics_applications()
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
    fn test_bioacoustics_rules() {
        let rules = BioacousticsRules::new();
        assert_eq!(rules.auditory_system().len(), 7);
        assert_eq!(rules.sound_production().len(), 7);
        assert_eq!(rules.animal_sound().len(), 7);
        assert_eq!(rules.acoustic_communication().len(), 7);
        assert_eq!(rules.sonar().len(), 7);
        assert_eq!(rules.acoustic_environment().len(), 7);
        assert_eq!(rules.acoustic_behavior().len(), 7);
        assert_eq!(rules.acoustic_measurement().len(), 7);
        assert_eq!(rules.noise().len(), 7);
        assert_eq!(rules.bioacoustics_applications().len(), 7);
    }

    #[test]
    fn test_bioacoustics_metadata() {
        let rules = BioacousticsRules::new();
        assert_eq!(rules.metadata().name, "生物声学规则");
    }
}
