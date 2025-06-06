struct Estudiante {
    nombre: String,
    legajo: u32,
    parciales: Vec<Parciales>,
}

struct Parciales {
    materia: String,
    nota: f32,
}

impl Parciales {
    fn new(materia: String, nota: f32) -> Parciales {
        if materia == "" || nota > 10.0 {
            panic!("datos incorrectos");
        }
        else {
            Parciales {
                materia: materia,
                nota: nota,
            }
        }    
    }
}

impl Estudiante {
    fn new(nombre: String, legajo: u32, parciales: Vec<Parciales>) -> Estudiante {
        if nombre == "" {
            panic!("datos incorrectos");
        }
        else {
            Estudiante {
                nombre: nombre,
                legajo: legajo,
                parciales: parciales,
            }
        }    
    }

    fn obtener_promedio(&self) -> f32 {
        let vector = &self.parciales;
        let mut suma: f32 = 0.0;
        for i in 0..vector.len() {
            suma += vector[i].nota;
        }
        let size = vector.len() as f32;
        let promedio = suma / size;
        promedio
    }

    fn obtener_nota_mas_alta(&self) -> String {
        let mut aux: String = "".to_string();
        let vector = &self.parciales;
        let mut max: f32 = -9999.99;
        for i in 0..vector.len() {
            if vector[i].nota > max {
                max = vector[i].nota;
                aux = format!("{} {}", max, vector[i].materia.to_string());
            }
        }

        aux
    }

    fn obtener_nota_mas_baja(&self) -> String {
        let mut aux: String = "".to_string();
        let vector = &self.parciales;
        let mut min: f32 = 9999.99;
        for i in 0..vector.len() {
            if vector[i].nota < min {
                min = vector[i].nota;
                aux = format!("{} {}", min, vector[i].materia.to_string());
            }
        }

        aux
    }

    fn total_examenes(&self) -> u32 {
        let vector = &self.parciales;
        let mut cant = 0;
        for i in vector {
            cant += 1;
        }
        cant
    }

    fn generar_informe(&self) -> String {
        if self.total_examenes() != 0 {
            format!(
            "Nombre: {} Legajo: {} Total examenes: {} Promedio {} Nota mas Alta: {} Nota mas baja: {}", 
            &self.nombre, &self.legajo, &self.total_examenes(), &self.obtener_promedio(), &self.obtener_nota_mas_alta(), self.obtener_nota_mas_baja()
            )    
        }else {
            return "El alumno no rindio ningun examen".to_string();
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_parcial(){
        let par = Parciales::new("cadp".to_string(), 7.0);
        assert_eq!(par.nota, 7.0);
    }

    #[should_panic]
    #[test]
    fn test_parcial_mal_creado(){
        let _par = Parciales::new("".to_string(), 7.0);
        let _par = Parciales::new("cadp".to_string(), 27.0);
    }

    #[test]
    fn test_crear_estudiante() {
        let par1 = Parciales::new("cadp".to_string(), 7.0);
        let par2 = Parciales::new("ac".to_string(), 5.0);
        let par3 = Parciales::new("taller".to_string(), 4.0);
        let par4 = Parciales::new("oc".to_string(), 10.0);
        let par5 = Parciales::new("fod".to_string(), 6.0);

        let mut vector  = Vec::new();
        vector.push(par1);
        vector.push(par2);
        vector.push(par3);
        vector.push(par4);
        vector.push(par5);
    
        let est = Estudiante::new("Fermin Moras".to_string(), 22993/5, vector);
        assert_eq!(est.generar_informe(),"Nombre: Fermin Moras Legajo: 4598 Total examenes: 5 Promedio 6.4 Nota mas Alta: 10 oc Nota mas baja: 4 taller".to_string());
    }

    #[test]
    fn test_estudiante_sin_parciales() {
        let mut vector = Vec::new();
        let est = Estudiante::new("Ramon Abila".to_string(), 12674/6, vector);

        assert_eq!(est.generar_informe(),"El alumno no rindio ningun examen".to_string());
    }

    #[should_panic]
    #[test]
    fn test_crear_mal_estudiante() {

        let par1 = Parciales::new("cadp".to_string(), 7.0);
        let par2 = Parciales::new("ac".to_string(), 5.0);

        let mut vector  = Vec::new();
        vector.push(par1);
        vector.push(par2);
    
        let _est = Estudiante::new("".to_string(), 22993/5, vector);
    }

    #[test]
    fn test_promedio_nota_alta_baja() {
        let par1 = Parciales::new("cadp".to_string(), 7.0);
        let par2 = Parciales::new("ac".to_string(), 5.0);
        let par3 = Parciales::new("taller".to_string(), 4.0);
        let par4 = Parciales::new("oc".to_string(), 10.0);
        let par5 = Parciales::new("fod".to_string(), 6.0);

        let mut vector  = Vec::new();
        vector.push(par1);
        vector.push(par2);
        vector.push(par3);
        vector.push(par4);
        vector.push(par5);
    
        let est = Estudiante::new("Fermin Moras".to_string(), 22993/5, vector);
        assert_eq!(est.obtener_promedio(),6.4);
        assert_eq!(est.obtener_nota_mas_alta(),"10.0 oc".to_string());
        assert_eq!(est.obtener_nota_mas_baja(), "4.0 taller".to_string());
    }

}