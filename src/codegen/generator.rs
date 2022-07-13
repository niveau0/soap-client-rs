use super::CodeGenModel;

pub fn generate_code(model: &CodeGenModel) -> String {
    let mut output = String::new();

    for svc in &model.services {
        output.push_str(&format!("// Service: {}\n", svc.name));

        for op in &svc.operations {
            output.push_str(&format!("pub async fn {}(&self", op.name));
            if let Some(input) = &op.input {
                for part in &input.parts {
                    output.push_str(&format!(", {}: {}", part.name, "String"));
                }
            }
            output.push_str(
                ") -> Result<String, Box<dyn std::error::Error>> {\n    unimplemented!()\n}\n\n",
            );
        }
    }

    output
}
