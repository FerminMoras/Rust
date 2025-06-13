use std::vec;

#[derive(Debug,PartialEq,Clone)]
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

fn al_menos_una<'a>(v: &'a Vec<Persona<'a>>, ciudad_act: &str) -> bool {
    v.iter().any(|p| p.ciudad == ciudad_act)
}

fn existe_persona<'a>(v: &'a Vec<Persona<'a>>, p: &Persona) -> bool {
    v.iter().any(|per| per == p)
}

fn edades<'a> (v: &'a Vec<Persona<'a>>) -> Vec<u8> {
    v.iter().map(|p|p.edad).collect()
}

fn personas_menormayor_edad<'a>(v: &'a Vec<Persona<'a>>) -> (Persona<'a>, Persona<'a>) {
    let mut menor = &v[0];
    let mut mayor = &v[0];

    for persona in v.iter() {
        if persona.salario < menor.salario ||
           (persona.salario == menor.salario && persona.edad > menor.edad) {
            menor = persona;
        }

        if persona.salario > mayor.salario ||
           (persona.salario == mayor.salario && persona.edad > mayor.edad) {
            mayor = persona;
        }
    }

    (menor.clone(), mayor.clone())
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
    let p2 = Persona::new("Ever", "Ludueña", "Av Mitre 234", "9 de Julio", 500000.60, 52);
    let p3 = Persona::new("Wanchope", "Abila", "Edison 898", "9 de Julio", 785000.60, 37);

    let mut vector:Vec<Persona<'_>>  = Vec::new();
    vector.push(p1.clone());
    vector.push(p2);
    vector.push(p3.clone());

    let resultado = lista_max_edad(&vector, 20, "9 de Julio");
    let ok = viven_ciudad(&vector, "9 de Julio");
    let ok2 = al_menos_una(&vector, "9 de Julio");
    let ok3 = existe_persona(&vector, &p1);
    let edades = edades(&vector);
    let menormayor = personas_menormayor_edad(&vector);

    //test max edad
    assert_eq!(resultado.len(),3);
    assert_eq!(resultado[0].nombre,"Fermin");
    assert_eq!(resultado[1].nombre,"Ever");
    assert_eq!(resultado[2].nombre,"Wanchope");

    //test si todas las personas viven en la misma ciudad
    assert_eq!(ok,true);

    //test si al menos 1 persona vive en la ciudad
    assert_eq!(ok2,true);

    //test si al menos una persona existe en el vector
    assert_eq!(ok3,true);

    //test retorna arreglo con las edades
    assert_eq!(edades[0], 26);
    assert_eq!(edades[1], 52);
    assert_eq!(edades[2], 37);

    //test persona mayor y menor edad
    assert_eq!(menormayor.0,p1);
    assert_eq!(menormayor.1,p3);
}    