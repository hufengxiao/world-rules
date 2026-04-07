//! 葬礼礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 葬礼文化类型
#[derive(Debug, Clone)]
pub enum FuneralCulture {
    Chinese,
    Western,
    Japanese,
}

/// 葬礼礼仪规则
pub struct FuneralEtiquette {
    metadata: RuleMetadata,
    culture: FuneralCulture,
}

impl FuneralEtiquette {
    pub fn new(culture: FuneralCulture) -> Self {
        Self {
            metadata: RuleMetadata::new(
                format!("{}葬礼礼仪", Self::culture_name(&culture)),
                "葬礼礼仪规范"
            )
            .with_origin(Self::culture_name(&culture)),
            culture,
        }
    }

    fn culture_name(culture: &FuneralCulture) -> &'static str {
        match culture {
            FuneralCulture::Chinese => "中式",
            FuneralCulture::Western => "西式",
            FuneralCulture::Japanese => "日式",
        }
    }

    /// 葬礼流程
    pub fn funeral_process(&self) -> Vec<&'static str> {
        match self.culture {
            FuneralCulture::Chinese => vec![
                "报丧",
                "入殓",
                "守灵",
                "告别仪式",
                "出殡",
                "火化/安葬",
                "头七祭奠",
            ],
            FuneralCulture::Western => vec![
                "通知亲友",
                "遗体告别",
                "宗教仪式",
                "墓地仪式",
                "追思会",
            ],
            FuneralCulture::Japanese => vec![
                "临终",
                "通夜",
                "告别式",
                "火化",
                "纳骨",
                "法事",
            ],
        }
    }

    /// 参加者礼仪
    pub fn attendee_etiquette(&self) -> Vec<&'static str> {
        match self.culture {
            FuneralCulture::Chinese => vec![
                "穿素色衣服 (黑/白/灰)",
                "送礼金 (白包)",
                "上香/鞠躬",
                "不穿鲜艳颜色",
                "不说喜庆话语",
            ],
            FuneralCulture::Western => vec![
                "穿黑色正装",
                "送花圈或鲜花",
                "参加宗教仪式",
                "默哀",
                "慰问家属",
            ],
            FuneralCulture::Japanese => vec![
                "穿黑色丧服",
                "准备香典 (礼金)",
                "烧香",
                "接受丧主回礼",
            ],
        }
    }

    /// 礼金指南
    pub fn gift_guidelines(&self) -> Vec<&'static str> {
        match self.culture {
            FuneralCulture::Chinese => vec![
                "一般朋友: 200-500元",
                "亲戚: 500-2000元",
                "使用白色信封",
                "金额避免双数",
                "写上姓名和奠仪字样",
            ],
            FuneralCulture::Western => vec![
                "可捐款代替礼金",
                "送鲜花或花圈",
                "金额因关系而异",
            ],
            FuneralCulture::Japanese => vec![
                "香典: 3000-10000日元",
                "新钞",
                "专用香典袋",
            ],
        }
    }

    /// 禁忌事项
    pub fn taboos(&self) -> Vec<&'static str> {
        match self.culture {
            FuneralCulture::Chinese => vec![
                "不说再见 (谐音不吉利)",
                "不穿红戴绿",
                "不拍照合影",
                "不嬉笑打闹",
                "不谈论喜庆话题",
            ],
            FuneralCulture::Western => vec![
                "不迟到",
                "手机静音",
                "仪式中不交谈",
            ],
            FuneralCulture::Japanese => vec![
                "不说再见",
                "不穿华丽服装",
                "遵守礼仪规范",
            ],
        }
    }
}

impl Rule for FuneralEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("funeral")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}葬礼礼仪】\n\n\
            葬礼流程:\n{}\n\n\
            参加者礼仪:\n{}\n\n\
            礼金指南:\n{}\n\n\
            禁忌事项:\n{}\n",
            Self::culture_name(&self.culture),
            self.funeral_process().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.attendee_etiquette().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.gift_guidelines().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.taboos().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_funeral() {
        let funeral = FuneralEtiquette::new(FuneralCulture::Chinese);
        assert!(funeral.funeral_process().contains(&"告别仪式"));
    }
}