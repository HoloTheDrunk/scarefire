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
    pub projection: glm::Mat4,
    pub view: glm::Mat4,
    pub view_proj: glm::Mat4,
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
            view: glm::ext::look_at(
                glm::vec3(2., 2., 2.),
                glm::vec3(0., 0., 0.),
                glm::vec3(0., 1., 0.),
            ),

            view_proj: glm::Mat4::from_array(&[Vector4::from_array(&[0.; 4]).to_owned(); 4])
                .to_owned(),
        }
    }

    fn update(&mut self) {
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

    pub fn set_fov(&mut self, fov: f32) {
        self.set_proj(&Camera::perspective(
            fov,
            self.ratio(),
            Camera::extract_near(&self.projection),
        ));
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        self.set_proj(&Camera::perspective(
            self.fov(),
            ratio,
            Camera::extract_near(&self.projection),
        ))
    }

    fn extract_near(projection: &glm::Mat4) -> f32 {
        projection[3][2]
    }

    pub fn fov(&self) -> f32 {
        if (self.is_orthographic()) {
            0.0
        } else {
            2.0 * (1.0 / self.projection[1][1]).atan()
        }
    }

    pub fn ratio(&self) -> f32 {
        let f: f32 = self.projection[1][1];
        (1.0 / (self.projection[0][0] / f)).abs()
    }

    pub fn position(&self) -> glm::Vec3 {
        let mut pos = glm::vec3(0., 0., 0.);
        for i in 0..3 {
            pos = pos
                - glm::vec3(self.view[0][i], self.view[1][i], self.view[2][i]) * self.view[3][i];
        }
        pos
    }

    pub fn forward(&self) -> glm::Vec3 {
        -glm::normalize(glm::vec3(self.view[0][2], self.view[1][2], self.view[2][2]))
    }

    pub fn right(&self) -> glm::Vec3 {
        glm::normalize(glm::vec3(self.view[0][0], self.view[1][0], self.view[2][0]))
    }

    pub fn up(&self) -> glm::Vec3 {
        glm::normalize(glm::vec3(self.view[0][1], self.view[1][1], self.view[2][1]))
    }

    pub fn is_orthographic(&self) -> bool {
        self.projection[3][3] == 1.
    }

    pub fn build_frustum(&self) -> Frustum {
        let camera_up = self.up();
        let camera_right = self.right();
        let camera_forward = self.forward();

        let half_fov = self.fov() * 0.5;
        let half_fov_v = (half_fov.tan() * self.ratio()).atan();

        let (c, s) = (half_fov.cos(), half_fov.sin());
        let (c_v, s_v) = (half_fov_v.cos(), half_fov_v.sin());

        Frustum {
            near_normal: camera_forward,
            top_normal: camera_forward * s - camera_up * c,
            bottom_normal: camera_forward * s + camera_up * c,
            right_normal: camera_forward * s_v - camera_right * c_v,
            left_normal: camera_forward * s_v + camera_right * c_v,
        }
    }
}
