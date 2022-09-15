use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Quaternion, Rad, Rotation3, Vector3};
use geometric_algebra::{
    pga3::{Point, Translator},
    RegressiveProduct,
};

use crate::{
    entity::{self},
    line_debugger, mesh,
    numeric::quat_to_rotor,
    renderer, rigid, solver,
};

pub struct World {
    cube: entity::Entity,
    rigid: RefCell<rigid::Rigid>,
}

impl World {
    pub fn new(renderer: &renderer::Renderer) -> World {
        let cube = entity::Entity::default().meshes(vec![Rc::new(mesh::Mesh::new_cube(renderer))]);

        let mut rigid = rigid::Rigid::new(1.0);
        rigid.external_force.z = -5.0;
        rigid.velocity.z = -0.2;
        rigid.angular_velocity.z = 1.0;
        rigid.frame.position.z = 5.0;
        rigid.frame.quaternion =
            Quaternion::from_axis_angle(Vector3::new(1.0, 0.5, 0.2).normalize(), Rad(1.0));
        rigid.past_frame = rigid.frame;

        World {
            cube,
            rigid: RefCell::new(rigid),
        }
    }

    pub fn integrate(&mut self, _t: f32, dt: f32, line_debugger: &mut line_debugger::LineDebugger) {
        solver::integrate(&self.rigid, dt, 25);

        let rigid = self.rigid.borrow();

        self.cube.spatial.translator = Translator::new(
            rigid.frame.position.x,
            rigid.frame.position.y,
            rigid.frame.position.z,
        );

        line_debugger.debug_lines(
            vec![
                Point::origin(),
                Point::at(
                    rigid.frame.position.x,
                    rigid.frame.position.y,
                    rigid.frame.position.z,
                ),
            ],
            [1.0, 1.0, 0.0].into(),
        );

        let a = Point::at(1.0, 0.0, 0.0);
        let b = Point::at(0.0, 1.0, 0.0);
        let c = Point::at(0.0, 0.0, 1.0);

        line_debugger.debug(a, Vector3::new(1.0, 0.0, 1.0));
        line_debugger.debug(b, Vector3::new(1.0, 0.0, 1.0));
        line_debugger.debug(c, Vector3::new(1.0, 0.0, 1.0));
        line_debugger.debug(a.regressive_product(b), Vector3::new(1.0, 0.0, 1.0));
        line_debugger.debug(b.regressive_product(c), Vector3::new(1.0, 0.0, 1.0));
        line_debugger.debug(c.regressive_product(a), Vector3::new(1.0, 0.0, 1.0));
        line_debugger.debug(
            a.regressive_product(b).regressive_product(c),
            Vector3::new(1.0, 0.0, 1.0),
        );

        self.cube.spatial.rotor = quat_to_rotor(rigid.frame.quaternion);
    }

    pub fn entities(&self) -> Vec<entity::Entity> {
        vec![self.cube.clone()]
    }
}
