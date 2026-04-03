//! 餐桌礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 餐桌礼仪地区变体
#[derive(Debug, Clone)]
pub enum DiningCulture {
    /// 中国餐桌礼仪
    Chinese,
    /// 西方餐桌礼仪
    Western,
    /// 日本餐桌礼仪
    Japanese,
    /// 韩国餐桌礼仪
    Korean,
}

/// 餐桌礼仪规则
pub struct DiningEtiquette {
    metadata: RuleMetadata,
    culture: DiningCulture,
}

impl DiningEtiquette {
    pub fn new(culture: DiningCulture) -> Self {
        Self {
            metadata: RuleMetadata::new(
                format!("{}餐桌礼仪", Self::culture_name(&culture)),
                "餐桌礼仪规范"
            )
            .with_origin(Self::culture_name(&culture)),
            culture,
        }
    }

    fn culture_name(culture: &DiningCulture) -> &'static str {
        match culture {
            DiningCulture::Chinese => "中国",
            DiningCulture::Western => "西方",
            DiningCulture::Japanese => "日本",
            DiningCulture::Korean => "韩国",
        }
    }

    pub fn culture(&self) -> &DiningCulture {
        &self.culture
    }

    /// 获取用餐顺序
    pub fn dining_order(&self) -> Vec<&'static str> {
        match self.culture {
            DiningCulture::Chinese => vec![
                "长者先动筷",
                "主人招呼开席",
                "先凉菜后热菜",
                "主食最后",
            ],
            DiningCulture::Western => vec![
                " Appetizers (开胃菜)",
                "Soup (汤)",
                "Main Course (主菜)",
                "Dessert (甜点)",
            ],
            DiningCulture::Japanese => vec![
                "先说「いただきます」",
                "按顺序享用",
                "最后说「ごちそうさま」",
            ],
            DiningCulture::Korean => vec![
                "长辈先动筷",
                "勺子用于汤饭",
                "筷子用于夹菜",
            ],
        }
    }

    /// 获取禁忌事项
    pub fn taboos(&self) -> Vec<&'static str> {
        match self.culture {
            DiningCulture::Chinese => vec![
                "不能用筷子指人",
                "不能把筷子插在饭里",
                "不能翻菜",
                "不能敲碗",
                "不能剩饭",
            ],
            DiningCulture::Western => vec![
                "肘部不能放桌上",
                "不能大声咀嚼",
                "不能伸手越过他人取菜",
                "餐巾不能塞在领口",
            ],
            DiningCulture::Japanese => vec![
                "不能传递食物",
                "不能把筷子放在碗上",
                "不能混合芥末和酱油",
            ],
            DiningCulture::Korean => vec![
                "不能从长辈面前经过",
                "不能提前离席",
                "不能把碗端起来吃",
            ],
        }
    }
}

impl Rule for DiningEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let order = self.dining_order();
        let taboos = self.taboos();

        format!(
            "【{}餐桌礼仪】\n\n\
            用餐顺序:\n{}\n\n\
            禁忌事项:\n{}\n",
            Self::culture_name(&self.culture),
            order.iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            taboos.iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_dining() {
        let etiquette = DiningEtiquette::new(DiningCulture::Chinese);
        assert!(etiquette.taboos().contains(&"不能用筷子指人"));
    }
}