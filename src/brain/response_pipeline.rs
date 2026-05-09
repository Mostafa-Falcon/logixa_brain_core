pub struct ResponsePipeline;

impl ResponsePipeline {
    pub fn clean(answer: &str) -> String {
        answer
            .trim()
            .replace("<|im_end|>", "")
            .replace("<|end|>", "")
            .trim()
            .to_string()
    }
}
