use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;
use yew_router::prelude::*; // 0.3.1

mod storage;
use storage::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
}

enum Msg {
    AddOne,
    RemoveOne(usize),
    update(String),
    Toggle(usize),
    Edittitle(usize, String),
    EdittitleDone(usize),
    LoadTodos(Todos),
}
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct Todo {
    id: i32,
    title: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct Todos {
    list: Vec<Todo>,
}
struct Model {
    list: Todos,
    value: String,
    edit: Vec<bool>,
    loaded: bool,
}

async fn run() -> Result<Vec<Todo>, Vec<Todo>> {
    let token = "Token ".to_string() + &getToken();

    log::info!("{}", token);

    let client = reqwest::Client::new();
    let resp_value = client
        .get("https://todo-app-csoc.herokuapp.com/todo/")
        .header("Authorization", &token)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let test: Vec<Todo> = serde_json::from_str(&resp_value).unwrap();
    Ok(test)
}

/*impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            list: Todos::default(),
            value: "".to_string(),
            edit: Vec::new(),
            loaded: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadTodos(x) => {
                self.list = x;
                self.loaded = true;
                true
            }

            Msg::AddOne => {
                let x = self.value.clone();
                self.value = "".to_string();
                if x != "" {
                    self.list.list.push(Todo { id: 0, title: x });
                    self.edit.push(false);
                }
                true
            }
            Msg::RemoveOne(x) => {
                self.list.list.remove(x);
                true
            }
            Msg::update(x) => {
                self.value = x;
                false
            }
            Msg::Toggle(x) => {
                if self.edit[x] {
                    self.edit[x] = false;
                } else {
                    self.edit[x] = true;
                }
                true
            }
            Msg::Edittitle(x, s) => {
                self.list.list[x].title = s;
                false
            }
            Msg::EdittitleDone(x) => {
                if self.list.list[x].title != "" {
                    self.edit[x] = false;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        let render_item = |index: usize, value: &Todo| -> Html {
            let x = &value.title;
            html! {

                <li class="list-group-item d-flex justify-content-between align-items-center">
                    if self.edit[index] {
                        <input id="input-button-1" type="text" value = {x.clone()}  onchange = {link.callback(move |event:Event| Msg::Edittitle(index.clone(),event.target_unchecked_into::<InputElement>().value()))}   class="form-control todo-edit-title-input" placeholder="Edit The title"/>
                        <div id="done-button-1"  class="input-group-append">
                            <button class="btn btn-outline-success todo-update-title" type="button" onclick = {link.callback(move |_| Msg::EdittitleDone(index.clone()))}>{"Done"}</button>
                        </div>
                    }

                    else{
                        <div id="title-1" class="todo-title">
                            {x}
                        </div>
                    }

                    <span id="title-actions-1">
                        if self.edit[index] == false {
                            <button style="margin-right:5px;" type="button" class="btn btn-outline-warning" onclick = {link.callback(move |_| Msg::Toggle(index.clone()))}>
                                <img src="https://res.cloudinary.com/nishantwrp/image/upload/v1587486663/CSOC/edit.png" width="18px" height="20px"/>
                            </button>
                        }

                        <button type="button" class="btn btn-outline-danger" onclick = {link.callback(move |_| Msg::RemoveOne(index.clone()))} >
                            <img src="https://res.cloudinary.com/nishantwrp/image/upload/v1587486661/CSOC/delete.svg" width="18px" height="22px" />
                        </button>
                    </span>
                </li>
            }
        };

        html! {
            <div class = "container mt-5" style = "text-align:center">
            <h3>{"Add new Todo"}</h3>
            <div class = "row justify-content-md-center mt-3">
                <div class = "col-3">
                    <input class = "form-control" value = {self.value.clone()}  type = "text" onchange = {link.callback(|event:Event| Msg::update(event.target_unchecked_into::<InputElement>().value()))}/>
                </div>
                <div class = "col-auto ms-2">
                    <button class = "btn btn-success" onclick = {link.callback(|_| Msg::AddOne)} >{"Add"}</button>
                </div>
            </div>
            <div class = "row justify-content-md-center mt-3">
                <div class = "col-4">
                    <ul id = "list" class="list-group todo-available-titles">
                        <span class="badge badge-pill todo-available-titles-text">{"Available titles"}</span>
                        {
                            for self.list.list.iter().enumerate().map(|(index,value)| render_item(index,value))
                        }
                    </ul>
                </div>
            </div>
        </div>
        }
    }
}*/

#[derive(Deserialize)]
struct token {
    token: String,
}

#[function_component(Home)]
fn home() -> Html {
    let empty: Vec<Todo> = Vec::new();
    let list = use_state(|| vec![]);
    let edit: UseStateHandle<Vec<bool>> = use_state(|| Vec::new());
    let value = use_state(|| "".to_string());
    {
        let list = list.clone();
        use_effect_with_deps(
            move |_| {
                let list = list.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let res = run().await.unwrap();
                    list.set(res);
                });
                || ()
            },
            (),
        );
    }
    /*let edit_title = |index: usize| {
        Callback::from(move |e: Event| {
            let input: InputElement = e.target_unchecked_into();
            (*list).borrow_mut()[index].title = input.value();
        })
    };
    let edit_done = |index: usize| {
        let edit = edit.clone();
        Callback::from(move |e: MouseEvent| {
            if (*list).borrow()[index].title != "" {
                //    (*edit)[index] = false;
                //    edit.set(*edit);
            }
        })
    };

    let toggle = |index: usize| {
        let edit = edit.clone();
        Callback::from(move |e: MouseEvent| {
            //   (*edit)[index] = true;
            //   edit.set(*edit);
        })
    };*/

    let remove = |index: usize| {
        let list = list.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut temp = (*list).clone();
            temp.remove(index);
            list.set(temp);
        })
    };

    //link.callback(move |event:Event| Msg::Edittitle(index.clone(),event.target_unchecked_into::<InputElement>().value()))
    let render_item = |index: usize, value: &Todo| -> Html {
        let x = &value.title;
        html! {
            <li class="list-group-item d-flex justify-content-between align-items-center">
                <div id="title-1" class="todo-title">
                    {x}
                </div>
                <span id="title-actions-1">
                        <button style="margin-right:5px;" type="button" class="btn btn-outline-warning">
                            <img src="https://res.cloudinary.com/nishantwrp/image/upload/v1587486663/CSOC/edit.png" width="18px" height="20px"/>
                        </button>

                    <button type="button" class="btn btn-outline-danger"  onclick = {remove(index.clone())} >
                        <img src="https://res.cloudinary.com/nishantwrp/image/upload/v1587486661/CSOC/delete.svg" width="18px" height="22px" />
                    </button>
                </span>
            </li>
        }
    };

    let update = {
        let value = value.clone();
        Callback::from(move |e: Event| {
            let input: InputElement = e.target_unchecked_into();
            value.set(input.value());
        })
    };
    let add = {
        let list = list.clone();
        let value = value.clone();
        Callback::from(move |e: MouseEvent| {
            let mut temp = (*list).clone();
            temp.push(Todo {
                id: 0,
                title: (*value).clone(),
            });
            list.set(temp);
            value.set("".to_string());
        })
    };

    html! {
        <div class = "container mt-5" style = "text-align:center">
        <h3>{"Add new Todo"}</h3>
        <div class = "row justify-content-md-center mt-3">
            <div class = "col-3">
                <input class = "form-control"  type = "text" onchange = {update} value = {(*value).clone()}/>
            </div>
            <div class = "col-auto ms-2">
                <button class = "btn btn-success" onclick = {add} >{"Add"}</button>
            </div>
        </div>
        <div class = "row justify-content-md-center mt-3">
            <div class = "col-4">
                <ul id = "list" class="list-group todo-available-titles">
                    <span class="badge badge-pill todo-available-titles-text">{"Available titles"}</span>
                    {
                        for list.iter().enumerate().map(|(index,value)| render_item(index,value))
                    }
                </ul>
            </div>
        </div>
    </div>
    }
}

