//! 茶道礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 茶道流派
#[derive(Debug, Clone)]
pub enum TeaCulture {
    /// 中国茶道
    Chinese,
    /// 日本茶道
    Japanese,
    /// 英式下午茶
    English,
}

/// 茶道礼仪规则
pub struct TeaEtiquette {
    metadata: RuleMetadata,
    culture: TeaCulture,
}

impl TeaEtiquette {
    pub fn new(culture: TeaCulture) -> Self {
        Self {
            metadata: RuleMetadata::new(
                format!("{}茶道礼仪", Self::culture_name(&culture)),
                "茶道礼仪规范"
            )
            .with_origin(Self::culture_name(&culture)),
            culture,
        }
    }

    fn culture_name(culture: &TeaCulture) -> &'static str {
        match culture {
            TeaCulture::Chinese => "中国",
            TeaCulture::Japanese => "日本",
            TeaCulture::English => "英式",
        }
    }

    /// 泡茶步骤
    pub fn brewing_steps(&self) -> Vec<&'static str> {
        match self.culture {
            TeaCulture::Chinese => vec![
                "温杯: 用热水温热茶杯",
                "置茶: 放入适量茶叶",
                "冲泡: 注入热水",
                "浸泡: 等待适当时间",
                "出汤: 将茶汤倒入杯中",
                "品饮: 分三口品茶",
            ],
            TeaCulture::Japanese => vec![
                "准备器具",
                "擦拭茶碗",
                "放入抹茶粉",
                "注入热水",
                "搅拌 (用茶筅)",
                "奉茶",
            ],
            TeaCulture::English => vec![
                "煮水至沸腾",
                "放入茶包或茶叶",
                "浸泡3-5分钟",
                "取出茶包",
                "加入牛奶和糖",
                "搭配点心享用",
            ],
        }
    }

    /// 品茶礼仪
    pub fn drinking_etiquette(&self) -> Vec<&'static str> {
        match self.culture {
            TeaCulture::Chinese => vec![
                "先闻香再品味",
                "小口慢饮",
                "不发出声响",
                "续杯时表示感谢",
                "客人应双手接茶",
                "主人最后饮茶",
            ],
            TeaCulture::Japanese => vec![
                "双手持碗",
                "先旋转茶碗避开正面",
                "分三口饮完",
                "最后发出声音表示好喝",
                "擦拭碗边",
                "归还茶碗",
            ],
            TeaCulture::English => vec![
                "搅拌茶时不要碰到杯壁",
                "搅拌后取出茶匙",
                "茶匙放在杯碟上",
                "端杯时手指不要穿过杯把",
                "小口品尝",
                "可以搭配饼干或三明治",
            ],
        }
    }

    /// 器具礼仪
    pub fn utensil_etiquette(&self) -> Vec<&'static str> {
        match self.culture {
            TeaCulture::Chinese => vec![
                "茶壶嘴不能对着客人",
                "茶杯不能倒满 (七分满)",
                "茶托不能空置",
                "客人杯空时要主动续茶",
            ],
            TeaCulture::Japanese => vec![
                "欣赏茶碗的美",
                "不要随意触碰茶具",
                "归还时要表示感谢",
            ],
            TeaCulture::English => vec![
                "杯碟始终放在桌上",
                "不用茶杯托着杯碟",
                "茶匙不要留在杯中",
            ],
        }
    }
}

impl Rule for TeaEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("tea")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}茶道礼仪】\n\n\
            泡茶步骤:\n{}\n\n\
            品茶礼仪:\n{}\n\n\
            器具礼仪:\n{}\n",
            Self::culture_name(&self.culture),
            self.brewing_steps().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.drinking_etiquette().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.utensil_etiquette().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_tea() {
        let tea = TeaEtiquette::new(TeaCulture::Chinese);
        assert!(tea.brewing_steps().contains(&"温杯: 用热水温热茶杯"));
    }
}