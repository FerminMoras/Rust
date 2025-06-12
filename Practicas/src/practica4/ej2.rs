#[derive(Debug,PartialEq)]
struct Persona<'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}

impl<'a> Persona<'a> {
    fn new(nombre: &'a str, apellido: &'a str, direccion: &'a str, ciudad: &'a str, salario: f64, edad: u8) -> Persona<'a> {
        Persona {
            nombre: nombre,
            apellido: apellido,
            direccion: direccion,
            ciudad: ciudad,
            salario: salario,
            edad: edad,
        }
    }
}

fn lista_max_salario<'a> (v: &'a Vec<Persona<'a>>, sal: f64) -> Vec<&'a Persona<'a>> {
    v.iter().filter(|p| p.salario > sal).collect()
}

fn lista_max_edad<'a>(v: &'a Vec<Persona<'a>>, edad_tope: u8, ciudad_act: &str) -> Vec<&'a Persona<'a>> {
    v.iter().filter(|p| p.edad > edad_tope && p.ciudad == ciudad_act).collect()
}

fn viven_ciudad<'a>(v: &'a Vec<Persona<'a>>, ciudad_act: &str) -> bool {
    v.iter().all(|p| p.ciudad == ciudad_act)
}

#[test]
fn test_max_salario() {
    let p1 = Persona::new("Fermin", "Moras", "La Rioja 668", "9 de Julio", 352000.60, 26);
    let p2 = Persona::new("Ever", "Ludueña", "Av Mitre 234", "9 de Julio", 500000.60, 37);
    let p3 = Persona::new("Wanchope", "Abila", "Edison 898", "9 de Julio", 785000.60, 52);

    let mut vector = Vec::new();
    vector.push(p1);
    vector.push(p2);
    vector.push(p3);

    let resultado = lista_max_salario(&vector, 400000.0);

    assert_eq!(resultado.len(),2);
    assert_eq!(resultado[0].nombre,"Ever");
    assert_eq!(resultado[1].nombre,"Wanchope");
}

#[test]
fn test_max_edad() {
    let p1 = Persona::new("Fermin", "Moras", "La Rioja 668", "9 de Julio", 352000.60, 26);
    let p2 = Persona::new("Ever", "Ludueña", "Av Mitre 234", "9 de Julio", 500000.60, 37);
    let p3 = Persona::new("Wanchope", "Abila", "Edison 898", "9 de Julio", 785000.60, 52);

    let mut vector = Vec::new();
    vector.push(p1);
    vector.push(p2);
    vector.push(p3);

    let resultado = lista_max_edad(&vector, 20, "9 de Julio");
    let ok = viven_ciudad(&vector, "9 de Julio");

    assert_eq!(ok,true);
    assert_eq!(resultado.len(),3);
    assert_eq!(resultado[0].nombre,"Fermin");
    assert_eq!(resultado[1].nombre,"Ever");
    assert_eq!(resultado[2].nombre,"Wanchope");
}    