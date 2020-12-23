// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Box;
use crate::Matrix;
use crate::Plane;
use crate::Point3D;
use crate::Sphere;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Frustum(Boxed<ffi::graphene_frustum_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_frustum_get_type(), ptr as *mut _) as *mut ffi::graphene_frustum_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_frustum_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || ffi::graphene_frustum_get_type(),
    }
}

impl Frustum {
    #[doc(alias = "graphene_frustum_contains_point")]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        unsafe {
            from_glib(ffi::graphene_frustum_contains_point(
                self.to_glib_none().0,
                point.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "graphene_frustum_equal")]
    fn equal(&self, b: &Frustum) -> bool {
        unsafe {
            from_glib(ffi::graphene_frustum_equal(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "graphene_frustum_get_planes")]
    //pub fn get_planes(&self, planes: /*Unimplemented*/FixedArray TypeId { ns_id: 1, id: 8 }; 6) {
    //    unsafe { TODO: call ffi:graphene_frustum_get_planes() }
    //}

    #[doc(alias = "graphene_frustum_init")]
    pub fn init(&mut self, p0: &Plane, p1: &Plane, p2: &Plane, p3: &Plane, p4: &Plane, p5: &Plane) {
        unsafe {
            ffi::graphene_frustum_init(
                self.to_glib_none_mut().0,
                p0.to_glib_none().0,
                p1.to_glib_none().0,
                p2.to_glib_none().0,
                p3.to_glib_none().0,
                p4.to_glib_none().0,
                p5.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "graphene_frustum_init_from_frustum")]
    pub fn init_from_frustum(&mut self, src: &Frustum) {
        unsafe {
            ffi::graphene_frustum_init_from_frustum(
                self.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "graphene_frustum_init_from_matrix")]
    pub fn init_from_matrix(&mut self, matrix: &Matrix) {
        unsafe {
            ffi::graphene_frustum_init_from_matrix(
                self.to_glib_none_mut().0,
                matrix.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "graphene_frustum_intersects_box")]
    pub fn intersects_box(&self, box_: &Box) -> bool {
        unsafe {
            from_glib(ffi::graphene_frustum_intersects_box(
                self.to_glib_none().0,
                box_.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "graphene_frustum_intersects_sphere")]
    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        unsafe {
            from_glib(ffi::graphene_frustum_intersects_sphere(
                self.to_glib_none().0,
                sphere.to_glib_none().0,
            ))
        }
    }
}

impl PartialEq for Frustum {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Frustum {}