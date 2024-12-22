use leptos::{prelude::*, component, view, IntoView};
use theta_chart::coord::{Axes, Rec};

use crate::core::REM;

#[allow(non_snake_case)]
#[component]
pub fn YAxis(region: Rec, axes: Axes) -> impl IntoView {
    let vector = region.get_vector();
    let mut mark_origin_x = REM;
    let mut text_anchor = "start";

    if vector.get_x() < 0. {
        mark_origin_x *= -1.;
        text_anchor = "end";
    }

    view! {
      {#[cfg(feature = "debug")]
      {
          let path = format!(
              "M {},{} l {},{} l {},{} l {},{} Z",
              0,
              0,
              vector.get_x(),
              0,
              0,
              vector.get_y(),
              -vector.get_x(),
              0,
          );
          view! {
            <circle id="originY" cx="0" cy="0" r="3"></circle>
            <line
              x1="0"
              y1="0"
              x2=vector.get_x()
              y2=vector.get_y()
              style="stroke:#0000ff33;stroke-width:2"
            ></line>
            <path id="regionY" d=path fill="#0000ff33"></path>
          }
      }}

      // Draw y-axis
      <g class="stick" dominant-baseline="middle" text-anchor=text_anchor stroke="currentColor">
        <line x1="0" y1="0" x2="0" y2=vector.get_y()></line>
        <line x1="0" y1="0" x2=mark_origin_x y2="0"></line>

        {axes
            .sticks
            .into_iter()
            .map(|stick| {
                let dy = stick.value * vector.get_y();
                view! {
                  <line x1="0" y1=dy x2=mark_origin_x / 2. y2=dy></line>
                  <text y=dy x=mark_origin_x fill="currentColor" stroke="none">
                    {stick.label}
                  </text>
                }
            })
            .collect::<Vec<_>>()}

      </g>
    }
}
