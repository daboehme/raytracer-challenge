use crate::camera::Camera;
use crate::color::Color;
use crate::linalg::{M4,V4};
use crate::lighting::{LightSource,Material};
use crate::plane::Plane;
use crate::shape::{BaseShape,Shape};
use crate::sphere::Sphere;
use crate::transform::Transform;
use crate::world::World;

use yaml_rust::{Yaml,YamlLoader};

use std::error;
use std::fmt;
use std::rc::Rc;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
enum ParseError {
    Missing,
    MissingElem(&'static str),
    UnknownValue(String),
    WrongType(&'static str),
    WrongTypeFor(&'static str,&'static str),
    In(&'static str, Box<dyn error::Error>)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Missing
                => f.write_fmt(format_args!("element missing")),
            ParseError::MissingElem(s)
                => f.write_fmt(format_args!("\"{}\" missing", s)),
            ParseError::UnknownValue(s)
                => f.write_fmt(format_args!("unknown value {}", s)),
            ParseError::WrongType(typestr)
                => f.write_fmt(format_args!("expected {}", typestr)),
            ParseError::WrongTypeFor(elem,typestr)
                => f.write_fmt(format_args!("{}: expected {}", elem, typestr)),
            ParseError::In(elem,err)
                => {
                    f.write_fmt(format_args!("In {}: ", elem))?;
                    err.fmt(f)
                }
        }
    }
}

impl error::Error for ParseError {}

const TYPE_V3  : &str = "3 floating-point values";
const TYPE_F32 : &str = "floating-point value";

fn read_v3_data(v: &Vec<Yaml>) -> Result<[f32;3]> {
    if v.len() != 3 {
        return Err(ParseError::WrongType(TYPE_V3).into())
    } else {
        let mut ret = [ 0.0, 0.0, 0.0 ];
        for i in 0..3 {
            ret[i] = match &v[i] {
                Yaml::Real(s) => s.parse::<f32>()?,
                _ => return Err(ParseError::WrongType(TYPE_V3).into())
            }
        }
        Ok (ret)
    }
}

fn read_v3(yml: &Yaml) -> Result<[f32;3]> {
    match yml {
        Yaml::Array(v) => Ok(read_v3_data(&v)?),
        Yaml::BadValue => Err(ParseError::Missing.into()),
        _ => Err(ParseError::WrongType(TYPE_V3).into())
    }
}

fn read_v3_or(yml: &Yaml, default: &[f32;3]) -> Result<[f32;3]> {
    match yml {
        Yaml::Array(v) => Ok(read_v3_data(&v)?),
        Yaml::BadValue => Ok(*default),
        _ => Err(ParseError::WrongType(TYPE_V3).into())
    }
}

fn read_f32(node: &Yaml) -> Result<f32> {
    let val = match node {
        Yaml::Real(s) => s.parse::<f32>()?,
        Yaml::BadValue => return Err(ParseError::Missing.into()),
        _ => return Err(ParseError::WrongType(TYPE_F32).into())
    };

    Ok(val)
}

fn read_camera(node: &Yaml) -> Result<Camera> {
    let mut width_height = [ ("width", 0), ("height", 0) ];
    for elem in width_height.iter_mut() {
        elem.1 = match node[elem.0] {
            Yaml::Integer(i) => i,
            Yaml::BadValue => return Err(ParseError::MissingElem(elem.0).into()),
            _ => return Err(ParseError::WrongTypeFor(elem.0, "integer").into())
        }
    }

    let fov = match read_f32(&node["field_of_view"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("field_of_view", e).into())
    };

    let from = match read_v3(&node["from"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("from", e).into())
    };
    let to = match read_v3(&node["to"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("to", e).into())
    };
    let up = match read_v3_or(&node["up"], &[ 0.0, 1.0, 0.0 ]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("up", e).into())
    };

    let w = width_height[0].1 as usize;
    let h = width_height[1].1 as usize;

    let from = V4::new_point(from[0], from[1], from[2]);
    let to = V4::new_point(to[0], to[1], to[2]);
    let up = V4::new_vector(up[0], up[1], up[2]);

    let vt = Transform::view_transform(&from, &to, &up);

    Ok(Camera::new(w, h, fov.to_radians(), &vt.matrix))
}

