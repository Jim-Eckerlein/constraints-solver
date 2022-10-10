use cgmath::{ElementWise, InnerSpace, Quaternion, Vector3, Zero};
use derive_setters::Setters;

use crate::frame::Frame;

#[derive(Debug, Clone, Copy, Setters)]
pub struct Rigid {
    /// Mass in `kg`
    pub mass: f64,

    /// Rotational inertia tensor.
    /// Since the geometry is assumed to be in rest space,
    /// this tensor is sparse and just the diagonal entries are stored.
    /// Measured in `kg m^2`.
    pub rotational_inertia: Vector3<f64>,

    /// Force acting on the rigid body outside its frame.
    /// Measured in `N`.
    pub external_force: Vector3<f64>,

    /// Force acting on the rigid body within its own frame.
    /// Measured in `N`.
    pub internal_force: Vector3<f64>,

    /// Torque acting on the rigid body outside its frame.
    /// Measrued in `N m`.
    pub external_torque: Vector3<f64>,

    /// Torque acting on the rigid body within its own frame.
    /// Measrued in `N m`.
    pub internal_torque: Vector3<f64>,

    /// Current velocity of the rigid body in `m s^-1`
    pub velocity: Vector3<f64>,

    /// Current angular velocity of the rigid body in `s^-1`
    pub angular_velocity: Vector3<f64>,

    pub frame: Frame,
    pub past_frame: Option<Frame>,

    pub color: Option<[f32; 3]>,
}

impl Rigid {
    pub fn new(mass: f64) -> Rigid {
        let extent = Vector3::new(1.0, 1.0, 1.0);
        let rotational_inertia = 1.0 / 12.0
            * mass
            * Vector3::new(
                extent.y * extent.y + extent.z * extent.z,
                extent.x * extent.x + extent.z * extent.z,
                extent.x * extent.x + extent.y * extent.y,
            );

        Rigid {
            mass,
            rotational_inertia,
            internal_force: Vector3::zero(),
            external_force: Vector3::zero(),
            internal_torque: Vector3::zero(),
            external_torque: Vector3::zero(),
            velocity: Vector3::zero(),
            angular_velocity: Vector3::zero(),
            frame: Frame::default(),
            past_frame: None,
            color: None,
        }
    }

    pub fn integrate(&mut self, dt: f64) {
        let force = self.external_force + self.frame.quaternion * self.internal_force;
        self.velocity += dt * force / self.mass;

        let torque = self.external_torque + self.frame.quaternion * self.internal_torque;
        self.angular_velocity += dt * torque.div_element_wise(self.rotational_inertia);

        self.past_frame = Some(self.frame);
        self.frame = self
            .frame
            .integrate(dt, self.velocity, self.angular_velocity);
    }

    pub fn derive(&mut self, dt: f64) {
        if let Some(past) = self.past_frame {
            (self.velocity, self.angular_velocity) = self.frame.derive(dt, past)
        } else {
            self.velocity = Vector3::zero();
            self.angular_velocity = Vector3::zero();
        }
    }

    /// Applies a linear impulse in a given direction and magnitude at a given location.
    /// Results in changes in both position and quaternion.
    pub fn apply_impulse(&mut self, impulse: Vector3<f64>, point: Vector3<f64>) {
        self.frame.position += impulse / self.mass;

        let log = (point - self.frame.position)
            .div_element_wise(self.rotational_inertia)
            .cross(impulse);
        let rotation = 0.5 * Quaternion::new(0.0, log.x, log.y, log.z) * self.frame.quaternion;
        self.frame.quaternion = (self.frame.quaternion + rotation).normalize();
    }

    /// Computes the position difference of a global point in the current frame from the same point in the past frame.
    pub fn delta(&self, global: Vector3<f64>) -> Vector3<f64> {
        if let Some(past) = self.past_frame {
            let local = self.frame.inverse().act(global);
            let past_global = past.act(local);
            global - past_global
        } else {
            Vector3::zero()
        }
    }
}
