use std::cell::RefCell;

use cgmath::{InnerSpace, Vector3};

use crate::{constraint::Constraint, rigid::Rigid};

const CUBE_VERTICES: [Vector3<f64>; 8] = [
    Vector3::new(-0.5, -0.5, -0.5),
    Vector3::new(0.5, -0.5, -0.5),
    Vector3::new(-0.5, 0.5, -0.5),
    Vector3::new(0.5, 0.5, -0.5),
    Vector3::new(-0.5, -0.5, 0.5),
    Vector3::new(0.5, -0.5, 0.5),
    Vector3::new(-0.5, 0.5, 0.5),
    Vector3::new(0.5, 0.5, 0.5),
];

pub fn ground<'a>(rigid: &'a RefCell<&'a mut Rigid>) -> Vec<Constraint> {
    let mut constraints = Vec::new();

    for vertex in CUBE_VERTICES {
        let position = rigid.borrow().frame.act(vertex);
        if position.z >= 0.0 {
            continue;
        }

        let target_position = Vector3::new(position.x, position.y, 0.0);
        let correction = target_position - position;
        let delta_position = rigid.borrow().delta(position);
        let delta_tangential_position = delta_position - delta_position.project_on(correction);

        constraints.push(Constraint {
            rigid,
            contacts: (position, target_position - 1.0 * delta_tangential_position),
            distance: 0.0,
        })
    }

    constraints
}

impl Rigid {
    #![allow(dead_code)]

    fn support(&self, dir: Vector3<f64>) -> Vector3<f64> {
        CUBE_VERTICES
            .into_iter()
            .map(|p| self.frame.act(p))
            .max_by(|a, b| a.dot(dir).total_cmp(&b.dot(dir)))
            .unwrap()
    }

    fn minkowski_support(&self, other: &Rigid, direction: Vector3<f64>) -> Vector3<f64> {
        self.support(direction) - other.support(-direction)
    }

    pub fn gjk(&self, other: &Rigid) -> bool {
        let mut direction = -self.minkowski_support(other, Vector3::unit_x());
        let mut simplex = Simplex::Point(-direction);

        loop {
            let support = self.minkowski_support(other, direction);

            if !same_direction(direction, support) {
                return false;
            }

            match simplex.enclose(support) {
                Ok((_, _, _, _)) => return true,
                Err((next_simplex, next_direction)) => {
                    simplex = next_simplex;
                    direction = next_direction;
                }
            };
        }
    }
}

/// Simplices up to 3-D.
/// For GJK to work, the invariant that points preceding within the simplex tuple were more recently added
/// to the simplex must be upheld.
#[derive(Debug, Clone, Copy)]
enum Simplex {
    Point(Vector3<f64>),
    Line(Vector3<f64>, Vector3<f64>),
    Triangle(Vector3<f64>, Vector3<f64>, Vector3<f64>),
}

type Tetrahedron = (Vector3<f64>, Vector3<f64>, Vector3<f64>, Vector3<f64>);

impl Simplex {
    fn enclose(self, v: Vector3<f64>) -> Result<Tetrahedron, (Self, Vector3<f64>)> {
        match self {
            Simplex::Point(a) => Err(Self::line(v, a)),
            Simplex::Line(a, b) => Err(Self::triangle(v, a, b)),
            Simplex::Triangle(a, b, c) => Self::tetrahedron(v, a, b, c),
        }
    }

    fn line(a: Vector3<f64>, b: Vector3<f64>) -> (Self, Vector3<f64>) {
        let ab = b - a;
        let ao = -a;

        if same_direction(ab, ao) {
            (Simplex::Line(a, b), ab.cross(ao).cross(ab))
        } else {
            (Simplex::Point(a), ao)
        }
    }

    fn triangle(a: Vector3<f64>, b: Vector3<f64>, c: Vector3<f64>) -> (Self, Vector3<f64>) {
        let ab = b - a;
        let ac = c - a;
        let ao = -a;

        let abc = ab.cross(ac);

        if same_direction(abc.cross(ac), ao) {
            if same_direction(ac, ao) {
                (Simplex::Line(a, c), ac.cross(ao).cross(ac))
            } else {
                Self::line(a, b)
            }
        } else if same_direction(ab.cross(abc), ao) {
            Self::line(a, b)
        } else if same_direction(abc, ao) {
            (Simplex::Triangle(a, b, c), abc)
        } else {
            (Simplex::Triangle(a, c, b), -abc)
        }
    }

    fn tetrahedron(
        a: Vector3<f64>,
        b: Vector3<f64>,
        c: Vector3<f64>,
        d: Vector3<f64>,
    ) -> Result<Tetrahedron, (Self, Vector3<f64>)> {
        let ab = b - a;
        let ac = c - a;
        let ad = d - a;
        let ao = -a;

        let abc = ab.cross(ac);
        let acd = ac.cross(ad);
        let adb = ad.cross(ab);

        if same_direction(abc, ao) {
            Err(Self::triangle(a, b, c))
        } else if same_direction(acd, ao) {
            Err(Self::triangle(a, c, d))
        } else if same_direction(adb, ao) {
            Err(Self::triangle(a, d, b))
        } else {
            Ok((a, b, c, d))
        }
    }
}

fn same_direction(a: Vector3<f64>, b: Vector3<f64>) -> bool {
    a.dot(b) > 0.0
}