fn read_pointlight(node: &Yaml) -> Result<LightSource> {
    let pos = match read_v3(&node["position"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("position", e).into())
    };

    let col = match read_v3_or(&node["intensity"], &[ 1.0, 1.0, 1.0 ]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("position", e).into())
    };

    let pos = V4::new_point(pos[0], pos[1], pos[2]);
    let col = Color::new(col[0], col[1], col[2]);

    Ok(LightSource { pos: pos, intensity: col })
}

fn read_lights(node: &Yaml) -> Result<Vec<LightSource>> {
    let mut lights = Vec::new();

    match node {
        Yaml::Array(v) => {
            for lnode in v {
                match lnode {
                    Yaml::Hash(kv) => {
                        for (key, val) in kv.iter() {
                            let key = key.as_str().unwrap();

                            match key {
                                "point" => {
                                    let l = read_pointlight(val)?;
                                    lights.push(l)
                                },
                                _ => return Err(ParseError::UnknownValue(String::from(key)).into())
                            }
                        }
                    },
                    _ => return Err(ParseError::WrongType("dict").into())
                }
            }
        },
        Yaml::BadValue => return Err(ParseError::Missing.into()),
        _ => return Err(ParseError::WrongType("array").into())
    }

    Ok(lights)
}

fn read_material(node: &Yaml) -> Result<Material> {
    let col = match read_v3(&node["color"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("color", e).into())
    };
    let ambient = match read_f32(&node["ambient"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("ambient", e).into())
    };
    let diffuse = match read_f32(&node["diffuse"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("diffuse", e).into())
    };
    let specular = match read_f32(&node["specular"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("ambient", e).into())
    };
    let shininess = match read_f32(&node["shininess"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("shininess", e).into())
    };

    Ok( Material {
        color: Color::new(col[0], col[1], col[2]),
        ambient: ambient,
        diffuse: diffuse,
        specular: specular,
        shininess: shininess
    })
}

fn read_transformations(nodes: &Vec<Yaml>) -> Result<M4> {
    let mut trans = Transform::new();

    for node in nodes.iter() {
        match node {
            Yaml::Hash(kv) => {
                for (key, val) in kv.iter() {
                    let key = key.as_str().unwrap();

                    match key {
                        "translate" => {
                            let v = read_v3(val)?;
                            trans = trans.translate(v[0], v[1], v[2]);
                        },
                        "scale" => {
                            let v = read_v3(val)?;
                            trans = trans.scale(v[0], v[1], v[2]);
                        },
                        "rotate_x" => {
                            let v = read_f32(val)?;
                            trans = trans.rotate_x(v.to_radians());
                        },
                        "rotate_y" => {
                            let v = read_f32(val)?;
                            trans = trans.rotate_y(v.to_radians());
                        },
                        "rotate_z" => {
                            let v = read_f32(val)?;
                            trans = trans.rotate_z(v.to_radians());
                        },
                        _ => return Err(ParseError::UnknownValue(String::from(key)).into())
                    }
                }
            }
            _ => return Err(ParseError::WrongType("dict").into())
        }
    }

    Ok(trans.matrix)
}

fn read_shape(root: &Yaml, node: &Yaml) -> Result< Rc<Shape> > {
    let shape = match node {
        Yaml::Hash(kv) => {
            match kv.iter().next() {
                Some((key,val)) => {
                    let key = key.as_str().unwrap();

                    let base: Box<dyn BaseShape> = match key {
                        "plane" => Box::new(Plane()),
                        "sphere" => Box::new(Sphere()),
                        _ => return Err(ParseError::UnknownValue(String::from(key)).into())
                    };

                    let trans = match &val["transformations"] {
                        Yaml::Array(v) => read_transformations(&v)?,
                        Yaml::BadValue => M4::identity(),
                        _ => return Err(ParseError::WrongTypeFor("transformations", "array").into())
                    };

                    let matnode = &val["material"];
                    let mat = match matnode {
                        Yaml::Hash(_) => read_material(&matnode)?,
                        Yaml::String(s) => read_material(&root[s.as_str()])?,
                        Yaml::BadValue => return Err(ParseError::MissingElem("material").into()),
                        _ => return Err(ParseError::WrongTypeFor("material", "dict or entry").into())
                    };

                    Rc::new(Shape::new(base, &mat, &trans))
                },
                None => return Err(ParseError::Missing.into())
            }
        },
        _ => return Err(ParseError::WrongType("dict").into())
    };

    Ok(shape)
}

