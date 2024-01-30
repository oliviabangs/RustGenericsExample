use std::f32::consts::PI;
enum LinearStyle {Running, Stem, Chain}
enum CircularStyle {French, Wheel}

struct ActiveThread {
    started: bool,
    total_cm: f32,
    remaining_cm: f32,
    strand_number: u32,
}

struct LinearStitch {
    start_x: f32,
    end_x: f32,
    stitch_length: f32,
    style: LinearStyle
}

impl LinearStitch {
    fn new(start_x: f32, end_x: f32, stitch_length: f32, style: LinearStyle) -> LinearStitch {
        LinearStitch {
            start_x: start_x,
            end_x: end_x,
            stitch_length: stitch_length,
            style: style
        }
    }
}

impl Stitch for LinearStitch {
    fn display_style(&self) -> () {
        match self.style {
            LinearStyle::Chain => println!("Chain Stitch!"),
            LinearStyle::Running => println!("Running Stitch!"),
            LinearStyle::Stem => println!("Stem Stitch!")
        }
    }

    fn check_possible(&self, current_thread: &mut ActiveThread) -> bool {
        if self.start_x > self.end_x {false}
        else {
            let distance = self.end_x - self.start_x;
            if distance > current_thread.remaining_cm {false}
            else if distance < self.stitch_length {false}
            else {true}
        }
    }

    fn update_thread(&self, current_thread: &mut ActiveThread) -> () {
        let distance = self.end_x - self.start_x;
        current_thread.remaining_cm -= distance;
    }
}

struct CircularStitch {
    x: f32,
    y: f32,
    radius: f32,
    style: CircularStyle
}

impl CircularStitch {
    fn new(x: f32, y: f32, radius: f32, style: CircularStyle) -> CircularStitch {
        CircularStitch {
            x: x,
            y: y,
            radius: radius,
            style: style
        }
    }
}

impl Stitch for CircularStitch {
    fn display_style(&self) -> () {
        match self.style {
            CircularStyle::French => println!("French Knot!"),
            CircularStyle::Wheel => println!("Woven Wheel!")
        }
    }

    fn check_possible(&self, current_thread: &mut ActiveThread) -> bool {
        if current_thread.remaining_cm > ((self.radius * self.radius) * PI) {true}
        else {false}
    }

    fn update_thread(&self, current_thread: &mut ActiveThread) -> () {
        let area = (self.radius * self.radius) * PI;
        current_thread.remaining_cm -= area;
    }
}

trait Stitch {
    fn display_style(&self) -> ();

    fn check_possible(&self, current_thread: &mut ActiveThread) -> bool;

    fn update_thread(&self, current_thread: &mut ActiveThread) -> ();

    fn make_stitch(&self, current_thread: &mut ActiveThread) -> () {
        if self.check_possible(current_thread) {
            self.update_thread(current_thread);
        }
        else {panic!("Cannot complete an impossible stitch.")}
    }
}

fn thread_needle(length: f32, strands: u32) -> ActiveThread {
    if strands > 6 {panic!("Cannot embroider with more than 6 strands.")}
    ActiveThread {started: true, total_cm: length, strand_number: strands, remaining_cm: length}
}

fn main() {
    let mut my_thread = thread_needle(40.0, 4);

    let stitch_1 = LinearStitch::new(1.0, 7.0, 2.0, LinearStyle::Running);
    let stitch_2 = LinearStitch::new(8.0, 23.0, 2.0, LinearStyle::Stem);
    let stitch_3 = CircularStitch::new(1.0, 10.0, 5.0, CircularStyle::Wheel);

    stitch_1.make_stitch(&mut my_thread);
    assert!(my_thread.remaining_cm == 34.0);
    stitch_2.make_stitch(&mut my_thread);
    assert!(my_thread.remaining_cm == 19.0);
    stitch_3.make_stitch(&mut my_thread);
}
