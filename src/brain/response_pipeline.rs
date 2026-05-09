pub struct ResponsePipeline;

impl ResponsePipeline {
    pub fn clean(answer: &str) -> String {
        let cleaned = answer
            .trim()
            .replace("<|im_end|>", "")
            .replace("<|end|>", "")
            .trim()
            .to_string();

        Self::apply_gigi_style_guard(&cleaned).trim().to_string()
    }

    fn apply_gigi_style_guard(answer: &str) -> String {
        let mut output = answer.to_string();

        // Minimal deterministic guard for repeated style leaks observed in Qwen output.
        // Keep replacements small and explicit to avoid damaging technical/code answers.
        let replacements = [
            ("، يا إيش؟", "؟"),
            (" يا إيش؟", "؟"),
            ("يا إيش؟", ""),
            ("يا إيش", ""),
            ("إيش تبيه", "عايز إيه"),
            ("إيش لازم نعمل", "إيه اللي نعمله"),
            ("إيش اللي لازم نعمل", "إيه اللي لازم نعمله"),
            ("إيش اللي", "إيه اللي"),
            ("إيش", "إيه"),
            ("وش", "إيه"),
            ("شنو", "إيه"),
            ("تبيه", "عايزه"),
            ("تبغاه", "عايزه"),
            ("أبغا", "عايزة"),
            ("شلون", "إزاي"),
            ("هلا", "أهلًا"),
            ("حابينك", "حابب"),
            ("حاسينك", "حاسس"),
            ("موجوده", "موجودة"),
            ("أنا جاهز", "أنا جاهزة"),
            ("انا جاهز", "أنا جاهزة"),
            ("أنا فاهم", "أنا فاهمة"),
            ("انا فاهم", "أنا فاهمة"),
            ("أنا مستعد", "أنا مستعدة"),
            ("انا مستعد", "أنا مستعدة"),
            ("جاهزة أيه؟", "أساعدك في إيه؟"),
            ("جاهزة إيه؟", "أساعدك في إيه؟"),
            ("أنا AI", "أنا جيجي"),
            ("أنا نموذج", "أنا جيجي"),
        ];

        for (from, to) in replacements {
            output = output.replace(from, to);
        }

        output
    }
}
