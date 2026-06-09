//! 矿物学定律

use crate::rules::core::{format_titled_sections, Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MineralogyLaws,
    name: "矿物学定律",
    desc: "矿物学基本定律",
    origin: "地质学",
    tags: ["科学", "地质", "矿物"]
}

impl MineralogyLaws {
    /// 矿物形成定律
    pub fn formation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("结晶定律", "晶体形成", "矿物结晶过程"),
            ("生长定律", "晶体生长", "晶体生长规律"),
            ("相变定律", "矿物相变", "矿物相变过程"),
            ("溶解定律", "溶解沉淀", "溶解沉淀平衡"),
            ("交代定律", "矿物交代", "矿物交代作用"),
            ("重结晶定律", "重结晶", "矿物重结晶过程"),
            ("沉淀定律", "矿物沉淀", "矿物沉淀规律"),
        ]
    }

    /// 矿物结构定律
    pub fn structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("晶体结构定律", "内部结构", "矿物晶体结构"),
            ("晶系定律", "七大晶系", "矿物晶系分类"),
            ("对称定律", "对称性质", "晶体对称性"),
            ("解理定律", "解理特征", "矿物解理性质"),
            ("断口定律", "断口特征", "矿物断口类型"),
            ("硬度定律", "莫氏硬度", "矿物硬度等级"),
            ("密度定律", "比重特征", "矿物比重特性"),
        ]
    }

    /// 矿物分类定律
    pub fn classification_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("化学分类定律", "化学成分", "化学成分分类"),
            ("结构分类定律", "结构类型", "结构分类方法"),
            ("成因分类定律", "形成条件", "成因分类系统"),
            ("矿物族定律", "矿物族", "矿物族分类"),
            ("矿物类定律", "矿物大类", "矿物大类划分"),
            ("变种定律", "矿物变种", "矿物变种分类"),
        ]
    }

    /// 矿物鉴定定律
    pub fn identification_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("鉴定定律", "鉴定方法", "矿物鉴定技术"),
            ("光学定律", "光学性质", "光学鉴定方法"),
            ("化学定律", "化学鉴定", "化学鉴定方法"),
            ("物理定律", "物理性质", "物理性质鉴定"),
            ("X射线定律", "衍射鉴定", "X射线衍射鉴定"),
            ("显微镜定律", "显微鉴定", "显微镜鉴定方法"),
        ]
    }

    /// 矿物大类
    pub fn mineral_classes(&self) -> Vec<&'static str> {
        vec![
            "自然元素",
            "硫化物",
            "氧化物",
            "氢氧化物",
            "卤化物",
            "碳酸盐",
            "硫酸盐",
            "硅酸盐",
        ]
    }

    /// 重要矿物
    pub fn important_minerals(&self) -> Vec<&'static str> {
        vec![
            "石英",
            "长石",
            "云母",
            "方解石",
            "金刚石",
            "石墨",
            "黄铁矿",
            "赤铁矿",
        ]
    }

    /// 晶体学
    pub fn crystallography(&self) -> Vec<&'static str> {
        vec![
            "七大晶系: 立方四方正交单斜三斜六方三方",
            "布拉维格子: 14种三维空间格子类型",
            "布拉格定律: 2d sinθ=nλ是X射线衍射基本条件",
            "晶体缺陷: 点缺陷线缺陷面缺陷和体缺陷",
            "同质多象: 同种化学成分在不同条件下形成不同晶体结构",
            "类质同象: 晶体中部分离子被其他离子替代",
        ]
    }

    /// 矿物性质
    pub fn mineral_properties(&self) -> Vec<&'static str> {
        vec![
            "解理: 矿物沿特定晶面方向裂开的性质",
            "断口: 矿物不沿解理面裂开的不规则断裂面",
            "光泽: 矿物表面对光的反射能力",
            "条痕: 矿物粉末的颜色比矿物颜色更稳定",
            "比重: 矿物密度与4°C水密度之比",
            "荧光: 矿物受紫外光照射后发光的性质",
            "压电性: 某些矿物受压后产生电荷的性质",
            "热电性: 温度变化时某些矿物产生电荷",
        ]
    }

    /// 宝石学
    pub fn gemology(&self) -> Vec<&'static str> {
        vec![
            "钻石: 由碳组成的最硬天然矿物",
            "红宝石: 含铬的红色刚玉",
            "蓝宝石: 除红色外的各色刚玉",
            "祖母绿: 含铬的绿色绿柱石",
            "翡翠: 以硬玉为主的多晶质集合体",
            "光学效应: 猫眼星光变色等特殊光学现象",
            "宝石切工: 影响宝石外观和价值的重要因素",
        ]
    }
}

impl Rule for MineralogyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("mineralogy")
    }

    fn explain(&self) -> String {
        format_titled_sections(
            "矿物学定律",
            &[
                ("形成定律", &self.formation_laws()),
                ("结构定律", &self.structure_laws()),
                ("分类定律", &self.classification_laws()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mineralogy_laws() {
        let laws = MineralogyLaws::new();
        assert!(!laws.formation_laws().is_empty());
        assert!(!laws.structure_laws().is_empty());
    }
}
