enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => {
                TrafficLight::Green
            }
            TrafficLight::Green => {
                TrafficLight::Yellow
            }
            TrafficLight::Yellow => {
                TrafficLight::Red
            }
        }
    }

    fn light(&self) -> String {
        match self {
            TrafficLight::Red => String::from("RED"),
            TrafficLight::Green => String::from("GREEN"),
            TrafficLight::Yellow => String::from("YELLOW"),
        }
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => {
                std::f64::consts::PI * radius * radius
            }
            Shape::Rectangle(width, height) => {
                width * height
            }
            Shape::Triangle(width, height) => {
                width * height / 2.0
            } 
        }
    }
}

fn main() {
    let mut trafficlight = TrafficLight::Green;
    for epoch in 0..10 {
        println!("Epoch[{}] : Light Color is {}", epoch, trafficlight.light());
        trafficlight = trafficlight.next();
    }
    test_shape();
}

fn test_shape() {
    let shapes = vec![
        Shape::Circle(3.0f64),
        Shape::Rectangle(4.0f64, 2.5f64),
        Shape::Triangle(3.0f64, 5.0f64),
    ];

    for shape in shapes {
        println!("Shape is {}", shape.area());
    }
}
