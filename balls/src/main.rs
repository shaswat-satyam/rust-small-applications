use macroquad::{prelude::*, rand::gen_range};
use std::ops::{AddAssign,MulAssign};

#[derive(Clone, Copy)]
struct Vector {
    x: f32,
    y: f32,
}

impl Vector{
    fn new(start:[f32;2], end:[f32;2]) -> Self{
        Vector{
            x: gen_range(start[0], start[1]),
            y: gen_range(end[0], end[1]),
        }
    }
    fn distance(&self, b:Vector) -> f32 {
        ((self.x - b.x).powf(2.) + (self.y - b.y).powf(2.)).powf(0.5)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }
}

struct Particle {
    position: Vector,
    velocity: Vector,
    radius: f32,
    color: Color,
}

impl Particle{
    fn new(speed:f32,radius:[f32;2]) -> Particle{
        Particle {
            position: Vector::new(
                          [0.,screen_width()],
                          [0.,screen_height()]
            ),
            //position: Vector {x: screen_width()/2., y:screen_height()/2.},
            velocity: Vector::new(
                          [-speed,speed],
                          [-speed,speed]
            ),
            radius: gen_range(radius[0], radius[1]),
            color: Color{
                r:gen_range(0.1, 1.),
                g:gen_range(0.1, 1.),
                b:gen_range(0.1, 1.),
                a:gen_range(0.1, 1.)
            },
        }
    }

    fn render(&self){
        draw_circle(self.position.x,self.position.y, self.radius, self.color);
    }

 
    fn update_position(&mut self){
        self.position += self.velocity;
    }

    fn check_collide_with_boundary(&mut self) {
        // LEFT
        if self.position.x - self.radius <= 0. {
            self.position.x = self.radius;
            self.velocity.x *= -1.;
        }

        // RIGHT
        if self.position.x + self.radius >= screen_width() {
            self.position.x = screen_width() - self.radius;
            self.velocity.x *= -1.;
        }

        // TOP
        if self.position.y - self.radius <= 0. {
            self.position.y = self.radius;
            self.velocity.y *= -1.;
        }

        // BOTTOM
        if self.position.y + self.radius >= screen_height() {
            self.position.y = screen_height() - self.radius;
            self.velocity.y *= -1.;
        }
    }

    fn collide_with_others(&mut self, other:&mut Particle){
        if self.position.distance(other.position) <= self.radius + other.radius {
            (self.velocity, other.velocity) = (other.velocity,self.velocity);
        }
    }

}

#[macroquad::main("BasicShapes")]
async fn main() {
    let count = 5;
    let radius = 80.;

    let speed: f32 = 5.;

    let mut particles = Vec::new();

    for _i in 0..count {
        particles.push(Particle::new(speed,[10.,radius]));
    }

    'main_loop: loop {
        clear_background(Color::new(0.0, 0.0, 0.0, 0.1));
        for i in 0..particles.len() {
            particles[i].update_position();
            particles[i].check_collide_with_boundary();
            for j in i+1..particles.len() {
                let (left, right) = particles.split_at_mut(j);
                let p1 = &mut left[i];
                let p2 = &mut right[0];

                p1.collide_with_others(p2);
            }

            particles[i].render();
        }

        draw_text(
            &format!("FPS:{0}",get_fps()),
            20.0,
            20.0,
            20.0,
            WHITE,
        );

        if is_key_down(KeyCode::Escape) {
            break 'main_loop;
        }

        next_frame().await
    }
}
