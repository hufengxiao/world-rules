//! 临床诊断学定律 - 研究疾病诊断的理论和方法
//!
//! 诊断学是连接基础医学与临床医学的桥梁，为疾病诊断提供方法和依据。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: DiagnosticsRules,
    name: "临床诊断学定律",
    desc: "疾病诊断的基本方法和原则",
    origin: "医学",
    tags: ["科学", "医学", "诊断学"]
}

impl DiagnosticsRules {
    /// 病史采集
    pub fn history_taking(&self) -> Vec<&'static str> {
        vec![
            "主诉: 患者就诊最主要症状或体征及持续时间",
            "现病史: 疾病发生、发展及诊治经过",
            "既往史: 过去的健康状况和疾病史",
            "个人史: 生活习惯、职业、婚育史",
            "家族史: 家族成员健康状况和遗传病史",
            "系统回顾: 各系统功能状态全面询问",
            "起病情况: 急性或慢性起病、诱因",
            "症状演变: 症状加重或缓解因素",
            "伴随症状: 与主要症状同时出现的其他症状",
            "阴性症状: 未出现但有鉴别意义的症状",
        ]
    }

    /// 体格检查基本方法
    pub fn physical_examination(&self) -> Vec<&'static str> {
        vec![
            "视诊: 用眼观察患者全身或局部表现",
            "触诊: 用手触摸感知局部病变特征",
            "叩诊: 用手指叩击身体某部发出声音判断病变",
            "听诊: 用耳或听诊器听取体内声音",
            "嗅诊: 用嗅觉辨别患者气味",
            "浅部触诊: 用手指轻压感知浅表病变",
            "深部触诊: 用力按压感知深部病变",
            "叩诊音: 清音、浊音、实音、鼓音、过清音",
            "听诊部位: 各瓣膜听诊区、肺部听诊区",
            "检查顺序: 头颈胸腹脊柱四肢神经",
        ]
    }

    /// 一般检查
    pub fn general_examination(&self) -> Vec<&'static str> {
        vec![
            "生命体征: 体温、脉搏、呼吸、血压",
            "发育体型: 高矮胖瘦、发育是否正常",
            "营养状态: 良好、中等、不良、肥胖",
            "意识状态: 清醒、嗜睡、昏睡、昏迷",
            "精神状态: 情绪、认知、行为表现",
            "面容表情: 急性病容、慢性病容、贫血面容",
            "体位姿势: 自主体位、被动体位、强迫体位",
            "步态: 正常步态、异常步态如跛行",
            "皮肤黏膜: 颜色、弹性、皮疹、出血点",
            "淋巴结: 浅表淋巴结有无肿大",
        ]
    }

    /// 头颈部检查
    pub fn head_neck_examination(&self) -> Vec<&'static str> {
        vec![
            "头颅: 大小、形状、压痛、包块",
            "眼: 眼睑、结膜、巩膜、瞳孔、眼球运动",
            "瞳孔反射: 直接对光反射、间接对光反射",
            "耳: 外耳道、鼓膜、听力",
            "鼻: 外形、鼻腔、鼻窦压痛",
            "口: 口唇、牙齿、牙龈、口腔黏膜、扁桃体",
            "咽部: 黏膜颜色、有无充血肿胀",
            "颈部: 外形、活动度、有无抵抗",
            "甲状腺: 大小、质地、有无结节压痛",
            "颈部血管: 颈静脉充盈、颈动脉搏动",
        ]
    }

    /// 胸部检查
    pub fn chest_examination(&self) -> Vec<&'static str> {
        vec![
            "胸廓: 形状、对称性、畸形",
            "呼吸运动: 呼吸频率、节律、深度",
            "触觉语颤: 双侧对比有无增强或减弱",
            "胸廓扩张度: 呼吸时胸廓活动度",
            "肺部叩诊: 肺下界、肺下界移动度",
            "肺部听诊: 呼吸音、啰音、胸膜摩擦音",
            "心脏视诊: 心前区隆起、心尖搏动",
            "心脏触诊: 心尖搏动位置、震颤、心包摩擦感",
            "心脏叩诊: 心界大小形状",
            "心脏听诊: 心音、杂音、心律、额外心音",
        ]
    }

    /// 腹部检查
    pub fn abdominal_examination(&self) -> Vec<&'static str> {
        vec![
            "腹部视诊: 外形、呼吸运动、腹壁静脉、胃肠型",
            "腹部触诊: 腹壁紧张度、压痛、反跳痛、包块",
            "肝脏触诊: 大小、质地、表面、压痛",
            "脾脏触诊: 大小、质地、切迹",
            "胆囊触诊: 墨菲征、库瓦西耶征",
            "腹部叩诊: 移动性浊音、肝界、脾界",
            "腹部听诊: 肠鸣音、血管杂音、振水音",
            "腹膜刺激征: 压痛、反跳痛、肌紧张",
            "腹水征: 移动性浊音、液波震颤",
            "疝: 腹股沟疝、脐疝、切口疝",
        ]
    }

    /// 神经系统检查
    pub fn neurological_examination(&self) -> Vec<&'static str> {
        vec![
            "意识状态: 格拉斯哥昏迷评分",
            "瞳孔: 大小、形状、对光反射",
            "脑神经: 12对脑神经功能检查",
            "运动系统: 肌力、肌张力、共济运动",
            "感觉系统: 浅感觉、深感觉、复合感觉",
            "反射: 生理反射、病理反射",
            "脑膜刺激征: 颈强直、克尼格征、布鲁津斯基征",
            "病理反射: 巴宾斯基征、查多克征等",
            "肌力分级: 0-5级肌力评定标准",
            "感觉检查: 痛觉、温觉、触觉、位置觉",
        ]
    }

    /// 实验室检查
    pub fn laboratory_examination(&self) -> Vec<&'static str> {
        vec![
            "血常规: 红细胞、白细胞、血小板计数及分类",
            "尿常规: 尿蛋白、尿糖、尿沉渣镜检",
            "便常规: 粪便性状、潜血试验",
            "肝功能: 转氨酶、胆红素、白蛋白",
            "肾功能: 血肌酐、尿素氮、肾小球滤过率",
            "电解质: 钾、钠、氯、钙、磷、镁",
            "血糖: 空腹血糖、餐后血糖、糖化血红蛋白",
            "血脂: 胆固醇、甘油三酯、高密度脂蛋白",
            "心肌标志物: 肌钙蛋白、肌酸激酶同工酶",
            "凝血功能: 凝血酶原时间、活化部分凝血活酶时间",
        ]
    }

    /// 心电图诊断
    pub fn ecg_diagnosis(&self) -> Vec<&'static str> {
        vec![
            "正常心电图: P波、QRS波群、T波形态时限",
            "心电轴: 正常、左偏、右偏判断标准",
            "心律失常: 窦性心律失常、期前收缩、房颤、室颤",
            "心肌缺血: ST段压低、T波倒置",
            "心肌梗死: 病理性Q波、ST段抬高、T波演变",
            "心室肥大: 左室肥大、右室肥大心电图表现",
            "束支传导阻滞: 完全性、不完全性束支阻滞",
            "电解质紊乱: 高钾、低钾心电图表现",
            "药物影响: 洋地黄效应、洋地黄中毒",
            "预激综合征: WPW综合征心电图特点",
        ]
    }

    /// 影像学诊断
    pub fn imaging_diagnosis(&self) -> Vec<&'static str> {
        vec![
            "X线检查: 透视、摄片、造影检查适应症",
            "CT检查: 平扫、增强扫描、三维重建",
            "MRI检查: T1加权、T2加权、增强扫描",
            "超声检查: B超、彩色多普勒、三维超声",
            "核医学检查: PET-CT、SPECT显像",
            "X线对比剂: 碘对比剂、钡剂使用原则",
            "CT值: 不同组织CT值范围",
            "MRI信号: T1高信号、T2高信号意义",
            "造影增强: 时间-密度曲线、强化模式",
            "辐射防护: 放射检查防护原则",
        ]
    }
}

impl Rule for DiagnosticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("diagnostics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "临床诊断学定律",
            &[
                ("病史采集", &self.history_taking()),
                ("体格检查基本方法", &self.physical_examination()),
                ("一般检查", &self.general_examination()),
                ("头颈部检查", &self.head_neck_examination()),
                ("胸部检查", &self.chest_examination()),
                ("腹部检查", &self.abdominal_examination()),
                ("神经系统检查", &self.neurological_examination()),
                ("实验室检查", &self.laboratory_examination()),
                ("心电图诊断", &self.ecg_diagnosis()),
                ("影像学诊断", &self.imaging_diagnosis()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostics_rules() {
        let rules = DiagnosticsRules::new();
        assert!(!rules.history_taking().is_empty());
        assert!(!rules.physical_examination().is_empty());
        assert!(!rules.laboratory_examination().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_diagnostics_metadata() {
        let rules = DiagnosticsRules::new();
        assert_eq!(rules.metadata().name, "临床诊断学定律");
    }
}
