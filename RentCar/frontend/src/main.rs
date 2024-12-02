use yew::prelude::*;

struct App {
    link: ComponentLink<Self>,
    cars: Vec<String>,
}

enum Msg {
    FetchCars,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            cars: vec![],
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchCars => {
                // Simulasi pengambilan data mobil
                self.cars = vec![
                    "Car: Toyota Avanza - Jakarta - Rp 300,000/hari".into(),
                    "Car: Honda Jazz - Bandung - Rp 400,000/hari".into(),
                ];
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "RentCar" }</h1>
                <div>
                    <h2>{ "Daftar Mobil" }</h2>
                    <button onclick={ctx.link().callback(|_| Msg::FetchCars)}>{ "Lihat Mobil" }</button>
                    <ul>
                        { for self.cars.iter().map(|car| html! { <li>{ car }</li> }) }
                    </ul>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<App>::new().mount_to_body();
}
