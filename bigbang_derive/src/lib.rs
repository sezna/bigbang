
extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use crate::proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(AsEntity)]
pub fn derive_as_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // type name
    let name = &input.ident;

    // generics
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics crate::AsEntity for #name #ty_generics #where_clause {
            fn as_entity(&self) -> Entity {
                return self.clone();
            }
            fn respond(&self, simulation_result: SimulationResult<Self>, time_step: f64) -> Self {
                let mut vx = self.vx;
                let mut vy = self.vy;
                let mut vz = self.vz;
                let (mut ax, mut ay, mut az) = simulation_result.gravitational_acceleration;
                for other in &simulation_result.collisions {
                    let (collision_ax, collision_ay, collision_az) = soft_body(self, other, 50f64);
                    ax += collision_ax;
                    ay += collision_ay;
                    az += collision_az;
                }
                vx += ax * time_step;
                vy += ay * time_step;
                vz += az * time_step;

                Entity {
                    vx,
                    vy,
                    vz,
                    x: self.x + (vx * time_step),
                    y: self.y + (vy * time_step),
                    z: self.z + (vz * time_step),
                    radius: self.radius,
                    mass: self.mass,
                }
            }
        }
    };

    TokenStream::from(expanded)
}