fn read_shapes(root: &Yaml, node: &Yaml) -> Result< Vec<Rc<Shape>> > {
    let mut shapes = Vec::new();

    match node {
        Yaml::Array(v) => {
            for shapenode in v {
                shapes.push(read_shape(root, shapenode)?);
            }
        },
        Yaml::BadValue => return Err(ParseError::Missing.into()),
        _ => return Err(ParseError::WrongType("array").into())
    }

    Ok(shapes)
}

pub fn read_yaml_scene_config(str: &str) -> Result<(Camera,World)> {
    let docs = YamlLoader::load_from_str(str)?;

    let camera = match read_camera(&docs[0]["camera"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("camera", e).into())
    };

    let lights = match read_lights(&docs[0]["lights"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("lights", e).into())
    };

    let shapes = match read_shapes(&docs[0], &docs[0]["shapes"]) {
        Ok(v) => v,
        Err(e) => return Err(ParseError::In("shapes", e).into())
    };

    Ok( (camera, World::new_with(lights, shapes)) )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::*;
    use float_cmp::*;

    #[test]
    fn read_camera_ok() {
        let s =
"
width: 640
height: 480
field_of_view:
  60.0
from: [ 0.0, 1.5, -5.0 ]
to:
  - 0.0
  - 1.0
  - 0.0
whatever: 42
";
        let docs = YamlLoader::load_from_str(&s).unwrap();

        let cam = read_camera(&docs[0]);
        assert!(cam.is_ok())

    }

    #[test]
    fn read_lights_ok() {
        let s =
"
- point:
    intensity: [ 1.0, 0.0, 0.0 ]
    position: [ 4.0, 5.5, -6.0 ]
- point:
    position: [ -5.0, 3.0, -8.0 ]
";

        let docs = YamlLoader::load_from_str(&s).unwrap();

        let lights = read_lights(&docs[0]).unwrap();

        assert_eq!(lights.len(), 2);
        assert_eq!(lights[0].pos, V4::new_point(4.0, 5.5, -6.0));
        assert_eq!(lights[1].intensity, Color::WHITE);
    }

    #[test]
    fn read_material_ok() {
        let s =
"
color: [ 1.0, 1.0, 1.0 ]
ambient: 0.2
diffuse: 0.7
specular: 0.2
shininess: 100.0
";

        let docs = YamlLoader::load_from_str(&s).unwrap();

        let mat = read_material(&docs[0]).unwrap();

        assert_eq!(mat.color, Color::WHITE);
        assert_eq!(mat.ambient, 0.2);
        assert_eq!(mat.shininess, 100.0);
    }

    #[test]
    fn read_transformations_ok() {
        let s =
"
- translate: [ 0.5, 1.0, 3.5 ]
- rotate_x: 180.0
";

        let docs = YamlLoader::load_from_str(&s).unwrap();

        let reference = Transform::new()
                            .translate(0.5, 1.0, 3.5)
                            .rotate_x(180_f32.to_radians());

        let trans = read_transformations(&docs[0].as_vec().unwrap()).unwrap();

        let v = V4::new_vector(1.0, 2.0, 3.0);

        assert!(approx_eq!(V4, &trans * v, reference.apply(v), epsilon = 0.0001));
    }

    #[test]
    fn read_shapes_ok() {
        let s =
"
- plane:
    material:
      color: [ 0.1, 0.3, 0.7 ]
      ambient: 0.3
      diffuse: 0.7
      specular: 0.2
      shininess: 20.0
- sphere:
    material:
      color: [ 0.2, 0.7, 0.4 ]
      ambient: 0.2
      diffuse: 0.7
      specular: 0.2
      shininess: 200.0
    transformations:
      - translate: [ 1.0, 2.0, -3.0 ]
";

        let docs = YamlLoader::load_from_str(&s).unwrap();

        let shapes = read_shapes(&docs[0], &docs[0]).unwrap();

        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes[0].material().ambient, 0.3);
        assert_eq!(shapes[1].material().shininess, 200.0);
    }

    #[test]
    fn read_mat_from_root() {
        let s =
"
.mat.a:
   color: [ 0.1, 0.3, 0.7 ]
   ambient: 0.3
   diffuse: 0.7
   specular: 0.2
   shininess: 20.0

shapes:
  - sphere:
      material: .mat.a
      transformations:
        - translate: [ 1.0, 2.0, -3.0 ]
";

        let docs = YamlLoader::load_from_str(&s).unwrap();

        let shapes = read_shapes(&docs[0], &docs[0]["shapes"]).unwrap();

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].material().ambient, 0.3);
    }
}