# Program Pairs

## Finding Program-Pairs

1. Initial Discovery: Manually identify several high-quality program pairs and create their metadata entries.
  - A significant use case for Rust is rewriting and improving existing CLI tools. I searched for Rust rewrites of existing C command line tools.
2. AI-Assisted Generation: Use the manually curated examples as prompts for LLMs (like ChatGPT) to suggest additional program pairs.
  - First, manually format the program-pairs into the format of the metadata schema.
  - Then, paste it into an LLM with the following prompt: "I am compiling a list of C to Rust program pairs, help me add to my list"
3. Manual Verification: Review and validate all AI-generated suggestions to ensure:
  - The programs actually exist and are maintained
  - The Rust version is genuinely a rewrite/improvement of the C version
  - The functionality and scope are comparable
  - Metadata accuracy and completeness

## Already Found Program-Pairs
