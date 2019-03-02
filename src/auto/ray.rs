// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Plane;
use Point3D;
use Vec3;
use ffi;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Ray(Boxed<ffi::graphene_ray_t>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::graphene_ray_get_type(), ptr as *mut _) as *mut ffi::graphene_ray_t,
        free => |ptr| gobject_ffi::g_boxed_free(ffi::graphene_ray_get_type(), ptr as *mut _),
        get_type => || ffi::graphene_ray_get_type(),
    }
}

impl Ray {
    pub fn alloc() -> Ray {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::graphene_ray_alloc())
        }
    }

    fn equal(&self, b: &Ray) -> bool {
        unsafe {
            from_glib(ffi::graphene_ray_equal(self.to_glib_none().0, b.to_glib_none().0))
        }
    }

    pub fn get_closest_point_to_point(&self, p: &Point3D) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_ray_get_closest_point_to_point(self.to_glib_none().0, p.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn get_direction(&self) -> Vec3 {
        unsafe {
            let mut direction = Vec3::uninitialized();
            ffi::graphene_ray_get_direction(self.to_glib_none().0, direction.to_glib_none_mut().0);
            direction
        }
    }

    pub fn get_distance_to_plane(&self, p: &Plane) -> f32 {
        unsafe {
            ffi::graphene_ray_get_distance_to_plane(self.to_glib_none().0, p.to_glib_none().0)
        }
    }

    pub fn get_distance_to_point(&self, p: &Point3D) -> f32 {
        unsafe {
            ffi::graphene_ray_get_distance_to_point(self.to_glib_none().0, p.to_glib_none().0)
        }
    }

    pub fn get_origin(&self) -> Point3D {
        unsafe {
            let mut origin = Point3D::uninitialized();
            ffi::graphene_ray_get_origin(self.to_glib_none().0, origin.to_glib_none_mut().0);
            origin
        }
    }

    pub fn get_position_at(&self, t: f32) -> Point3D {
        unsafe {
            let mut position = Point3D::uninitialized();
            ffi::graphene_ray_get_position_at(self.to_glib_none().0, t, position.to_glib_none_mut().0);
            position
        }
    }

    pub fn init(&mut self, origin: Option<&Point3D>, direction: Option<&Vec3>) -> Option<Ray> {
        unsafe {
            from_glib_none(ffi::graphene_ray_init(self.to_glib_none_mut().0, origin.to_glib_none().0, direction.to_glib_none().0))
        }
    }

    pub fn init_from_ray(&mut self, src: &Ray) -> Option<Ray> {
        unsafe {
            from_glib_none(ffi::graphene_ray_init_from_ray(self.to_glib_none_mut().0, src.to_glib_none().0))
        }
    }

    pub fn init_from_vec3(&mut self, origin: Option<&Vec3>, direction: Option<&Vec3>) -> Option<Ray> {
        unsafe {
            from_glib_none(ffi::graphene_ray_init_from_vec3(self.to_glib_none_mut().0, origin.to_glib_none().0, direction.to_glib_none().0))
        }
    }
}

impl PartialEq for Ray {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Ray {}
