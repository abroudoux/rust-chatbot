use leptos::*;
use crate::model::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(prompt: Conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;

    let model = extract(|data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner()
    })
    .await.unwrap();

    use llm::KnowModel;
    let character_name = "### Asssistant";
    let user_name = "### Human";
    let persona = "A chat between a human abnd an assistant";
    let mut history = format!(
        "{character_name}: Hello - How may I help you?\n\
        {user_name}: What\'s the capital of France?\n\
        {character_name}: Paris is the capital of France.\n"
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{character_name}: {msg}\n")
        } else {
            format!("{user_name}: {msg}\n")
        };

        history.push_str(&curr_line);
    }

    let mut res = String::new();
    let mut rng = thread_rng();
    let mut buf = String::new();

    let mut session = model.start.session(Default::default());

    session.infer(
        model.as_ref(),
        &mut rng,
        &llm::InferenceRequest {
            prompt: format!("{persona}\n{history}\n{character_name}:").as_str().into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
        inference_callback(String::from(user_name), &mut buf, &mut res),
    )
    .unwrap_or_else(|e| panic!("{e}"));

    Ok(String::from(""))
}

// cfg_if! {
//     if #[cfg(feature = "ssr")] {
//         use std::convert::Infallible;
//             fn inference_callback<'a>(
//                 stop_sequence: String,
//                 buf: &'a mut String,
//             )
    
//     }
// }
