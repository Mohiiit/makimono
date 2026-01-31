use leptos::prelude::*;
use visualizer_types::HealthResponse;

async fn fetch_health() -> Result<HealthResponse, String> {
    let response = gloo_net::http::Request::get("http://localhost:3000/api/health")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<HealthResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[component]
fn App() -> impl IntoView {
    let health = LocalResource::new(|| fetch_health());

    view! {
        <div class="min-h-screen bg-gray-900 text-white">
            <header class="bg-gray-800 border-b border-gray-700 px-6 py-4">
                <h1 class="text-2xl font-bold">"Madara DB Visualizer"</h1>
            </header>
            <main class="p-6">
                <div class="bg-gray-800 rounded-lg p-6 max-w-md">
                    <h2 class="text-xl font-semibold mb-4">"Status"</h2>
                    <Suspense fallback=move || view! { <p class="text-gray-400">"Loading..."</p> }>
                        {move || {
                            health.get().map(|result| {
                                match &*result {
                                    Ok(h) => view! {
                                        <p class="text-green-400">
                                            "API Status: " {h.status.clone()}
                                        </p>
                                    }.into_any(),
                                    Err(e) => view! {
                                        <p class="text-red-400">
                                            "Error: " {e.clone()}
                                        </p>
                                    }.into_any(),
                                }
                            })
                        }}
                    </Suspense>
                </div>
            </main>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
