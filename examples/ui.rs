use staff::{
    ui::{prelude::*, Staff},
    Natural,
};

fn app(cx: Scope) -> Element {
    render!(
        div { display: "flex", width: "100vw", height: "100vh", align_items: "center", justify_content: "center",
            Staff { 
                note { natural: Natural::F }
                note { natural: Natural::G }
                note { natural: Natural::A }
            }
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}