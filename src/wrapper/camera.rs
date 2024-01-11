use glm::Vector4;

pub struct Frustum {
    near_normal: glm::Vec3,
    // No far plane (zFar is +inf)
    top_normal: glm::Vec3,
    bottom_normal: glm::Vec3,
    right_normal: glm::Vec3,
    left_normal: glm::Vec3,
}

pub struct Camera {
    projection: glm::Mat4,
    view: glm::Mat4,
    view_proj: glm::Mat4,
}

impl Camera {
    #[rustfmt::skip]
    pub fn perspective(fov_y: f32, ratio: f32, z_near: f32) -> glm::Mat4 {
        let f = 1. / (fov_y / 2.).tan();
        glm::mat4(
            f / ratio, 0.,   0.  ,  0.,
                0.   , f ,   0.  ,  0.,
                0.   , 0.,   0.  , -1.,
                0.   , 0., z_near,  0.,
        )
    }

    pub fn new() -> Self {
        Self {
            projection: Camera::perspective(60f32.to_radians(), 16. / 9., 1e-3),
            // TODO: implement lookAt in glm-rs fork
            view: glm::ext::look_at(
                glm::vec3(2., 2., 2.),
                glm::vec3(0., 0., 0.),
                glm::vec3(0., 1., 0.),
            ),

            view_proj: glm::Mat4::from_array(&[Vector4::from_array(&[0.; 4]).to_owned(); 4])
                .to_owned(),
        }
    }

    pub fn update(&mut self) {
        self.view_proj = self.projection * self.view;
    }

    pub fn set_view(&mut self, matrix: &glm::Mat4) {
        self.view = *matrix;
        self.update();
    }

    pub fn set_proj(&mut self, matrix: &glm::Mat4) {
        self.projection = *matrix;
        self.update();
    }

    fn extract_ratio(projection: &glm::Mat4) -> f32 {
        let f: f32 = projection[1][1];
        (1.0 / (projection[0][0] / f)).abs()
    }

    fn extract_near(projection: &glm::Mat4) -> f32 {
        projection[3][2]
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.set_proj(&Camera::perspective(
            fov,
            Camera::extract_ratio(&self.projection),
            Camera::extract_near(&self.projection),
        ));
    }

    fn fov(&self) -> f32 {
        if (self.projection[3][3] == 1.0) {
            0.0
        } else {
            2.0 * (1.0 / self.projection[1][1]).atan()
        }
    }

    pub fn set_ratio(&self, ratio: f32) {
        self.set_proj(&Camera::perspective(
            Camera::fov(self),
            ratio,
            Camera::extract_near(&self.projection),
        ))
    }
}
