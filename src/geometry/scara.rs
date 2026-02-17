const ELBOW_UP:bool = false;

pub struct Scara {
    pub l1: f64,
    pub l2: f64,
    pub base: [f64; 2],
}

pub struct ScaraState {
    pub theta1: f64,
    pub theta2: f64,
}

impl Scara {
    pub fn fk(&self, state: &ScaraState) -> ([f64; 2], [f64; 2]) {
        let (x0, y0) = (self.base[0], self.base[1]);

        let x1 = x0 + self.l1 * state.theta1.cos();
        let y1 = y0 + self.l1 * state.theta1.sin();

        let x2 = x1 + self.l2 * (state.theta1 + state.theta2).cos();
        let y2 = y1 + self.l2 * (state.theta1 + state.theta2).sin();

        ([x1, y1], [x2, y2])
    }

    pub fn ik(&self, target: [f64; 2], state: &mut ScaraState) -> bool{

        let d = ((target[0] - self.base[0])*(target[0] - self.base[0]) + (target[1] - self.base[1])*(target[1] - self.base[1])).sqrt();
        let reach = self.l1 + self.l2;

        let possible = d < reach;

        let x = target[0] - self.base[0];
        let y = target[1] - self.base[1];

        if !possible{
            let x = (x*reach / d) - 0.1;
            let y = (y*reach / d) - 0.1;
            println!("d:{:.1},r{:.1} ==> ({:.1}, {:.1}) -> {:.1}",d,reach,x, y, (x*x + y*y).sqrt());
        }

        let r2 = (x*x + y*y).sqrt();

        let l1 = self.l1;
        let l2 = self.l2;

        let cos_theta2 = (r2 - l1 - l2) / (2.0 * (l1 * l2));
        
        print!("cos_theta2: {:.1}, ", cos_theta2);

        let theta2_a = cos_theta2.acos();
        let theta2_b = -theta2_a;

        let k1_a = l1+l2*theta2_a.cos();
        let k2_a = l2*theta2_a.sin();
        let k1_b = l1+l2*theta2_b.cos();
        let k2_b = l2*theta2_b.sin();

        let theta1_a = y.atan2(x) - k2_a.atan2(k1_a);
        let theta1_b = y.atan2(x) - k2_b.atan2(k1_b);

        if ELBOW_UP {
            state.theta1 = theta1_b;
            state.theta2 = theta2_b;
        }else{
            state.theta1 = theta1_a;
            state.theta2 = theta2_a;
        }
        
        println!("theta1: {:.1}, theta2: {:.1}", state.theta1, state.theta2);

        return possible

    }
}