#[function_component(Login)]
fn login() -> Html {
    let history = use_history().unwrap();
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let onchange = {
        let username = username.clone();
        Callback::from(move |e: Event| {
            let input: InputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let onchange_password = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let input: InputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let onclick = {
        let username = (*username).clone();
        let password = (*password).clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let mut map = HashMap::new();
            map.insert("username", username.clone());
            map.insert("password", password.clone());
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = reqwest::Client::new();
                let res = client
                    .post("https://todo-app-csoc.herokuapp.com/auth/login/")
                    .json(&map)
                    .send()
                    .await
                    .unwrap()
                    .json::<token>()
                    .await
                    .unwrap();
                setToken(res.token);
                history.push(Route::Home);
            });
        })
    };

    html! {
        <>
            <nav class="navbar navbar-expand-lg navbar-dark bg-primary">
                <a class="navbar-brand ms-5" href="#">{"Todo"}</a>
                <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarTogglerDemo03"
                    aria-controls="navbarTogglerDemo03" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>


                <div class="collapse navbar-collapse" id="navbarTogglerDemo03">
                    <ul class="navbar-nav me-auto mt-2 mt-lg-0">
                    <li class="nav-item">
                        <a class="nav-link">{"Register"}</a>
                    </li>
                    <li class="nav-item active">
                        <a class="nav-link">{"Login"}</a>
                    </li>
                    </ul>
                </div>
          </nav>
          <div style="padding-left:6%; max-width:70%; padding-top:6%;">
                <div style="padding-bottom:10px">
                    <span style="color:grey;font-size:20px;">
                    {"Login"}
                    </span>
                </div>
                <div class="form-group">
                    <label>{"Username"}</label>
                    <input type="text" class="form-control" id="inputUsername" {onchange}/>
                </div>
                <div class="form-group">
                    <label>{"Password"}</label>
                    <input type="Password" class="form-control" id="inputPassword" onchange = {onchange_password}/>
                </div>
                <button class="btn btn-outline-success mt-3" type = "submit" onclick = {onclick}>{"Log In"}</button>
          </div>

        </>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            if !isAuthenticated() {
                html! {
                    <Redirect<Route> to={Route::Login}/>
                }
            } else {
                html! {
                    <Home/>
                }
            }
        }
        Route::Login => html! {
            <Login/>
        },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}
