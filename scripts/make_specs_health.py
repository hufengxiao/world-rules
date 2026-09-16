#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""批量生成 _spec_*.json 规则定义文件（health 分类）"""
import json, os

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "specs_health")
os.makedirs(OUT, exist_ok=True)

def spec(file_name, struct, name, desc, origin, tags, catval, sections):
    return {
        "cat": "health", "file": file_name, "struct": struct,
        "name": name, "desc": desc, "origin": origin,
        "tags": tags, "category": 'RuleCategory::health("%s")' % catval,
        "sections": [{"title": t, "method": m, "items": items} for (t, m, items) in sections],
    }

RULES = [
    spec("migraine_management", "MigraineManagementRules", "偏头痛管理",
         "偏头痛预防、诱因识别与发作期护理的自我管理规则", "医学",
         ["健康","头痛","偏头痛","预防"], "migraine", [
         ("诱因识别","triggers",[
            "记录饮食摄入与发作关联",
            "规律作息避免睡眠剥夺",
            "避免过劳与压力累积",
            "避免强光闪烁与强烈气味",
            "少摄入含酪胺与亚硝酸盐食物",
         ]),
         ("规律生活","routine",[
            "固定睡眠时间与时长充足",
            "规律三餐不过度空腹",
            "适度规律锻炼",
            "减少酒精与咖啡因依赖",
         ]),
         ("急性期护理","acute",[
            "发作早期及时休息于安静暗处",
            "遵医嘱在早期服用对症药物",
            "冷敷额部或太阳穴缓解",
            "避免剧烈活动加重",
         ]),
         ("就医信号","seek_care",[
            "首次剧烈头痛就医排查",
            "伴随发热颈强直立即就医",
            "突发剧痛有神经症状需急诊",
            "携带用药史给医生评估",
         ]),
         ("日常管理","daily",[
            "避免滥用止痛药预防反跳",
            "规律复诊评估发作频率",
            "学习压力放松技巧",
            "如便秘腹胀亦专注身心平衡",
         ]),
    ]),
    spec("allergy_management", "AllergyManagementRules", "过敏管理",
         "食物过敏与接触性过敏的识别、规避与应急规则", "医学",
         ["健康","过敏","免疫"], "allergy", [
            ("识别过敏原","identify",[
               "明确记录已知过敏食物",
               "阅读食品配料表",
               "留意隐形过敏原如酱料",
               "就医做规范过敏原检测",
            ]),
            ("规避措施","avoid",[
               "外出用餐主动告知过敏信息",
               "不抱侥幸尝试疑似过敏食物",
               "注意交叉污染避免共用厨具",
               "携带过敏信息卡片",
            ]),
            ("应急用药","emergency",[
               "严重过敏者随身携带抗过敏药",
               "备好肾上腺素注射剂使用者",
               "出现喉头水肿立即呼救",
               "症状加重不拖延就医",
            ]),
            ("日常管理","daily",[
               "保持居室通风减少尘螨",
               "花粉季减少外出与开窗",
               "宠物接触时注意清洗",
               "遵医嘱规范用药不自行停药",
            ]),
    ]),
    spec("hypertension_lifestyle", "HypertensionLifestyleRules", "高血压生活方式管理",
         "高血压患者控制血压的饮食、运动与作息规则", "医学",
         ["健康","血压","慢病","生活方式"], "hypertension", [
            ("饮食控盐","salt",[
            "每日摄盐量控制不超过5克",
            "少吃腌制腊味与加工食品",
            "烹调多用天然调味替代盐",
            "警惕酱料隐形盐分",
            ]),
            ("均衡饮食","diet",[
            "多摄入蔬果与全谷物",
            "控制饱和脂肪与胆固醇",
            "适量优质蛋白",
            "限制甜食与含糖饮料",
            ]),
            ("运动与体重","exercise",[
            "每周中等强度运动等时机积累",
            "减轻体重控制腰围",
            "戒烟并控制饮酒",
            "规律作息避免熬夜",
            ]),
            ("用药与监测","medication",[
            "遵医嘱规律服用降压药",
            "不自行停药或改量",
            "家庭自测固定时段记录",
            "定期复查与医生沟通调整",
            ]),
    ]),
    spec("food_safety","FoodSafetyRules","家庭食品安全",
         "家庭厨房中预防食源性疾病的选购、储存与烹调规则", "医学",
         ["健康","食品","安全","卫生","烹饪"], "food_safety", [
            ("选购","shopping",[
               "选购新鲜且在保质期内食品",
               "购买冷藏冷冻食品注意冷链",
               "查看包装完整与生产日期",
               "不买涨袋罐装与异味蛋",
            ]),
            ("储存","storage",[
               "生熟分开存放避免交叉",
               "冷藏温度低于4℃",
               "冷冻解冻按冷藏解冻方式",
               "剩菜及时放凉并当天食用",
            ]),
            ("烹调","cooking",[
               "肉类禽蛋彻底煮熟",
               "中心温度达到充分杀菌",
               "蔬菜清洗处理",
               "刀具砧板生熟分开",
            ]),
            ("卫生","hygiene",[
               "饭前便后洗手",
               "处理生肉后清洁台面",
               "不食用变色变味剩菜",
               "及时清理厨余垃圾",
            ]),
    ]),
]

for i, s in enumerate(RULES):
    fn = os.path.join(OUT, "_spec_%02d_%s.json" % (i, s["file"]))
    with open(fn, "w", encoding="utf-8") as f:
        json.dump(s, f, ensure_ascii=False, indent=2)
    print("wrote", fn)