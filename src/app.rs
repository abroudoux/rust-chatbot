use leptos::*;
use leptos_meta::*;

use crate::model::conversation::Conversation;

#[component]
pub fn App() -> impl IntoView {

    provide_meta_context();

    let (conversation, set_conversation) = create_signal(Conversation::new());

    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Rust chatbot"/>

        <ChatArea conversation/>
        <TypeArea send/>
    }
}