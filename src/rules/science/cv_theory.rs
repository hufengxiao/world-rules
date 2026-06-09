//! 计算机视觉理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CvTheoryRules,
    name: "计算机视觉理论",
    desc: "计算机视觉理论定律",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("cv_theory"),
    sections: [("基础", section_0), ("应用", section_1)]
}

impl CvTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["卷积特征提取", "目标检测", "图像分割"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["人脸识别", "自动驾驶视觉", "医学影像"]
    }
}
