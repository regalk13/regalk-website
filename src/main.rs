
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{Router, routing::get};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use regalk::app::*;
    use std::{convert::Infallible};
    use http::Response;
    use axum::body::Body;
    use axum::response::Redirect;
    use axum::extract::Path;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/rss.xml", get(|| async {
            const RSS_CONTENT: &str = include_str!("../rss.xml");
            let body = Body::from(format!("{}", RSS_CONTENT));
            let res = Response::new(body);
            Ok::<_, Infallible>(res)
        }))
        .route(
            "/.well-known/matrix/:path",
            get(|Path(path): Path<String>| async move {
                Redirect::permanent(&format!("https://matrix.regalk.dev/.well-known/matrix/{}", path))
            }),
        )
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
