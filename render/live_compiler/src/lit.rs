use crate::ty::Ty;
use crate::val::Val;
use crate::colors::Color;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lit {
    Bool(bool),
    Int(i32),
    Float(f32),
    Color(Color)
    
}

impl Lit {
    pub fn to_ty(self) -> Ty {
        match self {
            Lit::Bool(_) => Ty::Bool,
            Lit::Int(_) => Ty::Int,
            Lit::Float(_) => Ty::Float,
            Lit::Color(_) => Ty::Vec4
        }
    }

    pub fn to_val(self) -> Val {
        match self {
            Lit::Bool(lit) => Val::Bool(lit),
            Lit::Int(lit) => Val::Int(lit as i32),
            Lit::Float(lit) => Val::Float(lit),
            Lit::Color(lit) => Val::Vec4(lit.to_vec4())
        }
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lit::Bool(lit) => write!(f, "{}", lit),
            Lit::Int(lit) => write!(f, "{}", lit),
            Lit::Float(lit) => {
                if lit.abs().fract() < 0.00000001 {
                    write!(f, "{}.0", lit)
                } else {
                    write!(f, "{}", lit)
                }
            },
            Lit::Color(lit) =>{
                write!(f, "color({},{},{},{})", lit.r,lit.g,lit.b,lit.a)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum TyLit {
    Bool,
    Int,
    Float,
    Bvec2,
    Bvec3,
    Bvec4,
    Ivec2,
    Ivec3,
    Ivec4,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
    Texture2D,
}

impl TyLit {
    pub fn to_ty(self) -> Ty {
        match self {
            TyLit::Bool => Ty::Bool,
            TyLit::Int => Ty::Int,
            TyLit::Float => Ty::Float,
            TyLit::Bvec2 => Ty::Bvec2,
            TyLit::Bvec3 => Ty::Bvec3,
            TyLit::Bvec4 => Ty::Bvec4,
            TyLit::Ivec2 => Ty::Ivec2,
            TyLit::Ivec3 => Ty::Ivec3,
            TyLit::Ivec4 => Ty::Ivec4,
            TyLit::Vec2 => Ty::Vec2,
            TyLit::Vec3 => Ty::Vec3,
            TyLit::Vec4 => Ty::Vec4,
            TyLit::Mat2 => Ty::Mat2,
            TyLit::Mat3 => Ty::Mat3,
            TyLit::Mat4 => Ty::Mat4,
            TyLit::Texture2D => Ty::Texture2D,
        }
    }
    /*
    pub fn from_ident(ident:Ident)->Option<TyLit>{
        if ident == Ident::new("bool"){return Some(TyLit::Bool)}
        if ident == Ident::new("int"){return Some(TyLit::Int)}
        if ident == Ident::new("float"){return Some(TyLit::Float)}
        if ident == Ident::new("vec2"){return Some(TyLit::Vec2)}
        if ident == Ident::new("vec3"){return Some(TyLit::Vec3)}
        if ident == Ident::new("vec4"){return Some(TyLit::Vec4)}
        if ident == Ident::new("mat2"){return Some(TyLit::Mat2)}
        if ident == Ident::new("mat3"){return Some(TyLit::Mat3)}
        if ident == Ident::new("mat4"){return Some(TyLit::Mat4)}
        if ident == Ident::new("texture2D"){return Some(TyLit::Texture2D)}
        if ident == Ident::new("bvec2"){return Some(TyLit::Bvec2)}
        if ident == Ident::new("bvec3"){return Some(TyLit::Bvec3)}
        if ident == Ident::new("bvec4"){return Some(TyLit::Bvec4)}
        if ident == Ident::new("ivec2"){return Some(TyLit::Ivec2)}
        if ident == Ident::new("ivec3"){return Some(TyLit::Ivec3)}
        if ident == Ident::new("ivec4"){return Some(TyLit::Ivec4)}
        None
    }*/
}

impl fmt::Display for TyLit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TyLit::Bool => "bool",
                TyLit::Int => "int",
                TyLit::Float => "float",
                TyLit::Bvec2 => "bvec2",
                TyLit::Bvec3 => "bvec3",
                TyLit::Bvec4 => "bvec4",
                TyLit::Ivec2 => "ivec2",
                TyLit::Ivec3 => "ivec3",
                TyLit::Ivec4 => "ivec4",
                TyLit::Vec2 => "vec2",
                TyLit::Vec3 => "vec3",
                TyLit::Vec4 => "vec4",
                TyLit::Mat2 => "mat2",
                TyLit::Mat3 => "mat3",
                TyLit::Mat4 => "mat4",
                TyLit::Texture2D => "texture2D",
            }
        )
    }
}
