use rust_bert::pipelines::{
    common::ModelType,
    text_generation::{TextGenerationConfig, TextGenerationModel},
};
use rust_bert::resources::LocalResource;
use std::path::PathBuf;

fn main() {
    let model_resource = Box::new(LocalResource {
        local_path: PathBuf::from("/Users/qbui/local/rust_projects/bert_test/data/rust_model.ot"),
    });
    let config_resource = Box::new(LocalResource {
        local_path: PathBuf::from("/Users/qbui/local/rust_projects/bert_test/data/config.json"),
    });
    let vocab_resource = Box::new(LocalResource {
        local_path: PathBuf::from("/Users/qbui/local/rust_projects/bert_test/data/vocab.json"),
    });
    let merges_resource = Box::new(LocalResource {
        local_path: PathBuf::from("/Users/qbui/local/rust_projects/bert_test/data/merges.txt"),
    });

    let generate_config = TextGenerationConfig {
        model_type: ModelType::GPTNeo,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource: Some(merges_resource),
        num_beams: 5,
        no_repeat_ngram_size: 2,
        max_length: Some(100),
        ..Default::default()
    };

    let model = TextGenerationModel::new(generate_config).unwrap();

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let split = line.split('/').collect::<Vec<&str>>();
        let slc = split.as_slice();
        let output = model.generate(&slc[1..], Some(slc[0]));
        for sentence in output {
            println!("{sentence}");
        }
    }
}
