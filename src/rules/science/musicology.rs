//! 音乐学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 音乐学定律集合
pub struct MusicologyLaws {
    metadata: RuleMetadata,
}

impl MusicologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("音乐学定律", "音乐学基本定律")
                .with_origin("艺术科学")
                .with_tags(vec!["科学".into(), "音乐".into()]),
        }
    }

    /// 音乐理论定律
    pub fn theory_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("音高定律", "频率决定", "音高由频率决定"),
            ("音程定律", "音高关系", "音程音高差"),
            ("和弦定律", "音组合", "和弦音组合规则"),
            ("调性定律", "调式系统", "调性调式规则"),
            ("节拍定律", "时间单位", "节拍时间单位"),
            ("节奏定律", "时值排列", "节奏时值规律"),
            ("旋律定律", "音线", "旋律音线规律"),
            ("和声定律", "声部配合", "和声声部规则"),
        ]
    }

    /// 音乐物理定律
    pub fn physics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("声音传播定律", "介质传播", "声音在介质中传播"),
            ("谐波定律", "泛音序列", "谐波泛音规律"),
            ("共振定律", "共振现象", "共振放大声音"),
            ("音色定律", "波形特征", "音色波形特征"),
            ("声学定律", "声学原理", "声学基本原理"),
            ("音强定律", "振幅决定", "音强由振幅决定"),
        ]
    }

    /// 音乐心理定律
    pub fn psychology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("音乐感知定律", "听觉感知", "音乐听觉感知"),
            ("音乐情感定律", "情感引发", "音乐引发情感"),
            ("音乐记忆定律", "记忆存储", "音乐记忆规律"),
            ("音乐认知定律", "认知过程", "音乐认知过程"),
            ("音乐联想定律", "联想效应", "音乐联想效应"),
            ("音乐期待定律", "期待效应", "音乐期待心理"),
        ]
    }

    /// 音乐创作定律
    pub fn composition_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("作曲定律", "创作技法", "作曲技法规律"),
            ("配器定律", "乐器编排", "配器编排规则"),
            ("编曲定律", "结构安排", "编曲结构规律"),
            ("主题定律", "主题发展", "主题发展技法"),
            ("变奏定律", "变化技法", "变奏创作技法"),
            ("对位定律", "声部对位", "对位创作技法"),
        ]
    }

    /// 音乐体裁
    pub fn music_genres(&self) -> Vec<&'static str> {
        vec![
            "交响乐",
            "协奏曲",
            "奏鸣曲",
            "歌剧",
            "合唱",
            "室内乐",
            "流行音乐",
            "民族音乐",
        ]
    }

    /// 音乐要素
    pub fn elements(&self) -> Vec<&'static str> {
        vec![
            "音高", "音长", "音强", "音色", "节奏", "旋律", "和声", "织体",
        ]
    }

    /// 音乐理论
    pub fn music_theory(&self) -> Vec<&'static str> {
        vec![
            "十二平均律: 将八度等分为12个半音的律制",
            "五声音阶: 宫商角徵羽五个音构成的音阶",
            "和声学: 研究和弦连接规律的学科",
            "对位法: 多个独立声部协调结合的技术",
            "曲式学: 音乐作品的结构形式",
            "配器法: 为不同乐器分配声部的技术",
        ]
    }

    /// 音乐心理学
    pub fn music_psychology(&self) -> Vec<&'static str> {
        vec![
            "音高感知: 人耳对频率的主观感知",
            "节奏感知: 对音乐时间模式的认知加工",
            "音乐期待: 听者对音乐走向的预期",
            "音乐情感: 音乐引发的主观情感体验",
            "绝对音感: 无需参考即可识别音高的能力",
            "音乐记忆: 对旋律节奏和声的编码和提取",
        ]
    }

    /// 民族音乐学
    pub fn ethnomusicology(&self) -> Vec<&'static str> {
        vec![
            "世界音乐: 非西方艺术音乐传统的音乐文化",
            "音乐与仪式: 音乐在宗教和社会仪式中的功能",
            "口传传统: 无文字社会中音乐的传承方式",
            "音乐全球化: 不同音乐传统相互影响融合",
            "乐器分类: 体鸣膜鸣弦鸣气鸣和电鸣五大类",
            "音乐认同: 音乐在构建群体认同中的作用",
        ]
    }
}

impl Default for MusicologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MusicologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("musicology")
    }

    fn explain(&self) -> String {
        format!(
            "【音乐学定律】\n\n理论定律:\n{}\n\n物理定律:\n{}\n\n心理定律:\n{}\n",
            self.theory_laws()
                .iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.physics_laws()
                .iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.psychology_laws()
                .iter()
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
    fn test_musicology_laws() {
        let laws = MusicologyLaws::new();
        assert!(!laws.theory_laws().is_empty());
        assert!(!laws.physics_laws().is_empty());
    }
}
