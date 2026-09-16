//! 博物馆参观礼仪
//!
//! 博物馆美术馆参观，涵盖静默、拍照、保护展品等行为规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MuseumEtiquetteRules,
    name: "博物馆参观礼仪",
    desc: "博物馆美术馆参观，涵盖静默、拍照、保护展品等行为规范",
    origin: "国际",
    tags: ["社交", "礼仪", "博物馆", "美术馆", "参观"]
}

impl MuseumEtiquetteRules {
    /// 参观准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前了解场馆开放时间和预约要求",
            "查看当前展览主题与参观须知",
            "穿着得体，避免过于暴露或夸张",
            "避免携带大型背包或箱包入馆",
            "提前了解拍照与闪光灯规定",
            "手机调至静音，注意场馆内请勿接打",
            "零食与饮料放在包内，不在展厅进食",
            "为老人和孩子预留体力与时间",
        ]
    }

    /// 观展行为
    pub fn viewing_etiquette(&self) -> Vec<&'static str> {
        vec![
            "保持安静，轻声交谈",
            "与展品保持安全距离，不触摸展品",
            "按参观动线顺序欣赏，不逆行",
            "人多时主动礼让后排观众",
            "不跨越护栏或围绳",
            "不用手扶玻璃展柜",
            "不在展厅内奔跑打闹",
            "不长时间重咳出声影响他人",
        ]
    }

    /// 拍照规范
    pub fn photography(&self) -> Vec<&'static str> {
        vec![
            "遵守展厅拍照标识与规定",
            "标有禁止拍摄的展区不拍摄",
            "不开启闪光灯以免损伤展品",
            "不用三脚架或自拍杆拥挤",
            "拍摄他人讲解或表演先征得同意",
            "不占用展品前位置长时间摆拍",
            "不遮挡其他观众视线",
            "照片用于个人学习而不商用",
        ]
    }

    /// 讲解与导览
    pub fn guided_tour(&self) -> Vec<&'static str> {
        vec![
            "团体可提前预约讲解服务",
            "听讲时紧跟其他成员不掉队",
            "提问举手示意，不打断讲解员",
            "控制提问数量顾及他人时间",
            "不代替他人反复追问",
            "讲解过程中保持安静与专注",
            "对讲解员介绍表示感谢",
            "离队前告知随行人员去向",
        ]
    }

    /// 场馆公约
    pub fn public_space(&self) -> Vec<&'static str> {
        vec![
            "不在展厅内饮食与吸烟",
            "不随地丢弃垃圾就近分类",
            "不躺卧占用人行座椅",
            "爱护公共设施与文创空间",
            "遇到导览讲解人员避开行走",
            "高峰期注意疏散与秩序",
        ]
    }

    /// 特殊情况
    pub fn special_situations(&self) -> Vec<&'static str> {
        vec![
            "特展遵守附加参观规定",
            "临时特展不许带饮料入内",
            "大型临时展览注意人流管限",
            "对残障观众提供必要礼让",
            "紧急疏散时听从工作人员指挥",
        ]
    }
}

impl Rule for MuseumEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("museum")
    }

    fn explain(&self) -> String {
        format!(
            "【博物馆参观礼仪】\n{}",
            [
                format!(
                    "参观准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观展行为：\\n{}",
                    self.viewing_etiquette()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "拍照规范：\\n{}",
                    self.photography()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "讲解与导览：\\n{}",
                    self.guided_tour()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场馆公约：\\n{}",
                    self.public_space()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊情况：\\n{}",
                    self.special_situations()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_museumetiquetterules_basic() {
        let rules = MuseumEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "博物馆参观礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.viewing_etiquette().is_empty());
        assert!(!rules.photography().is_empty());
        assert!(!rules.guided_tour().is_empty());
        assert!(!rules.public_space().is_empty());
        assert!(!rules.special_situations().is_empty());
    }

    #[test]
    fn test_museumetiquetterules_validation() {
        let rules = MuseumEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("museum"));
    }

    #[test]
    fn test_museumetiquetterules_explain() {
        let rules = MuseumEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("参观准备"));
        assert!(e.contains("观展行为"));
        assert!(e.contains("拍照规范"));
    }
}
